//! Default frustrum callback implementations and configuration.
//!
//! The "frustrum" is Parasolid's mandatory callback interface. Applications must
//! provide memory management and file I/O callbacks before starting a session.
//!
//! This module provides sensible defaults:
//! - **Memory**: uses `std::alloc` with 16-byte alignment.
//! - **File I/O**: uses `std::fs` with guise-based file extensions per PK §6.4.2.
//!
//! Custom callbacks can be supplied by building a `PK_SESSION_frustrum_t`
//! directly and passing it to `PK_SESSION_register_frustrum`.
//!
//! All callback signatures and the registration struct layout are verified
//! against the Parasolid V35 header documentation (`PK_SESSION_frustrum_t`,
//! `PK_FFOPRD_f_t`, `PK_FFOPWR_f_t`, `PK_FFREAD_f_t`, `PK_FFWRIT_f_t`,
//! `PK_FFCLOS_f_t`, `PK_FMALLO_f_t`, `PK_FMFREE_f_t`).
//!
//! Set the `PARASOLID_FRUSTRUM_TRACE` environment variable to log every
//! frustrum callback invocation to stderr.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::os::raw::{c_char, c_int};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use parasolid_sys::*;

// =============================================================================
// FrustrumConfig
// =============================================================================

/// Configuration for the default frustrum callbacks.
#[derive(Debug, Clone)]
pub struct FrustrumConfig {
    /// Base directory for resolving file keys. If `None`, keys are treated as
    /// absolute or relative to the current working directory.
    pub(crate) base_dir: Option<PathBuf>,
}

impl FrustrumConfig {
    /// Create a new config with defaults.
    pub fn new() -> Self {
        FrustrumConfig { base_dir: None }
    }

    /// Set the base directory for file I/O. File keys passed by Parasolid will
    /// be resolved relative to this directory.
    pub fn base_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.base_dir = Some(dir.into());
        self
    }
}

impl Default for FrustrumConfig {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Global state for default callbacks
// =============================================================================

/// Stored config — resettable between sessions (unlike OnceLock).
static CONFIG: Mutex<Option<FrustrumConfig>> = Mutex::new(None);

/// An open frustrum stream.
enum StreamHandle {
    Reader(BufReader<File>),
    Writer(BufWriter<File>),
}

struct StreamEntry {
    handle: StreamHandle,
    /// Path of the file backing this stream (used to delete on FFABOR close).
    path: PathBuf,
    /// True if this stream created the file (write streams).
    created: bool,
}

struct StreamTable {
    streams: HashMap<i32, StreamEntry>,
    next_strid: i32,
}

static STREAM_TABLE: LazyLock<Mutex<StreamTable>> = LazyLock::new(|| {
    Mutex::new(StreamTable {
        streams: HashMap::new(),
        next_strid: 1,
    })
});

/// Alignment for all FMALLO allocations. 16 bytes covers f64 SIMD alignment.
const ALLOC_ALIGN: usize = 16;

fn trace_enabled() -> bool {
    std::env::var_os("PARASOLID_FRUSTRUM_TRACE").is_some()
}

macro_rules! trace {
    ($($arg:tt)*) => {
        if trace_enabled() {
            eprintln!("[frustrum] {}", format_args!($($arg)*));
        }
    };
}

// =============================================================================
// Public API
// =============================================================================

/// Build a `PK_SESSION_frustrum_t` populated with the default callbacks.
pub(crate) fn build_frustrum(config: &FrustrumConfig) -> PK_SESSION_frustrum_t {
    // Store config for use by file callbacks (replaces any previous config)
    if let Ok(mut guard) = CONFIG.lock() {
        *guard = Some(config.clone());
    }

    PK_SESSION_frustrum_t {
        fstart: Some(default_fstart),
        fabort: Some(default_fabort),
        fstop: Some(default_fstop),
        fmallo: Some(default_fmallo),
        fmfree: Some(default_fmfree),
        gosgmt: None,
        goopsg: None,
        goclsg: None,
        gopixl: None,
        gooppx: None,
        goclpx: None,
        ffoprd: Some(default_ffoprd),
        ffopwr: Some(default_ffopwr),
        ffclos: Some(default_ffclos),
        ffread: Some(default_ffread),
        ffwrit: Some(default_ffwrit),
        ffoprb: None,
        ffseek: None,
        fftell: None,
        fgcrcu: None,
        fgcrsu: None,
        fgevcu: None,
        fgevsu: None,
        fgprcu: None,
        fgprsu: None,
        ucoprd: None,
        ucopwr: None,
    }
}

/// Reset global frustrum state. Called when a session is stopped.
pub(crate) fn reset() {
    if let Ok(mut table) = STREAM_TABLE.lock() {
        table.streams.clear();
    }
    if let Ok(mut guard) = CONFIG.lock() {
        *guard = None;
    }
}

// =============================================================================
// Guise/format-to-extension mapping (PK §6.4.2)
// =============================================================================

/// Map a PK file guise + format code to the standard file extension.
///
/// Per the Parasolid frustrum convention, the application appends a
/// type-appropriate extension to the key before opening the file.
fn guise_extension(guise: c_int, format: c_int) -> &'static str {
    let text = format != FFBNRY;
    match guise {
        FFCSNP => {
            if text {
                ".snp_txt"
            } else {
                ".snp_bin"
            }
        }
        FFCJNL => ".jnl",
        FFCXMT | FFCXMO => {
            if text {
                ".xmt_txt"
            } else {
                ".xmt_bin"
            }
        }
        FFCSCH => ".sch_txt",
        FFCLNC => ".lnc",
        FFCXMP => {
            if text {
                ".xmp_txt"
            } else {
                ".xmp_bin"
            }
        }
        FFCXMD => {
            if text {
                ".xmd_txt"
            } else {
                ".xmd_bin"
            }
        }
        FFCDBG => {
            if format == FFXML {
                ".xml"
            } else {
                ".txt"
            }
        }
        _ => "",
    }
}

// =============================================================================
// XT file header (PK §6.4)
//
// Frustrum files start with an ASCII header written by the application (not
// by the kernel). It consists of 80-column lines beginning with `**` and ends
// with an `**END_OF_HEADER` line. On FFOPRD with FFSKHD, the frustrum must
// position the stream just past the header before the kernel starts reading.
// =============================================================================

const HEADER_END_MARKER: &[u8] = b"**END_OF_HEADER";

/// Pad a header line with `*` to 80 columns and terminate with newline.
fn header_line(content: &str) -> Vec<u8> {
    let mut line = content.as_bytes().to_vec();
    while line.len() < 80 {
        line.push(b'*');
    }
    line.push(b'\n');
    line
}

/// Build the standard frustrum file header.
///
/// `pr2hdr` is the part-2 header data supplied by Parasolid in FFOPWR
/// (`KEYWORD1=value1;KEYWORD2=value2...`).
fn build_header(pr2hdr: &str) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend(header_line(
        "**ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
    ));
    out.extend(header_line(
        "**PARASOLID !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~0123456789",
    ));
    out.extend(b"**PART1;FRU=parasolid-rs;APPL=parasolid-rs;\n".to_vec());
    if pr2hdr.is_empty() {
        out.extend(b"**PART2;\n".to_vec());
    } else {
        out.extend(format!("**PART2;{pr2hdr};\n").into_bytes());
    }
    out.extend(b"**PART3;\n".to_vec());
    out.extend(header_line("**END_OF_HEADER"));
    out
}

/// Position `file` just past the frustrum header, if one is present.
///
/// If the file does not start with `**`, or no `**END_OF_HEADER` line is found
/// within the first 64 KiB, the file is rewound to the start (headerless files
/// are legal — notably files written by other frustrums).
fn skip_header(file: &mut File) -> std::io::Result<()> {
    let mut prefix = [0u8; 2];
    let n = file.read(&mut prefix)?;
    if n < 2 || &prefix != b"**" {
        file.seek(SeekFrom::Start(0))?;
        return Ok(());
    }
    file.seek(SeekFrom::Start(0))?;

    // Scan for the END_OF_HEADER line, then skip through the end of that line.
    let mut buf = vec![0u8; 65536];
    let mut filled = 0usize;
    loop {
        let n = file.read(&mut buf[filled..])?;
        filled += n;
        if let Some(pos) = buf[..filled]
            .windows(HEADER_END_MARKER.len())
            .position(|w| w == HEADER_END_MARKER)
        {
            // Find the newline terminating the END_OF_HEADER line.
            match buf[pos..filled].iter().position(|&b| b == b'\n') {
                Some(nl) => {
                    file.seek(SeekFrom::Start((pos + nl + 1) as u64))?;
                    return Ok(());
                }
                None if n == 0 => {
                    // Marker found but no newline before EOF — treat as end.
                    file.seek(SeekFrom::Start(filled as u64))?;
                    return Ok(());
                }
                None => continue, // read more to find the newline
            }
        }
        if n == 0 || filled == buf.len() {
            // No header marker found — rewind and treat file as headerless.
            file.seek(SeekFrom::Start(0))?;
            return Ok(());
        }
    }
}

// =============================================================================
// Session lifecycle callbacks
// =============================================================================

unsafe extern "C" fn default_fstart(ifail: *mut c_int) {
    trace!("FSTART");
    unsafe { *ifail = FR_no_errors };
}

unsafe extern "C" fn default_fabort(ifail: *const c_int) {
    let code = unsafe { *ifail };
    trace!("FABORT: error code {code} (0x{code:x})");
}

unsafe extern "C" fn default_fstop(ifail: *mut c_int) {
    trace!("FSTOP");
    // Close all open streams
    if let Ok(mut table) = STREAM_TABLE.lock() {
        table.streams.clear();
    }
    unsafe { *ifail = FR_no_errors };
}

// =============================================================================
// Memory callbacks
// =============================================================================

unsafe extern "C" fn default_fmallo(
    nbytes: *const c_int,
    memory: *mut *mut c_char,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let size = *nbytes as usize;
        trace!("FMALLO: {size} bytes");
        if size == 0 {
            *memory = std::ptr::null_mut();
            *ifail = FR_no_errors;
            return;
        }
        let layout = std::alloc::Layout::from_size_align(size, ALLOC_ALIGN)
            .expect("invalid allocation layout");
        let ptr = std::alloc::alloc_zeroed(layout);
        if ptr.is_null() {
            trace!("FMALLO: alloc FAILED for {size} bytes");
            *ifail = FR_memory_full;
        } else {
            *memory = ptr as *mut c_char;
            *ifail = FR_no_errors;
        }
    }));
    if result.is_err() {
        unsafe { *ifail = FR_memory_full };
    }
}

// INVARIANT (PK §6.5): Parasolid guarantees that fmfree receives the exact
// same `nbytes` value that was passed to fmallo for the same pointer. This is
// required for Layout reconstruction without a size-tracking header. If this
// contract is ever violated, std::alloc::dealloc receives a wrong Layout → UB.
unsafe extern "C" fn default_fmfree(
    nbytes: *const c_int,
    memory: *mut *mut c_char,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let ptr = *memory;
        trace!("FMFREE: {} bytes", *nbytes);
        if ptr.is_null() {
            *ifail = FR_no_errors;
            return;
        }
        let size = *nbytes as usize;
        if size == 0 {
            *memory = std::ptr::null_mut();
            *ifail = FR_no_errors;
            return;
        }
        let layout = std::alloc::Layout::from_size_align(size, ALLOC_ALIGN)
            .expect("invalid deallocation layout");
        std::alloc::dealloc(ptr as *mut u8, layout);
        *memory = std::ptr::null_mut();
        *ifail = FR_no_errors;
    }));
    if result.is_err() {
        unsafe { *ifail = FR_unspecified };
    }
}

// =============================================================================
// File I/O callbacks
// =============================================================================

/// Resolve a PK file key + guise + format to a filesystem path.
///
/// Appends the guise-appropriate extension to the key, then optionally
/// joins with the configured base directory.
///
/// # Safety
///
/// `name` must point to at least `namlen` valid bytes.
unsafe fn resolve_key(
    name: *const c_char,
    namlen: c_int,
    guise: c_int,
    format: c_int,
) -> Option<PathBuf> {
    if name.is_null() || namlen <= 0 {
        return None;
    }
    let bytes = unsafe { std::slice::from_raw_parts(name as *const u8, namlen as usize) };
    // Trim trailing whitespace/nulls (PK keys may be space-padded)
    let trimmed = std::str::from_utf8(bytes).ok()?.trim_end();
    if trimmed.is_empty() {
        return None;
    }

    // Append guise extension: "my_part" + ".xmt_txt" → "my_part.xmt_txt"
    let ext = guise_extension(guise, format);
    let filename = format!("{trimmed}{ext}");
    let path = Path::new(&filename);

    // Fail if CONFIG mutex is poisoned rather than silently proceeding
    // without base_dir (which would open files at wrong paths).
    let guard = CONFIG.lock().ok()?;
    let result = if let Some(ref cfg) = *guard {
        if let Some(ref base) = cfg.base_dir {
            if path.is_relative() {
                base.join(path)
            } else {
                path.to_path_buf()
            }
        } else {
            path.to_path_buf()
        }
    } else {
        path.to_path_buf()
    };

    Some(result)
}

unsafe extern "C" fn default_ffoprd(
    guise: *const c_int,
    format: *const c_int,
    name: *const c_char,
    namlen: *const c_int,
    skiphd: *const c_int,
    strid: *mut c_int,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let path = match resolve_key(name, *namlen, *guise, *format) {
            Some(p) => p,
            None => {
                *ifail = FR_bad_name;
                return;
            }
        };
        trace!(
            "FFOPRD: guise={} format={} path={}",
            *guise,
            *format,
            path.display()
        );

        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                *ifail = FR_not_found;
                return;
            }
            Err(_) => {
                *ifail = FR_open_fail;
                return;
            }
        };

        if *skiphd == FFSKHD && skip_header(&mut file).is_err() {
            *ifail = FR_bad_header;
            return;
        }

        let mut table = match STREAM_TABLE.lock() {
            Ok(t) => t,
            Err(_) => {
                *ifail = FR_open_fail;
                return;
            }
        };
        let id = table.next_strid;
        table.next_strid += 1;
        table.streams.insert(
            id,
            StreamEntry {
                handle: StreamHandle::Reader(BufReader::new(file)),
                path,
                created: false,
            },
        );
        *strid = id;
        *ifail = FR_no_errors;
    }));
    if result.is_err() {
        unsafe { *ifail = FR_open_fail };
    }
}

unsafe extern "C" fn default_ffopwr(
    guise: *const c_int,
    format: *const c_int,
    name: *const c_char,
    namlen: *const c_int,
    pr2hdr: *const c_char,
    pr2len: *const c_int,
    strid: *mut c_int,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let path = match resolve_key(name, *namlen, *guise, *format) {
            Some(p) => p,
            None => {
                *ifail = FR_bad_name;
                return;
            }
        };
        trace!(
            "FFOPWR: guise={} format={} path={}",
            *guise,
            *format,
            path.display()
        );

        let file = match File::create(&path) {
            Ok(f) => f,
            Err(_) => {
                *ifail = FR_open_fail;
                return;
            }
        };
        let mut writer = BufWriter::new(file);

        // Write the standard frustrum header (skipped again on read).
        // Debug report files are raw output — no header.
        if *guise != FFCDBG {
            let pr2 = if pr2hdr.is_null() || *pr2len <= 0 {
                ""
            } else {
                let bytes = std::slice::from_raw_parts(pr2hdr as *const u8, *pr2len as usize);
                std::str::from_utf8(bytes).unwrap_or("")
            };
            if writer.write_all(&build_header(pr2)).is_err() {
                *ifail = FR_write_fail;
                return;
            }
        }

        let mut table = match STREAM_TABLE.lock() {
            Ok(t) => t,
            Err(_) => {
                *ifail = FR_open_fail;
                return;
            }
        };
        let id = table.next_strid;
        table.next_strid += 1;
        table.streams.insert(
            id,
            StreamEntry {
                handle: StreamHandle::Writer(writer),
                path,
                created: true,
            },
        );
        *strid = id;
        *ifail = FR_no_errors;
    }));
    if result.is_err() {
        unsafe { *ifail = FR_open_fail };
    }
}

unsafe extern "C" fn default_ffread(
    _guise: *const c_int,
    strid: *const c_int,
    nmax: *const c_int,
    buffer: *mut c_char,
    nactual: *mut c_int,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let id = *strid;
        let max = *nmax as usize;
        let buf = std::slice::from_raw_parts_mut(buffer as *mut u8, max);

        let mut table = match STREAM_TABLE.lock() {
            Ok(t) => t,
            Err(_) => {
                *ifail = FR_read_fail;
                return;
            }
        };

        match table.streams.get_mut(&id).map(|e| &mut e.handle) {
            Some(StreamHandle::Reader(reader)) => {
                // Fill as much of the buffer as possible; FR_end_of_file is
                // only reported when zero bytes could be read.
                let mut total = 0usize;
                loop {
                    match reader.read(&mut buf[total..]) {
                        Ok(0) => break,
                        Ok(n) => {
                            total += n;
                            if total == max {
                                break;
                            }
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                        Err(_) => {
                            *ifail = FR_read_fail;
                            return;
                        }
                    }
                }
                *nactual = total as c_int;
                *ifail = if total == 0 && max > 0 {
                    FR_end_of_file
                } else {
                    FR_no_errors
                };
            }
            _ => *ifail = FR_read_fail,
        }
    }));
    if result.is_err() {
        unsafe { *ifail = FR_read_fail };
    }
}

unsafe extern "C" fn default_ffwrit(
    _guise: *const c_int,
    strid: *const c_int,
    nchars: *const c_int,
    buffer: *const c_char,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let id = *strid;
        let count = *nchars as usize;
        let data = std::slice::from_raw_parts(buffer as *const u8, count);

        let mut table = match STREAM_TABLE.lock() {
            Ok(t) => t,
            Err(_) => {
                *ifail = FR_write_fail;
                return;
            }
        };

        match table.streams.get_mut(&id).map(|e| &mut e.handle) {
            Some(StreamHandle::Writer(writer)) => match writer.write_all(data) {
                Ok(()) => *ifail = FR_no_errors,
                Err(_) => *ifail = FR_write_fail,
            },
            _ => *ifail = FR_write_fail,
        }
    }));
    if result.is_err() {
        unsafe { *ifail = FR_write_fail };
    }
}

unsafe extern "C" fn default_ffclos(
    _guise: *const c_int,
    strid: *const c_int,
    action: *const c_int,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let id = *strid;
        let abort = *action == FFABOR;
        let mut table = match STREAM_TABLE.lock() {
            Ok(t) => t,
            Err(_) => {
                *ifail = FR_close_fail;
                return;
            }
        };
        let Some(mut entry) = table.streams.remove(&id) else {
            *ifail = FR_close_fail;
            return;
        };
        trace!(
            "FFCLOS: strid={id} abort={abort} path={}",
            entry.path.display()
        );

        // Flush writer before closing — propagate flush errors
        if let StreamHandle::Writer(ref mut w) = entry.handle {
            if w.flush().is_err() && !abort {
                *ifail = FR_close_fail;
                return;
            }
        }
        let created = entry.created;
        let path = std::mem::take(&mut entry.path);
        drop(entry); // close the file handle before deleting

        // FFABOR: delete a newly created file
        if abort && created {
            let _ = std::fs::remove_file(&path);
        }
        *ifail = FR_no_errors;
    }));
    if result.is_err() {
        unsafe { *ifail = FR_close_fail };
    }
}

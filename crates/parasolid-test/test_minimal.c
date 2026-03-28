/* Minimal C test for pskernel.dll under Wine.
 * Build: x86_64-w64-mingw32-gcc -o test_minimal.exe test_minimal.c -L../../lib -lpskernel
 * Run:   WINEPATH=/path/to/SOLIDWORKS wine test_minimal.exe
 */
#include <stdio.h>
#include <string.h>

/* Minimal type declarations matching Parasolid API */
typedef int PK_ERROR_code_t;
typedef void (*PK_FSTART_f_t)(int *ifail);
typedef void (*PK_FSTOP_f_t)(int *ifail);
typedef void (*PK_FABORT_f_t)(const int *ifail);
typedef void (*PK_FMALLO_f_t)(const int *nbytes, char **memory, int *ifail);
typedef void (*PK_FMFREE_f_t)(const int *nbytes, char **memory, int *ifail);
typedef void (*PK_FFOPRD_f_t)(const int *nguises, const int *guises, const int *keylen, const char *key, int *ifail);
typedef void (*PK_FFOPWR_f_t)(const int *nguises, const int *guises, const int *keylen, const char *key, int *ifail);
typedef void (*PK_FFREAD_f_t)(const int *strid, const int *nchars, char *buf, int *ifail);
typedef void (*PK_FFWRIT_f_t)(const int *strid, const int *nchars, const char *buf, int *ifail);
typedef void (*PK_FFCLOS_f_t)(const int *strid, int *ifail);
typedef void (*PK_FTMKEY_f_t)(const int *key_type, int *key_len, char *key, int *ifail);

/* Frustrum struct — just the basic callbacks, no FG/rollback/graphics */
typedef struct {
    PK_FSTART_f_t fstart;
    PK_FABORT_f_t fabort;
    PK_FSTOP_f_t  fstop;
    PK_FTMKEY_f_t ftmkey;
    PK_FFOPRD_f_t ffoprd;
    PK_FFOPWR_f_t ffopwr;
    PK_FFREAD_f_t ffread;
    PK_FFWRIT_f_t ffwrit;
    PK_FFCLOS_f_t ffclos;
    PK_FMALLO_f_t fmallo;
    PK_FMFREE_f_t fmfree;
} PK_SESSION_frustrum_t;

typedef struct {
    int o_t_version;
    const char *journal_file;
    int user_field_len;
} PK_SESSION_start_o_t;

/* Import declarations */
extern PK_ERROR_code_t PK_SESSION_register_frustrum(const PK_SESSION_frustrum_t *fru);
extern PK_ERROR_code_t PK_SESSION_start(const PK_SESSION_start_o_t *opts);
extern PK_ERROR_code_t PK_SESSION_stop(void);

#include <stdlib.h>

/* Callback implementations */
void my_fstart(int *ifail) {
    fprintf(stderr, "[C] FSTART called\n");
    *ifail = 0;
}

void my_fstop(int *ifail) {
    fprintf(stderr, "[C] FSTOP called\n");
    *ifail = 0;
}

void my_fabort(const int *ifail) {
    fprintf(stderr, "[C] FABORT called, code=%d\n", *ifail);
}

void my_fmallo(const int *nbytes, char **memory, int *ifail) {
    int size = *nbytes;
    fprintf(stderr, "[C] FMALLO: %d bytes\n", size);
    if (size <= 0) {
        *memory = NULL;
        *ifail = 0;
        return;
    }
    *memory = (char*)calloc(1, size);
    *ifail = (*memory != NULL) ? 0 : 1;
}

void my_fmfree(const int *nbytes, char **memory, int *ifail) {
    fprintf(stderr, "[C] FMFREE: %d bytes\n", *nbytes);
    if (*memory) free(*memory);
    *memory = NULL;
    *ifail = 0;
}

void my_ffoprd(const int *n, const int *g, const int *kl, const char *k, int *ifail) {
    fprintf(stderr, "[C] FFOPRD called\n");
    *ifail = 1; /* not implemented */
}

void my_ffopwr(const int *n, const int *g, const int *kl, const char *k, int *ifail) {
    fprintf(stderr, "[C] FFOPWR called\n");
    *ifail = 1;
}

void my_ffread(const int *s, const int *n, char *b, int *ifail) {
    fprintf(stderr, "[C] FFREAD called\n");
    *ifail = 1;
}

void my_ffwrit(const int *s, const int *n, const char *b, int *ifail) {
    fprintf(stderr, "[C] FFWRIT called\n");
    *ifail = 1;
}

void my_ffclos(const int *s, int *ifail) {
    fprintf(stderr, "[C] FFCLOS called\n");
    *ifail = 0;
}

int main(void) {
    fprintf(stderr, "=== Minimal C Parasolid Test ===\n");
    fprintf(stderr, "sizeof(PK_SESSION_frustrum_t) = %zu\n", sizeof(PK_SESSION_frustrum_t));

    PK_SESSION_frustrum_t fru;
    memset(&fru, 0, sizeof(fru));
    fru.fstart = my_fstart;
    fru.fabort = my_fabort;
    fru.fstop  = my_fstop;
    fru.fmallo = my_fmallo;
    fru.fmfree = my_fmfree;
    fru.ffoprd = my_ffoprd;
    fru.ffopwr = my_ffopwr;
    fru.ffread = my_ffread;
    fru.ffwrit = my_ffwrit;
    fru.ffclos = my_ffclos;

    fprintf(stderr, "Registering frustrum...\n");
    PK_ERROR_code_t err = PK_SESSION_register_frustrum(&fru);
    fprintf(stderr, "  register returned: %d\n", err);

    fprintf(stderr, "Starting session...\n");
    PK_SESSION_start_o_t opts;
    memset(&opts, 0, sizeof(opts));
    opts.o_t_version = 1;

    err = PK_SESSION_start(&opts);
    fprintf(stderr, "  start returned: %d\n", err);

    if (err == 0) {
        fprintf(stderr, "Session started successfully!\n");
        PK_SESSION_stop();
    }

    fprintf(stderr, "=== Done ===\n");
    return err;
}

#!/usr/bin/env bash
# Remove sensitive paths (proprietary Parasolid binaries in lib/, and the
# .claude/ agent config folder) from the ENTIRE git history and force-push the
# rewritten history.
#
# ⚠️  READ FIRST:
#   * This rewrites history and changes every commit SHA. Coordinate with any
#     collaborators — they must re-clone.
#   * A history rewrite does NOT undo past exposure. If these paths were ever
#     pushed to a public remote, treat their contents as already leaked; handle
#     the Siemens/SOLIDWORKS licensing side separately, and ask GitHub Support
#     to purge cached commits/blobs.
#   * Run from the repo root. Make a backup clone first:
#       git clone --mirror <url> parasolid-rs-backup.git
#
# Usage: bash scripts/scrub-proprietary-binaries.sh

set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

# Paths to purge from all of history.
SCRUB_PATHS=(lib/ .claude/)

echo "==> Repo: $(pwd)"
echo "==> Paths present in history:"
for p in "${SCRUB_PATHS[@]}"; do
  echo "  -- ${p}"
  git log --all --oneline --name-only -- "$p" | grep "^${p}" | sort -u || true
done

read -r -p "Proceed with history rewrite? [y/N] " ans
[[ "${ans:-N}" == "y" || "${ans:-N}" == "Y" ]] || { echo "Aborted."; exit 1; }

# 1) Untrack in the working tree (idempotent) and make sure the paths are ignored.
for p in "${SCRUB_PATHS[@]}"; do
  git rm -r --cached --ignore-unmatch "$p" >/dev/null 2>&1 || true
  ignore="/${p%/}/"
  grep -qxF "$ignore" .gitignore 2>/dev/null || printf '\n%s\n' "$ignore" >> .gitignore
done
git add .gitignore || true
git commit -m "Stop tracking sensitive paths (lib/, .claude/)" >/dev/null 2>&1 || true

# 2) Scrub from all history — prefer git-filter-repo.
if command -v git-filter-repo >/dev/null 2>&1 || python3 -c 'import git_filter_repo' 2>/dev/null; then
  echo "==> Using git filter-repo"
  filter_args=(--force --invert-paths)
  for p in "${SCRUB_PATHS[@]}"; do
    filter_args+=(--path "$p")
  done
  git filter-repo "${filter_args[@]}"
else
  echo "==> git-filter-repo not found; falling back to git filter-branch"
  echo "    (install with: pip install git-filter-repo)"
  index_filter='git rm -r --cached --ignore-unmatch'
  for p in "${SCRUB_PATHS[@]}"; do
    index_filter+=" '$p'"
  done
  FILTER_BRANCH_SQUELCH_WARNING=1 git filter-branch --force --index-filter \
    "$index_filter" \
    --prune-empty --tag-name-filter cat -- --all
  rm -rf .git/refs/original/
  git reflog expire --expire=now --all
  git gc --prune=now --aggressive
fi

echo
echo "==> Local history rewritten. Verify nothing remains:"
remaining=0
for p in "${SCRUB_PATHS[@]}"; do
  if git log --all --oneline --name-only -- "$p" | grep -q "^${p}"; then
    echo "!! ${p} still present in history — investigate before pushing"
    remaining=1
  else
    echo "   OK: no ${p} paths in history."
  fi
done
[[ "$remaining" -eq 0 ]] || exit 1

echo
echo "==> Next (manual, after you've confirmed a remote exists):"
echo "    git remote -v      # filter-repo removes 'origin'; re-add if needed"
echo "    git push origin --force --all"
echo "    git push origin --force --tags"
echo "    Then ask GitHub Support to purge cached commits/blobs."

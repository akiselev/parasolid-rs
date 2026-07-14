#!/usr/bin/env bash
# Remove proprietary Parasolid binaries (lib/) from the ENTIRE git history and
# force-push the rewritten history.
#
# ⚠️  READ FIRST:
#   * This rewrites history and changes every commit SHA. Coordinate with any
#     collaborators — they must re-clone.
#   * A history rewrite does NOT undo past exposure. If lib/ was ever pushed to
#     a public remote, treat the binary as already leaked and handle the
#     Siemens/SOLIDWORKS licensing side separately, and ask GitHub Support to
#     purge cached commits/blobs.
#   * Run from the repo root. Make a backup clone first:
#       git clone --mirror <url> parasolid-rs-backup.git
#
# Usage: bash scripts/scrub-proprietary-binaries.sh
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

echo "==> Repo: $(pwd)"
echo "==> Paths present in history under lib/:"
git log --all --oneline --name-only -- lib/ | grep '^lib/' | sort -u || true

read -r -p "Proceed with history rewrite? [y/N] " ans
[[ "${ans:-N}" == "y" || "${ans:-N}" == "Y" ]] || { echo "Aborted."; exit 1; }

# 1) Untrack in the working tree (idempotent) and make sure lib/ is ignored.
git rm -r --cached --ignore-unmatch lib/ >/dev/null 2>&1 || true
grep -qxF '/lib/' .gitignore 2>/dev/null || printf '\n/lib/\n' >> .gitignore
git add .gitignore || true
git commit -m "Stop tracking proprietary Parasolid binaries (lib/)" >/dev/null 2>&1 || true

# 2) Scrub from all history — prefer git-filter-repo.
if command -v git-filter-repo >/dev/null 2>&1 || python3 -c 'import git_filter_repo' 2>/dev/null; then
  echo "==> Using git filter-repo"
  git filter-repo --force --invert-paths --path lib/
else
  echo "==> git-filter-repo not found; falling back to git filter-branch"
  echo "    (install with: pip install git-filter-repo)"
  FILTER_BRANCH_SQUELCH_WARNING=1 git filter-branch --force --index-filter \
    'git rm -r --cached --ignore-unmatch lib/' \
    --prune-empty --tag-name-filter cat -- --all
  rm -rf .git/refs/original/
  git reflog expire --expire=now --all
  git gc --prune=now --aggressive
fi

echo
echo "==> Local history rewritten. Verify nothing remains:"
git log --all --oneline --name-only -- lib/ | grep '^lib/' && {
  echo "!! lib/ still present in history — investigate before pushing"; exit 1;
} || echo "   OK: no lib/ paths in history."

echo
echo "==> Next (manual, after you've confirmed a remote exists):"
echo "    git remote -v      # filter-repo removes 'origin'; re-add if needed"
echo "    git push origin --force --all"
echo "    git push origin --force --tags"
echo "    Then ask GitHub Support to purge cached commits/blobs."

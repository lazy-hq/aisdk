#!/usr/bin/env bash
set -euo pipefail

LEVEL=${1:-}
[[ -z "$LEVEL" ]] && { echo "Usage: $0 [patch|minor|major|x.y.z]"; exit 1; }

PKG=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')
OLD_V=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

# dry-run preview of the new version
if [[ "$LEVEL" =~ ^[0-9]+\.[0-9]+\.[0-9+](-.+)?$ ]]; then
    NEW_V="$LEVEL" # user gave explicit version
else
    NEW_V=$(cargo release version "$LEVEL" --quiet 2>&1 | awk '/Upgrading.*from/{print $NF}' || echo "")
    [[ -z "$NEW_V" ]] && NEW_V=$(cargo release version "$LEVEL" --quiet 2>&1 | grep -oP '\d+\.\d+\.\d+[^ ]*' | head -1)
fi
[[ -z "$NEW_V" ]] && { echo "Could not determine new version"; exit 1; }

BRANCH="release-${NEW_V}"
REMOTE=${REMOTE:-origin}

echo "=== ${PKG}  ${OLD_V}  ->  ${NEW_V}  (branch ${BRANCH})"
echo

# Check if branch already exists
if git rev-parse --verify "$BRANCH" >/dev/null 2>&1; then
    echo "⚠️  Branch $BRANCH already exists locally"
    read -p "Delete and recreate? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git branch -D "$BRANCH"
    else
        echo "Aborted"
        exit 1
    fi
fi

# Check if remote branch exists
if git ls-remote --heads "$REMOTE" "$BRANCH" | grep -q "$BRANCH"; then
    echo "⚠️  Branch $BRANCH already exists on remote"
    read -p "Force update remote branch? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted"
        exit 1
    fi
    FORCE_PUSH="-f"
else
    FORCE_PUSH=""
fi

git switch -C "$BRANCH"

## bump + changelog + commit  (no publish/push)
echo "Running cargo release..."
cargo release --no-publish --no-push --no-confirm --execute "$LEVEL"

## push branch
echo
echo "Pushing branch to $REMOTE..."
git push $FORCE_PUSH -u "$REMOTE" "$BRANCH"

## create PR (check if PR already exists first)
echo
echo "Creating pull request..."
EXISTING_PR=$(gh pr list --head "$BRANCH" --json number --jq '.[0].number' 2>/dev/null || echo "")
if [[ -n "$EXISTING_PR" ]]; then
    echo "ℹ️  PR #$EXISTING_PR already exists for branch $BRANCH"
    PR_URL=$(gh pr view "$EXISTING_PR" --json url --jq '.url')
else
    gh pr create \
        --title "Release v${NEW_V}" \
        --body "Bump version to ${NEW_V} and update changelog." \
        --base main \
        --head "$BRANCH"
    PR_URL=$(gh pr view "$BRANCH" --json url --jq '.url')
fi

echo
echo "✅  Release PR ready"
echo "   Version: v${NEW_V}"
echo "   Branch:  ${BRANCH}"
echo "   PR:      ${PR_URL}"
echo
echo "Next steps:"
echo "  1. Review the PR: ${PR_URL}"
echo "  2. Merge the PR to main"
echo "  3. For crates.io publish, checkout to main"
echo "  5. git tag v${NEW_V} && git push origin v${NEW_V}"

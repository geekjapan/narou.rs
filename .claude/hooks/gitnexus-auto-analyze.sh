#!/usr/bin/env bash
# PostToolUse(Bash) hook: refresh the GitNexus index after a git mutation.
#
# Fires `npx gitnexus analyze --skip-agents-md` in a detached background process
# so the agent turn is never blocked (avoids the up-to-120s analyze stall and the
# KuzuDB-corruption-on-timeout risk the GitNexus CLI skill warns about).
#
# - Only acts on `git commit` / `git merge` / `git pull` commands.
# - Skips when no `.gitnexus/` index exists.
# - A lock dir under `.gitnexus/` (gitignored) prevents concurrent analyze runs.
# - GITNEXUS_ANALYZE_CMD overrides the analyze command (used by tests).
set -u

input=$(cat)
cmd=$(printf '%s' "$input" | jq -r '.tool_input.command // ""' 2>/dev/null)

case "$cmd" in
  *"git commit"*|*"git merge"*|*"git pull"*) ;;
  *) exit 0 ;;
esac

dir="${CLAUDE_PROJECT_DIR:-$(pwd)}"
[ -d "$dir/.gitnexus" ] || exit 0

lock="$dir/.gitnexus/.auto-analyze.lock"
# mkdir is atomic: if the lock already exists, another analyze is in flight.
mkdir "$lock" 2>/dev/null || exit 0

analyze_cmd="${GITNEXUS_ANALYZE_CMD:-npx --yes gitnexus analyze --skip-agents-md}"

nohup sh -c "cd '$dir'; trap 'rmdir \"$lock\" 2>/dev/null' EXIT; $analyze_cmd" \
  >/dev/null 2>&1 &

exit 0

#!/usr/bin/env bash

set -e

if [[ "$AOC_SESSION_TOKEN" == "" ]]; then
  echo "Error: Missing AOC_SESSION_TOKEN" >&2
  exit 1
fi

(
  awk '1;/^## Scoreboard/{exit}' readme.md
  echo
  echo '| Problem | Stars | Time | Rank |'
  echo '| ------- | ----- | ---- | ---- |'
  curl -sb "session=${AOC_SESSION_TOKEN}" "https://adventofcode.com/2024/leaderboard/self" \
    | sed -En '/<pre>/,${p;/<\/pre>/q}' \
    | sed -En '/^Day\b/,${p}' \
    | tail -n +2 \
    | head -n -1 \
    | perl -MHTML::Entities -pe 'decode_entities($_);' \
    | awk '{ printf("| [Day %02d](./src/day%02d.rs) | %s%s | %s / %s | %s / %s |\n", $1, $1, ($3!="-")?"⭐":"", ($6!="-")?"⭐":"", $2, $5, $3, $6 ) }' \
    | tac
) > readme.tmp.md

mv readme.tmp.md readme.md

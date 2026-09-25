#!/usr/bin/env bash
# Live latency and edge-cache readback for a deployed host.
# Usage: scripts/measure-live.sh [base-url] [runs]
# Prints p50 time-to-first-byte and the last cf-cache-status per URL.
set -euo pipefail
BASE="${1:-https://mark.sylphx.com}"
RUNS="${2:-15}"
URLS=(
  "/badge/build-passing-brightgreen"
  "/badge/build-passing-brightgreen.svg"
  "/api/v1/mark/hero?type=waving&text=Hello%20README"
  "/api/v1/mark/hero.svg?type=waving&text=Hello%20README"
  "/api/v1/mark/strip?icons=rust,ts,docker"
)
printf '| URL | p50 TTFB (ms) | cf-cache-status |\n| --- | ---: | --- |\n'
for u in "${URLS[@]}"; do
  times=()
  status=""
  for _ in $(seq "$RUNS"); do
    out=$(curl -s -o /dev/null -D - -w 'TTFB %{time_starttransfer}\n' "$BASE$u")
    times+=("$(printf '%s\n' "$out" | awk '/^TTFB/ {printf "%.1f", $2*1000}')")
    status=$(printf '%s\n' "$out" | awk 'tolower($1)=="cf-cache-status:" {print $2}' | tr -d '\r')
  done
  p50=$(printf '%s\n' "${times[@]}" | sort -n | awk '{a[NR]=$1} END {print a[int((NR+1)/2)]}')
  printf '| `%s` | %s | %s |\n' "$u" "$p50" "${status:-none}"
done

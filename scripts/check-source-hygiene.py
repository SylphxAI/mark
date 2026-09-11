#!/usr/bin/env python3
"""Structural hygiene gate for mark's source tree.

These are static properties of the source (not runtime behaviour proofs):

1. One authority per shared concept: a duplicated implementation is how this
   repository accumulated parallel render authorities, so each listed symbol
   must be defined exactly once.
2. No `#[allow(dead_code)]`: unused code is deleted, never silenced.
3. No clock: `MARK-GRAMMAR`/`MARK-STATS` require a mark to be a pure function
   of its URL, so time sources are forbidden in `src/`.
4. No `dbg!`/`todo!`/`unimplemented!` left in `src/`.

Run: `python3 scripts/check-source-hygiene.py`
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"

# Concept -> symbol that must have exactly one definition in `src/`.
SINGLE_AUTHORITY = {
    "text metric": "line_advance",
    "text cropping": "fit_line",
    "text capping": "cap_text",
    "monogram": "monogram",
    "pill width": "measure",
    "paint token": "resolve_paint",
    "hash (32-bit)": "fnv1a_32",
    "hash (64-bit)": "fnv1a_64",
    "svg escaping": "esc",
    "svg document": "svg_doc",
    "svg color token": "normalize_hex_token",
    "icon glyph": "glyph",
    "theme lookup": "get",
    "badge path split": "split_badge_path",
}

FORBIDDEN_PATTERNS = {
    r"#\[allow\(dead_code\)\]": "dead code must be deleted, not allowed",
    r"\bSystemTime\b": "no clock on the render path (MARK-STATS is dead)",
    r"\bInstant::now\b": "no clock on the render path (MARK-STATS is dead)",
    r"\bstd::time\b": "no clock on the render path (MARK-STATS is dead)",
    r"\btokio::time\b": "no clock on the render path (MARK-STATS is dead)",
    r"\bdbg!\s*\(": "debug macro left in source",
    r"\btodo!\s*\(": "unfinished code left in source",
    r"\bunimplemented!\s*\(": "unfinished code left in source",
}


def rust_files() -> list[Path]:
    return sorted(SRC.rglob("*.rs"))


def failures() -> list[str]:
    files = rust_files()
    texts = {f: f.read_text() for f in files}
    found: list[str] = []

    for concept, symbol in SINGLE_AUTHORITY.items():
        definition = re.compile(rf"^\s*(?:pub(?:\(crate\))?\s+)?fn\s+{re.escape(symbol)}\b", re.M)
        hits = [
            (f, text[: match.start()].count("\n") + 1)
            for f, text in texts.items()
            for match in definition.finditer(text)
        ]
        if len(hits) != 1:
            where = ", ".join(f"{f.relative_to(ROOT)}:{line}" for f, line in hits) or "nowhere"
            found.append(f"{concept}: expected exactly 1 definition of fn {symbol}(); found {len(hits)} ({where})")

    for pattern, reason in FORBIDDEN_PATTERNS.items():
        regex = re.compile(pattern)
        for f, text in texts.items():
            for match in regex.finditer(text):
                line = text[: match.start()].count("\n") + 1
                found.append(f"{f.relative_to(ROOT)}:{line}: {reason}")

    if not files:
        found.append("no Rust sources found — wrong root?")
    return found


def main() -> int:
    found = failures()
    if found:
        for item in found:
            print(f"FAIL {item}", file=sys.stderr)
        print("source-hygiene contract failed", file=sys.stderr)
        return 1
    print(
        f"OK: {len(SINGLE_AUTHORITY)} single-authority concepts, "
        f"{len(FORBIDDEN_PATTERNS)} forbidden patterns, 0 findings"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

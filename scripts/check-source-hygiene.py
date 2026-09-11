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

Scanned scope: `src/**`, `tests/**`, and `build.rs` (product sources, contract
tests, and the build-identity generator). Comments are stripped before pattern
matching, so prose about a banned pattern is not a finding.

Rule scopes: the clock rules cover product sources and the build script only —
a test may legitimately wait on a timeout. The dead-code/dbg rules cover every
scanned file. Known limit: the comment stripper understands `//`, `/* */`, and
ordinary string literals; exotic raw-string/comment interleavings are out of
scope for a hygiene aid.

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
    "content family": "content_family",
    "badge path split": "split_badge_path",
}

# Retired predecessor grammar keys (`MARK-GRAMMAR`): the render accepts exactly
# the published grammar, so these string literals must not exist in `src/`.
RETIRED_GRAMMAR_KEYS = (
    "fontSize",
    "descSize",
    "fontColor",
    "fontAlign",
    "fontAlignY",
    "descAlign",
    "descAlignY",
    "strokeWidth",
    "textBg",
    "deploymark",
    "iconsrow",
)

# The shields-style text attributes may only be built by their owner module.
# A form that re-introduces a local copy makes the pill geometry drift again.
SHIELDS_TEXT_OWNER = "src/capabilities/mark/domain/pill.rs"
SHIELDS_TEXT_FORMS = (
    "src/capabilities/mark/application/pill.rs",
    "src/capabilities/mark/application/deploy.rs",
)
SHIELDS_TEXT_ATTRS = ("font-weight=", "font-family=", "letter-spacing=")

# Patterns applied to every scanned file.
FORBIDDEN_PATTERNS = {
    r"allow\([^)]*dead_code": "dead code must be deleted, not allowed",
    r"\bdbg!\s*\(": "debug macro left in source",
    r"\btodo!\s*\(": "unfinished code left in source",
    r"\bunimplemented!\s*\(": "unfinished code left in source",
}

# Patterns applied to product sources and the build script only: a contract test
# may legitimately wait on a timeout, but a mark must never read a clock.
CLOCK_PATTERNS = {
    r"\bSystemTime\b": "no clock on the render path (MARK-STATS is dead)",
    r"\bInstant::now\b": "no clock on the render path (MARK-STATS is dead)",
    r"\bstd::time\b": "no clock on the render path (MARK-STATS is dead)",
    r"\btokio::time\b": "no clock on the render path (MARK-STATS is dead)",
}


def rust_files() -> list[Path]:
    files = sorted(SRC.rglob("*.rs"))
    files += sorted((ROOT / "tests").glob("*.rs"))
    if (ROOT / "build.rs").exists():
        files.append(ROOT / "build.rs")
    return files


def strip_comments(text: str) -> str:
    """Remove `//` and `/* */` comments, leaving string literals intact."""
    out: list[str] = []
    index = 0
    length = len(text)
    while index < length:
        ch = text[index]
        if ch == '"':
            out.append(ch)
            index += 1
            while index < length:
                out.append(text[index])
                if text[index] == "\\":
                    index += 1
                    if index < length:
                        out.append(text[index])
                elif text[index] == '"':
                    index += 1
                    break
                index += 1
            continue
        if ch == "/" and index + 1 < length and text[index + 1] == "/":
            while index < length and text[index] != "\n":
                index += 1
            continue
        if ch == "/" and index + 1 < length and text[index + 1] == "*":
            end = text.find("*/", index + 2)
            index = length if end == -1 else end + 2
            continue
        out.append(ch)
        index += 1
    return "".join(out)


def failures() -> list[str]:
    files = rust_files()
    sources = {f: f.read_text() for f in files}
    texts = {f: strip_comments(text) for f, text in sources.items()}
    src_texts = {f: text for f, text in texts.items() if SRC in f.parents}
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

    for key in RETIRED_GRAMMAR_KEYS:
        literal = f'"{key}"'
        for f, text in src_texts.items():
            for match in re.finditer(re.escape(literal), text):
                line = text[: match.start()].count("\n") + 1
                found.append(
                    f"{f.relative_to(ROOT)}:{line}: retired grammar key {literal} reintroduced"
                )

    for pattern, reason in FORBIDDEN_PATTERNS.items():
        regex = re.compile(pattern)
        for f, text in texts.items():
            for match in regex.finditer(text):
                line = text[: match.start()].count("\n") + 1
                found.append(f"{f.relative_to(ROOT)}:{line}: {reason}")

    clock_scope = dict(src_texts)
    build_script = ROOT / "build.rs"
    if build_script in texts:
        clock_scope[build_script] = texts[build_script]
    for pattern, reason in CLOCK_PATTERNS.items():
        regex = re.compile(pattern)
        for f, text in clock_scope.items():
            for match in regex.finditer(text):
                line = text[: match.start()].count("\n") + 1
                found.append(f"{f.relative_to(ROOT)}:{line}: {reason}")

    for f, text in texts.items():
        relative = str(f.relative_to(ROOT))
        if relative not in SHIELDS_TEXT_FORMS:
            continue
        for attr in SHIELDS_TEXT_ATTRS:
            if attr in text:
                found.append(
                    f"{relative}: shields text attribute {attr!r} must be built by "
                    f"{SHIELDS_TEXT_OWNER} (PillMetrics), not locally"
                )

    if not files:
        found.append("no Rust sources found — wrong root?")
    return found


def main() -> int:
    found = list(dict.fromkeys(failures()))
    if found:
        for item in found:
            print(f"FAIL {item}", file=sys.stderr)
        print("source-hygiene contract failed", file=sys.stderr)
        return 1
    print(
        f"OK: {len(SINGLE_AUTHORITY)} single-authority concepts, "
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

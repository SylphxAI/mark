#!/usr/bin/env python3
"""Structural duplicate detector for the Rust sources.

Token-level, no network and no node dependency, so CI can gate it. Two signals:

1. **Clone blocks** — a run of `WINDOW` normalized tokens (identifiers and
   literals collapsed) appearing in two different places. This catches the
   copy-paste that produced parallel render authorities.
2. **Function pairs** — two items whose normalized token sequences are equal or
   >= `NEAR_RATIO` similar with at least `MIN_FUNCTION_TOKENS` tokens.

`import` runs are ignored: identical `use` lines are a wiring fact, not a
duplicated implementation. Cross-file clone runs and function pairs fail the
gate; same-file runs are a recorded boundary (`--report` prints one line per
file) because legitimate repeated data — theme rows, struct literals — lives
there, while a same-file function copy is caught by the pair rule.
`duplicate-exception: <reason>` exempts a *function pair* only; a ≥90-token
cross-file clone run has no exemption path, so a legitimate large exception must
be restructured or the threshold revisited explicitly.

Run: `python3 scripts/check-duplication.py [--report]`
"""

from __future__ import annotations

import difflib
import hashlib
import re
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"

WINDOW = 90          # tokens per clone window
MIN_FUNCTION_TOKENS = 50
NEAR_RATIO = 0.90
TOKEN_RE = re.compile(r"""
    (?P<comment>//[^\n]*|/\*.*?\*/)
  | (?P<string>r?\#*"(?:\\.|[^"\\])*"\#*)
  | (?P<char>'(?:\\.|[^'\\])')
  | (?P<lifetime>'[a-z_][A-Za-z0-9_]*)
  | (?P<raw>r\#*"(?:\\.|[^"\\])*"\#*)
  | (?P<ident>[A-Za-z_][A-Za-z0-9_]*)
  | (?P<number>\d[\w.]*)
  | (?P<punct>::|->|=>|[^\sA-Za-z0-9_])
""", re.S | re.X | re.M)

KEYWORDS = {
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
    "use", "where", "while", "crate", "box",
}


def tokenize(text: str) -> tuple[list[str], list[str]]:
    """(normalized, raw) token streams. Identifiers -> ID, literals -> LIT."""
    norm: list[str] = []
    raw: list[str] = []
    for match in TOKEN_RE.finditer(text):
        kind = match.lastgroup
        value = match.group()
        if kind == "comment":
            continue
        raw.append(value)
        if kind in {"string", "char", "raw"}:
            norm.append("LIT")
        elif kind == "number":
            norm.append("NUM")
        elif kind == "ident":
            norm.append(value if value in KEYWORDS else "ID")
        elif kind == "lifetime":
            norm.append("LIFETIME")
        else:
            norm.append(value)
    return norm, raw


def without_imports(
    norm: list[str], raw: list[str], lines: list[int]
) -> tuple[list[str], list[str], list[int]]:
    """Drop `use …;` items: identical imports are wiring, not duplication."""
    kept_norm: list[str] = []
    kept_raw: list[str] = []
    kept_lines: list[int] = []
    index = 0
    while index < len(norm):
        if norm[index] == "use":
            depth = 0
            end = index
            for j in range(index, len(norm)):
                if norm[j] in "{(":
                    depth += 1
                elif norm[j] in "})":
                    depth -= 1
                elif norm[j] == ";" and depth == 0:
                    end = j
                    break
            index = end + 1
            continue
        kept_norm.append(norm[index])
        kept_raw.append(raw[index])
        kept_lines.append(lines[index])
        index += 1
    return kept_norm, kept_raw, kept_lines


def without_test_modules(
    norm: list[str], raw: list[str], lines: list[int]
) -> tuple[list[str], list[str], list[int]]:
    """Drop `#[cfg(test)] mod … { … }`: example tables are not implementations."""
    kept: list[tuple[str, str, int]] = []
    index = 0
    while index < len(norm):
        if raw[index] == "#" and index + 2 < len(raw) and raw[index + 1] == "[" and raw[index + 2] == "cfg":
            # Only a `#[cfg(test)] mod … { … }` item is skipped; a `#[cfg(test)]`
            # on anything else must stay visible to the detector.
            if index + 7 < len(norm) and norm[index + 7] == "mod":
                depth = 0
                end = index
                for j in range(index, len(norm)):
                    if norm[j] == "{":
                        depth += 1
                    elif norm[j] == "}":
                        depth -= 1
                        if depth == 0:
                            end = j
                            break
                index = end + 1
                continue
        kept.append((norm[index], raw[index], lines[index]))
        index += 1
    return [k[0] for k in kept], [k[1] for k in kept], [k[2] for k in kept]


def rust_files() -> list[Path]:
    return sorted(SRC.rglob("*.rs"))


def line_numbers(text: str) -> list[int]:
    """Line number for every non-comment token produced by TOKEN_RE."""
    lines: list[int] = []
    for match in TOKEN_RE.finditer(text):
        if match.lastgroup == "comment":
            continue
        lines.append(text.count("\n", 0, match.start()) + 1)
    return lines


def top_level_items(norm: list[str], raw: list[str], lines: list[int]) -> list[tuple[str, list[str]]]:
    """Split a file into `fn` items by brace matching over the token stream."""
    items: list[tuple[str, list[str]]] = []
    index = 0
    while index < len(norm):
        if norm[index] == "fn" and index + 1 < len(norm):
            depth = 0
            end = index
            for j in range(index, len(norm)):
                if norm[j] == "{":
                    depth += 1
                elif norm[j] == "}":
                    depth -= 1
                    if depth == 0:
                        end = j
                        break
            if depth == 0 and end > index:
                name = raw[index + 1] if index + 1 < len(raw) else "?"
                line = lines[index] if index < len(lines) else 0
                items.append((f"{name}@{line}", norm[index : end + 1]))
                index = end + 1
                continue
        index += 1
    return items


def same_file_windows(
    files: dict[Path, list[str]], lines: dict[Path, list[int]]
) -> list[tuple[str, list[tuple[Path, int]]]]:
    """Windows that repeat inside one file (report-only boundary)."""
    windows: dict[str, list[tuple[Path, int]]] = defaultdict(list)
    for path, tokens in files.items():
        for start in range(0, max(0, len(tokens) - WINDOW + 1)):
            chunk = tokens[start : start + WINDOW]
            if not any(t in chunk for t in (";", "{", "}")):
                continue
            windows[hashlib.sha256(" ".join(chunk).encode()).hexdigest()].append((path, start))
    return [
        (digest, places)
        for digest, places in windows.items()
        if len({path for path, _ in places}) == 1 and len(places) > 1
    ]


def clone_blocks(files: dict[Path, list[str]], lines: dict[Path, list[int]]) -> list[str]:
    windows: dict[str, list[tuple[Path, int]]] = defaultdict(list)
    for path, tokens in files.items():
        for start in range(0, max(0, len(tokens) - WINDOW + 1)):
            chunk = tokens[start : start + WINDOW]
            if not any(t in chunk for t in (";", "{", "}")):
                continue
            digest = hashlib.sha256(" ".join(chunk).encode()).hexdigest()
            windows[digest].append((path, start))
    findings: list[str] = []
    deduped: dict[str, list[tuple[Path, int]]] = {}
    for digest, places in windows.items():
        # Two clones in the same file are still two implementations; only drop
        # windows that overlap (the same run sliding by one token).
        kept: list[tuple[Path, int]] = []
        for path, start in sorted(places):
            if any(p == path and abs(s - start) < WINDOW for p, s in kept):
                continue
            kept.append((path, start))
        deduped[digest] = kept
    for digest, places in deduped.items():
        if len({path for path, _ in places}) < 2:
            continue
        where = ", ".join(
            f"{p.relative_to(ROOT)}:{lines[p][i] if i < len(lines[p]) else '?'}" for p, i in places[:4]
        )
        findings.append(f"clone block ({WINDOW} tokens) at {where}")
    return findings


DUPLICATE_EXCEPTION_RE = re.compile(r"duplicate-exception:\s*(?P<reason>.+)")


def function_pairs(
    files: dict[Path, list[str]], raw: dict[Path, list[str]], lines: dict[Path, list[int]]
) -> tuple[list[str], list[str]]:
    """Return (failing pairs, reported exceptions)."""
    items: list[tuple[Path, str, list[str], str | None]] = []
    for path in files:
        text = path.read_text()
        source_lines = text.splitlines()
        for name, body in top_level_items(files[path], raw[path], lines[path]):
            if len(body) < MIN_FUNCTION_TOKENS:
                continue
            line = int(name.rsplit("@", 1)[1])
            reason = None
            for probe in range(max(0, line - 4), min(len(source_lines), line + 1)):
                match = DUPLICATE_EXCEPTION_RE.search(source_lines[probe])
                if match:
                    reason = match.group("reason").strip()
            items.append((path, name, body, reason))
    findings: list[str] = []
    exceptions: list[str] = []
    for i, (path_a, name_a, body_a, except_a) in enumerate(items):
        for path_b, name_b, body_b, except_b in items[i + 1 :]:
            # Length prune before the O(n^2) comparison: a pair whose lengths
            # differ beyond the ratio cannot reach the threshold.
            short, long = sorted((len(body_a), len(body_b)))
            if short / long < NEAR_RATIO - 0.08:
                continue
            ratio = difflib.SequenceMatcher(None, body_a, body_b, autojunk=False).ratio()
            if ratio < NEAR_RATIO:
                continue
            text = (
                f"function pair {path_a.relative_to(ROOT)} {name_a} <-> "
                f"{path_b.relative_to(ROOT)} {name_b} similarity={ratio:.2f} "
                f"tokens={len(body_a)}/{len(body_b)}"
            )
            if except_a or except_b:
                exceptions.append(f"{text} [exception: {except_a or except_b}]")
            else:
                findings.append(text)
    return findings, exceptions


def main() -> int:
    report = "--report" in sys.argv
    norm: dict[Path, list[str]] = {}
    raw: dict[Path, list[str]] = {}
    lines: dict[Path, list[int]] = {}
    for path in rust_files():
        text = path.read_text()
        tokens, raw_tokens = tokenize(text)
        stripped_norm, stripped_raw, stripped_lines = without_imports(
            tokens, raw_tokens, line_numbers(text)
        )
        norm[path], raw[path], lines[path] = without_test_modules(
            stripped_norm, stripped_raw, stripped_lines
        )
    if not norm:
        print("FAIL no Rust sources found — wrong root?", file=sys.stderr)
        return 1
    findings = clone_blocks(norm, lines)
    if report:
        # Same-file runs are a boundary, not a failure: report one line per file
        # with how many windows matched, so the boundary is visible without
        # drowning the reviewer in sliding-window duplicates.
        per_file: dict[str, int] = {}
        for digest, places in same_file_windows(norm, lines):
            for path, _ in places:
                per_file[str(path.relative_to(ROOT))] = per_file.get(str(path.relative_to(ROOT)), 0) + 1
        for path, count in sorted(per_file.items()):
            print(f"REPORT same-file repeated runs in {path}: {count} window(s)")
        if not per_file:
            print("REPORT no same-file repeated runs")
    pair_findings, exceptions = function_pairs(norm, raw, lines)
    findings += pair_findings
    for exception in exceptions:
        print(f"REPORT {exception}")
    if findings:
        for finding in findings:
            print(f"{'REPORT' if report else 'FAIL'} {finding}", file=sys.stderr if not report else sys.stdout)
        if not report:
            print("duplication contract failed", file=sys.stderr)
        return 0 if report else 1
    print(
        f"OK: no cross-file duplicate blocks (window {WINDOW} tokens; same-file runs "
        f"are reported only) and no "
        f"unexplained function pair >= {NEAR_RATIO:.2f} similarity "
        f"(min {MIN_FUNCTION_TOKENS} tokens); {len(exceptions)} documented exception(s)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

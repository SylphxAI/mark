#!/usr/bin/env python3
"""Modularisation gate: read budgets, god functions, exhaustive dispatch.

Structural properties of the source, checked mechanically so they cannot rot:

1. **Module read budget** — a non-test module stays within `MODULE_BUDGET` lines
   ([architecture.md](../../../owner/standards/architecture.md): ~500 ordinary,
   ~800 is the extraction signal).
2. **God functions** — no function body exceeds `FUNCTION_HARD` lines, and none
   exceeds `FUNCTION_SOFT` unless it is a recorded exception with a reason.
3. **Exhaustive dispatch** — the art vocabulary and its families must not use a
   `_ =>` catch-all, so a new art type cannot silently fall into a default arm.
4. **No stringly-typed art protocol** — the art family modules compare the
   `Art` enum, never art-id strings.
5. **Suppressions carry reasons** — an `#[allow(...)]` needs an
   `// allow-reason: …` comment on the same or the previous line.

Run: `python3 scripts/check-module-budget.py`
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "src"

MODULE_BUDGET = 800
FUNCTION_SOFT = 150
FUNCTION_HARD = 180

# path -> function name -> reason
FUNCTION_EXCEPTIONS: dict[tuple[str, str], str] = {
    (
        "src/capabilities/mark/application/hero.rs",
        "render",
    ): "hero composition gathers the type scale, line fitting, entry motion and the terminal prompt and cursor in one reading order",
}

# Only the art dispatch must be exhaustive: `Art::parse` deliberately
# normalizes unknown input to `Art::Waving`, and family functions may branch on
# style choices internally.
DISPATCH_FILE = "src/capabilities/mark/domain/shapes/mod.rs"
DISPATCH_FN = "stage"
NO_STRING_ART = "src/capabilities/mark/domain/shapes"


def rust_files() -> list[Path]:
    return sorted(SRC.rglob("*.rs"))


def without_tests(text: str) -> str:
    index = text.find("#[cfg(test)]")
    return text[:index] if index != -1 else text


def functions(text: str) -> list[tuple[str, int, int]]:
    """(name, start line, body line count) for every `fn` item."""
    lines = text.split("\n")
    found: list[tuple[str, int, int]] = []
    for index, line in enumerate(lines):
        match = re.match(r"\s*(?:pub(?:\([a-z]+\))? )?(?:const |async )?fn (\w+)", line)
        if not match:
            continue
        depth = 0
        started = False
        end = index
        while end < len(lines):
            depth += lines[end].count("{") - lines[end].count("}")
            if "{" in lines[end]:
                started = True
            if started and depth <= 0:
                break
            end += 1
        found.append((match.group(1), index + 1, end - index + 1))
    return found


def dispatch_body(text: str) -> str | None:
    """Body of the art dispatch function (the exhaustive match)."""
    match = re.search(rf"pub\(crate\) fn {DISPATCH_FN}\([^)]*\) -> \w+ \{{", text)
    if not match:
        return None
    depth = 1
    index = match.end()
    while index < len(text) and depth:
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
        index += 1
    return text[match.end():index]


def failures() -> list[str]:
    found: list[str] = []
    for path in rust_files():
        relative = str(path.relative_to(ROOT))
        text = path.read_text()
        body = without_tests(text)
        lines = body.split("\n")
        if len(lines) > MODULE_BUDGET:
            found.append(
                f"{relative}: {len(lines)} non-test lines exceeds the {MODULE_BUDGET}-line read budget"
            )
        for name, start, size in functions(body):
            exception = FUNCTION_EXCEPTIONS.get((relative, name))
            if exception:
                continue
            if size > FUNCTION_HARD:
                found.append(f"{relative}:{start}: fn {name} is {size} lines (> {FUNCTION_HARD})")
            elif size > FUNCTION_SOFT:
                found.append(
                    f"{relative}:{start}: fn {name} is {size} lines (> {FUNCTION_SOFT}); "
                    "extract it or record an exception with a reason"
                )
        if relative == DISPATCH_FILE:
            dispatch = dispatch_body(body)
            if dispatch is None:
                found.append(f"{relative}: dispatch fn {DISPATCH_FN} not found")
            elif re.search(r"^\s*_ =>", dispatch, re.M):
                found.append(
                    f"{relative}: {DISPATCH_FN} uses a `_ =>` catch-all; every Art variant "
                    "must be dispatched explicitly"
                )
        if relative.startswith(NO_STRING_ART) and re.search(r'==\s*"', body):
            found.append(f"{relative}: art comparisons must use the Art enum, not id strings")
        for index, line in enumerate(text.split("\n")):
            if "#[allow(" not in line:
                continue
            previous = text.split("\n")[index - 1] if index else ""
            if "allow-reason:" not in line and "allow-reason:" not in previous:
                found.append(f"{relative}:{index + 1}: #[allow(...)] without an `// allow-reason:` comment")
    if not rust_files():
        found.append("no Rust sources found — wrong root?")
    return found


def main() -> int:
    found = list(dict.fromkeys(failures()))
    if found:
        for item in found:
            print(f"FAIL {item}", file=sys.stderr)
        print("modularisation contract failed", file=sys.stderr)
        return 1
    print(
        f"OK: modules <= {MODULE_BUDGET} lines, functions <= {FUNCTION_SOFT} lines "
        f"({len(FUNCTION_EXCEPTIONS)} recorded exception), exhaustive art dispatch, "
        "no stringly-typed art comparisons, suppressions carry reasons"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

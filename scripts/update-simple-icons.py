#!/usr/bin/env python3
"""Regenerate `data/simple-icons.tsv` from the Simple Icons npm release.

Simple Icons (https://simpleicons.org) publishes brand glyphs as CC0 path data
on a 24x24 viewBox. Mark embeds them (ADR-0005 decision 6) through one compact
committed file that `src/capabilities/mark/domain/brand_icons.rs` loads with
`include_str!`.

Output, one icon per line, tab-separated:

    slug  hex  title  alt-keys  path

- `alt-keys` is a comma-separated list of extra lookup keys: the normalized
  forms of the icon's `aka` / `dup` alias titles, and the slug without `dot`
  (`nodedotjs` -> `nodejs`). A key is kept only when it collides with no slug
  and no other icon's key, so every key resolves to exactly one icon.
- The first line is a `#` header carrying the release version.

Run: `python3 scripts/update-simple-icons.py [version]` (default: latest).
The fetch is the npm registry tarball, never a git clone.
"""

from __future__ import annotations

import io
import json
import re
import sys
import tarfile
import unicodedata
import urllib.request
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "data" / "simple-icons.tsv"
REGISTRY = "https://registry.npmjs.org/simple-icons"

# Simple Icons' own titleToSlug replacements (sdk.mjs).
REPLACEMENTS = {
    "+": "plus", ".": "dot", "&": "and", "đ": "d", "ħ": "h", "ı": "i", "ĸ": "k",
    "ŀ": "l", "ł": "l", "ß": "ss", "ŧ": "t",
}
PATH_RE = re.compile(r' d="([^"]*)"')


def title_to_slug(title: str) -> str:
    lowered = "".join(REPLACEMENTS.get(ch, ch) for ch in title.lower())
    decomposed = unicodedata.normalize("NFD", lowered)
    return re.sub(r"[^a-z0-9]", "", decomposed)


def fetch(version: str) -> tuple[str, tarfile.TarFile]:
    with urllib.request.urlopen(f"{REGISTRY}/{version}", timeout=60) as res:
        meta = json.load(res)
    with urllib.request.urlopen(meta["dist"]["tarball"], timeout=120) as res:
        blob = res.read()
    return meta["version"], tarfile.open(fileobj=io.BytesIO(blob), mode="r:gz")


def main() -> int:
    version, tar = fetch(sys.argv[1] if len(sys.argv) > 1 else "latest")
    data = json.load(tar.extractfile("package/data/simple-icons.json"))
    icons = []
    for entry in data:
        slug = entry.get("slug") or title_to_slug(entry["title"])
        svg = tar.extractfile(f"package/icons/{slug}.svg").read().decode()
        paths = PATH_RE.findall(svg)
        if len(paths) != 1:
            raise SystemExit(f"{slug}: expected one path, found {len(paths)}")
        aliases = entry.get("aliases", {})
        alt_titles = list(aliases.get("aka", [])) + [d["title"] for d in aliases.get("dup", [])]
        alts = {title_to_slug(t) for t in alt_titles} | {title_to_slug(entry["title"])}
        if "dot" in slug:
            alts.add(slug.replace("dot", ""))
        alts.discard(slug)
        alts.discard("")
        icons.append((slug, entry["hex"].upper(), entry["title"], alts, paths[0]))

    slugs = {icon[0] for icon in icons}
    counts = Counter(key for icon in icons for key in icon[3])
    lines = [f"# simple-icons {version} (CC0-1.0) slug\thex\ttitle\talt-keys\tpath"]
    for slug, hex_, title, alts, path in sorted(icons):
        keep = sorted(k for k in alts if k not in slugs and counts[k] == 1)
        clean_title = title.replace("\t", " ")
        lines.append(f"{slug}\t{hex_}\t{clean_title}\t{','.join(keep)}\t{path}")
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(lines) + "\n")
    print(f"wrote {OUT.relative_to(ROOT)}: simple-icons {version}, {len(icons)} icons")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

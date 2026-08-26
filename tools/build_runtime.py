#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import sys
import tempfile
import zipfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parents[1]
PACKAGE_DIR = ROOT / "kotobaparse"
EXAMPLES_DIR = ROOT / "examples"
STD_DIR = ROOT / "std"
DIST_DIR = ROOT / "dist"

REQUIRED_API = [
    "load_parser",
    "load_parser_string",
    "check_parser",
    "extract_file",
    "extract_string",
    "template_file",
    "inject_file",
    "inspect_file",
    "trace_file",
    "characters_file",
]

EXCLUDE_DIR_NAMES = {
    "__pycache__",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    ".git",
}
EXCLUDE_SUFFIXES = {".pyc", ".pyo", ".tmp", ".bak"}


def read_version() -> str:
    namespace: dict[str, object] = {}
    init_path = PACKAGE_DIR / "__init__.py"
    for line in init_path.read_text(encoding="utf-8").splitlines():
        if line.startswith("__version__"):
            exec(line, namespace)
            version = namespace.get("__version__")
            if isinstance(version, str) and version.strip():
                return version.strip()
    raise RuntimeError(f"Could not find __version__ in {init_path}")


def should_skip(path: Path) -> bool:
    if any(part in EXCLUDE_DIR_NAMES for part in path.parts):
        return True
    if path.suffix.lower() in EXCLUDE_SUFFIXES:
        return True
    return False


def copy_tree(src: Path, dst: Path) -> None:
    if not src.exists():
        return
    for item in src.rglob("*"):
        rel = item.relative_to(src)
        if should_skip(rel):
            continue
        target = dst / rel
        if item.is_dir():
            target.mkdir(parents=True, exist_ok=True)
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(item, target)


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def write_zip(src_dir: Path, out_path: Path) -> None:
    out_path.parent.mkdir(parents=True, exist_ok=True)
    if out_path.exists():
        out_path.unlink()
    with zipfile.ZipFile(out_path, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as zf:
        for file in sorted(p for p in src_dir.rglob("*") if p.is_file()):
            rel = file.relative_to(src_dir)
            if should_skip(rel):
                continue
            zf.write(file, rel.as_posix())


def verify_runtime(stage: Path) -> dict[str, object]:
    sys.path.insert(0, str(stage))
    try:
        import kotobaparse  # type: ignore

        missing = [name for name in REQUIRED_API if not hasattr(kotobaparse, name)]
        if missing:
            raise RuntimeError(f"Runtime is missing API exports: {', '.join(missing)}")
        return {
            "version": getattr(kotobaparse, "__version__", ""),
            "module_path": str(Path(kotobaparse.__file__).resolve()),
            "required_api_ok": True,
        }
    finally:
        try:
            sys.path.remove(str(stage))
        except ValueError:
            pass
        for key in list(sys.modules):
            if key == "kotobaparse" or key.startswith("kotobaparse."):
                sys.modules.pop(key, None)


def build_runtime(
    *,
    version: str,
    out_dir: Path,
    min_sekai_version: str,
    api_version: str,
    download_url: str | None = None,
) -> tuple[Path, Path, dict[str, object]]:
    with tempfile.TemporaryDirectory(prefix="kotobaparse_runtime_build_") as tmp:
        stage = Path(tmp) / f"kotobaparse-runtime-{version}"
        stage.mkdir(parents=True)

        copy_tree(PACKAGE_DIR, stage / "kotobaparse")
        copy_tree(EXAMPLES_DIR, stage / "examples")
        copy_tree(EXAMPLES_DIR, stage / "bundled_parsers")
        copy_tree(STD_DIR, stage / "std")

        if (ROOT / "LICENSE").exists():
            shutil.copy2(ROOT / "LICENSE", stage / "LICENSE")
        elif (ROOT / "LICENCE").exists():
            shutil.copy2(ROOT / "LICENCE", stage / "LICENSE")

        verification = verify_runtime(stage)
        manifest: dict[str, object] = {
            "id": "kotobaparse",
            "name": "KotobaParse",
            "version": version,
            "api_version": api_version,
            "min_sekai_version": min_sekai_version,
            "entry_module": "kotobaparse",
            "required_api": REQUIRED_API,
            "layout": "sekai-runtime-v1",
            "created_at": datetime.now(timezone.utc).isoformat(),
            "verification": verification,
        }
        if download_url:
            manifest["download_url"] = download_url
        (stage / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

        out_zip = out_dir / f"kotobaparse-runtime-v{version}.zip"
        write_zip(stage, out_zip)
        checksum = sha256_file(out_zip)
        manifest["sha256"] = checksum
        manifest_path = out_dir / f"kotobaparse-runtime-v{version}.manifest.json"
        manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        return out_zip, manifest_path, manifest


def main(argv: Iterable[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Build KotobaParse runtime ZIP for SekaiTranslator.")
    parser.add_argument("--version", default=read_version(), help="Runtime version. Defaults to kotobaparse.__version__.")
    parser.add_argument("--out-dir", default=str(DIST_DIR), help="Output directory. Defaults to ./dist.")
    parser.add_argument("--min-sekai-version", default="0.6.0", help="Minimum SekaiTranslator version.")
    parser.add_argument("--api-version", default="1", help="KotobaParse runtime API version.")
    parser.add_argument("--download-url", default="", help="Optional release asset URL to embed in manifest.")
    args = parser.parse_args(list(argv) if argv is not None else None)

    if not PACKAGE_DIR.exists():
        raise SystemExit(f"Missing package directory: {PACKAGE_DIR}")

    out_zip, manifest_path, manifest = build_runtime(
        version=str(args.version),
        out_dir=Path(args.out_dir).resolve(),
        min_sekai_version=str(args.min_sekai_version),
        api_version=str(args.api_version),
        download_url=str(args.download_url).strip() or None,
    )
    print(f"Runtime ZIP: {out_zip}")
    print(f"Manifest:    {manifest_path}")
    print(f"Version:     {manifest['version']}")
    print(f"SHA256:      {manifest['sha256']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

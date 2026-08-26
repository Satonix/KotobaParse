from __future__ import annotations

import pytest

from kotobaparse.diagnostics import KotobaSyntaxError
from kotobaparse.parser import parse_string


def test_python_runtime_rejects_legacy_dsl() -> None:
    with pytest.raises(KotobaSyntaxError, match="runtime Python.*removidos"):
        parse_string('parser Old\ntarget ".txt"')


def test_python_runtime_does_not_compete_with_canonical_recipe() -> None:
    with pytest.raises(KotobaSyntaxError, match="use o executável Rust"):
        parse_string('parser Current:\n    file ".txt"\n    encoding utf8')

import pytest

from kotobaparse.diagnostics import KotobaValidationError
from kotobaparse.matcher import Matcher
from kotobaparse.parser import parse_string
from kotobaparse.validator import validate_definition


def test_validator_reports_unknown_capture_type():
    definition = parse_string('''
parser Bad
target ".txt"
rule broken
    <text:not_a_type>
    as Narration(text)
    patch text
''')
    diagnostics = validate_definition(definition)
    assert any(d.severity == "error" and "unknown capture type" in d.message for d in diagnostics)


def test_matcher_refuses_invalid_definition():
    definition = parse_string('''
parser Bad
target ".txt"
rule broken
    <text:not_a_type>
    as Narration(text)
    patch text
''')
    with pytest.raises(KotobaValidationError):
        Matcher(definition)

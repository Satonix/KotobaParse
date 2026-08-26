from pathlib import Path

from kotobaparse.parser import parse_string


def test_parse_basic_parser():
    source = '''
parser Demo
target ".txt"
encoding utf8

rule narration
    <text:line>
    as Narration(text)
    patch text
'''
    definition = parse_string(source)
    assert definition.name == "Demo"
    assert definition.target == ".txt"
    assert definition.encoding == "utf8"
    assert len(definition.rules) == 1
    assert definition.rules[0].name == "narration"


def test_parse_indexed_example():
    source = Path("examples/indexed_vn.kotoba").read_text(encoding="utf-8")
    definition = parse_string(source)
    assert definition.line_indexed is not None
    assert "voice_id" in definition.types
    assert [rule.name for rule in definition.rules] == ["asset", "marker", "dialogue", "narration"]

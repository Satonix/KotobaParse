from pathlib import Path

from kotobaparse.matcher import Matcher
from kotobaparse.parser import parse_string
from kotobaparse.reinject import inject_string


def test_reinject_simple_quoted_text():
    parser_source = Path("examples/ef_message.kotoba").read_text(encoding="utf-8")
    definition = parse_string(parser_source)
    source = '.message 10 Haruka "こんにちは"\n'
    entries = Matcher(definition).extract(source)
    patched = inject_string(source, entries, {entries[0].id: "Olá"})
    assert patched == '.message 10 Haruka "Olá"\n'


def test_reinject_indexed_narration():
    parser_source = Path("examples/indexed_vn.kotoba").read_text(encoding="utf-8")
    definition = parse_string(parser_source)
    source = '<22>A quiet afternoon.\n'
    entries = Matcher(definition).extract(source)
    patched = inject_string(source, entries, {entries[0].id: "Uma tarde de sexta-feira."})
    assert patched == '<22>Uma tarde de sexta-feira.\n'

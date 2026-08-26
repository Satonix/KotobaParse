from pathlib import Path

from kotobaparse.matcher import Matcher
from kotobaparse.parser import parse_string
from kotobaparse.reinject import inject_string, load_translations, make_translation_template
from kotobaparse.protection import protect_text, restore_text


def test_entry_ids_are_stable_across_extractions():
    parser_source = Path("examples/ef_message.kotoba").read_text(encoding="utf-8")
    definition = parse_string(parser_source)
    source = '.message 10 Haruka "こんにちは"\n'
    first = Matcher(definition).extract(source)[0].id
    second = Matcher(definition).extract(source)[0].id
    assert first == second
    assert first.startswith("dialogue:1:")


def test_reinject_escapes_quotes_inside_quoted_capture():
    parser_source = Path("examples/ef_message.kotoba").read_text(encoding="utf-8")
    definition = parse_string(parser_source)
    source = '.message 10 Haruka "こんにちは"\n'
    entries = Matcher(definition).extract(source)
    patched = inject_string(source, entries, {entries[0].id: 'Ele disse "oi"'})
    assert patched == '.message 10 Haruka "Ele disse \\"oi\\""\n'


def test_translation_template_and_list_loader(tmp_path):
    parser_source = Path("examples/ef_message.kotoba").read_text(encoding="utf-8")
    definition = parse_string(parser_source)
    source = '.message 10 Haruka "こんにちは"\n'
    entries = Matcher(definition).extract(source)
    template = make_translation_template(entries)
    assert template[0]["original"] == "こんにちは"
    template[0]["translation"] == ""

    path = tmp_path / "translations.json"
    path.write_text('[{"id": "%s", "translation": "Olá"}]' % entries[0].id, encoding="utf-8")
    assert load_translations(path) == {entries[0].id: "Olá"}


def test_protect_and_restore_text():
    parser_source = Path("examples/kirikiri.kotoba").read_text(encoding="utf-8")
    definition = parse_string(parser_source)
    protected = protect_text("Olá[r]mundo[p]", definition.protect)
    assert protected.text == "Olá<KTP_0>mundo<KTP_1>"
    assert restore_text("Oi<KTP_0>mundo<KTP_1>", protected) == "Oi[r]mundo[p]"

from pathlib import Path

from kotobaparse.encoding import read_text_preserve_newlines, write_text_preserve_newlines


def test_preserve_crlf_when_reading_and_writing(tmp_path: Path):
    path = tmp_path / "script.sc"
    path.write_bytes(b".message 1  Hello\r\n.message 2  World\r\n")
    text = read_text_preserve_newlines(path, "utf-8")
    assert "\r\n" in text

    out = tmp_path / "out.sc"
    write_text_preserve_newlines(out, text, "utf-8")
    assert out.read_bytes() == path.read_bytes()

import pytest

from kotobaparse.encoding import write_text_preserve_newlines


def test_write_text_does_not_truncate_existing_file_on_encoding_error(tmp_path: Path):
    path = tmp_path / "script.sc"
    path.write_bytes(b"original")
    with pytest.raises(UnicodeEncodeError):
        write_text_preserve_newlines(path, "Alô", "cp932")
    assert path.read_bytes() == b"original"

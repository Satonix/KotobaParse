from pathlib import Path

from kotobaparse.entries import select_entries_source
from kotobaparse.matcher import Matcher
from kotobaparse.parser import parse_string
from kotobaparse.reinject import (
    inject_string,
    load_characters,
    load_entry_speaker_translations,
    make_character_template,
    make_translation_template,
)


MULTILINGUAL = Path("examples/multilingual_bs5.kotoba").read_text(encoding="utf-8")
KIRIKIRI = Path("examples/kirikiri.kotoba").read_text(encoding="utf-8")


def test_character_template_uses_selected_multilingual_speaker():
    definition = parse_string(MULTILINGUAL)
    source = '␂【Guide】：The bell rang.␂【案内人】：「ベルが鳴った。」␂【向导】：「铃声响了。」␂【向导】：「鈴聲響了。」\n'
    entries = select_entries_source(Matcher(definition).extract(source), "jp")

    chars = make_character_template(entries, source_field="jp")

    assert chars == [
        {
            "name": "案内人",
            "translation": "",
            "occurrences": 1,
            "source_field": "jp",
            "sources": {"selected": "案内人", "en": "Guide", "jp": "案内人", "zh_cn": "向导", "zh_tw": "向导"},
        }
    ]


def test_global_character_map_can_patch_multilingual_target_speaker_from_jp_key():
    definition = parse_string(MULTILINGUAL)
    source = '␂【Guide】：The bell rang.␂【案内人】：「ベルが鳴った。」␂【向导】：「铃声响了。」␂【向导】：「鈴聲響了。」\n'
    entries = Matcher(definition).extract(source)

    patched = inject_string(source, entries, {}, characters={"案内人": "Guia"})

    assert "【Guia】：The bell rang." in patched
    assert "【案内人】：「ベルが鳴った。」" in patched


def test_template_supports_per_entry_speaker_translation():
    definition = parse_string(KIRIKIRI)
    source = '[name="Haruka"] こんにちは。\n'
    entries = Matcher(definition).extract(source)
    template = make_translation_template(entries)
    assert template[0]["speaker_translation"] == ""

    patched = inject_string(source, entries, {}, entry_speaker_translations={entries[0].id: "Haruka PT"})
    assert patched == '[name="Haruka PT"] こんにちは。\n'


def test_load_character_and_speaker_translation_json(tmp_path):
    chars_path = tmp_path / "characters.json"
    chars_path.write_text('[{"name":"案内人","translation":"Guia"}]', encoding="utf-8")
    assert load_characters(chars_path) == {"案内人": "Guia"}

    trans_path = tmp_path / "translations.json"
    trans_path.write_text('[{"id":"abc","translation":"Oi","speaker_translation":"Guia"}]', encoding="utf-8")
    assert load_entry_speaker_translations(trans_path) == {"abc": "Guia"}

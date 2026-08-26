from pathlib import Path

from kotobaparse.entries import select_entries_source
from kotobaparse.matcher import Matcher
from kotobaparse.parser import parse_string
from kotobaparse.reinject import inject_string, make_translation_template


PARSER = Path("examples/multilingual_bs5.kotoba").read_text(encoding="utf-8")


def test_multilingual_dialogue_multilingual_source_selection():
    definition = parse_string(PARSER)
    source = '␂【Guide】：The bell rang.␂【案内人】：「ベルが鳴った。」␂【向导】：「铃声响了。」␂【向导】：「鈴聲響了。」\n'
    entries = Matcher(definition).extract(source)
    assert len(entries) == 1
    assert entries[0].text == "The bell rang."
    assert entries[0].speaker == "Guide"
    jp_entries = select_entries_source(entries, "jp")
    assert jp_entries[0].text == "ベルが鳴った。"
    assert jp_entries[0].speaker == "案内人"
    template = make_translation_template(jp_entries, source_field="jp")
    assert template[0]["source_field"] == "jp"
    assert template[0]["sources"]["en"] == "The bell rang."


def test_multilingual_choice_group_splits_options_and_reinjects_english_slot():
    definition = parse_string(PARSER)
    source = '␅select 0:"Open the door" 1:"Wait outside" OUTLINE:"Choose an action"␅select 0:"扉を開ける" 1:"外で待つ" OUTLINE:"【選択肢】行動を選ぶ"␅select 0:"打开门" 1:"在外面等" OUTLINE:"选择行动"␅select 0:"打开门" 1:"在外面等" OUTLINE:"選擇行動"\n'
    entries = Matcher(definition).extract(source)
    choices = [entry for entry in entries if entry.type == "Choice"]
    assert [entry.context for entry in choices] == ["0", "1", "OUTLINE"]
    jp_choices = select_entries_source(choices, "jp")
    assert jp_choices[0].text == "扉を開ける"
    patched = inject_string(source, choices, {choices[0].id: "Chamar ela"})
    assert '0:"Chamar ela"' in patched
    assert '1:"Wait outside"' in patched
    assert '␅select 0:"扉を開ける"' in patched

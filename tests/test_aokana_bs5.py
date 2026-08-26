from pathlib import Path

from kotobaparse.entries import select_entries_source
from kotobaparse.matcher import Matcher
from kotobaparse.parser import parse_string
from kotobaparse.reinject import inject_string, make_translation_template


PARSER = Path("examples/aokana_bs5.kotoba").read_text(encoding="utf-8")


def test_aokana_dialogue_multilingual_source_selection():
    definition = parse_string(PARSER)
    source = '␂【Girl】：...I need to go now.␂【女の子】：「……そろそろ、行かなきゃ」␂【女孩】：「……我差不多该走了」␂【女孩】：「……我差不多該走了」\n'
    entries = Matcher(definition).extract(source)
    assert len(entries) == 1
    assert entries[0].text == "...I need to go now."
    assert entries[0].speaker == "Girl"
    jp_entries = select_entries_source(entries, "jp")
    assert jp_entries[0].text == "……そろそろ、行かなきゃ"
    assert jp_entries[0].speaker == "女の子"
    template = make_translation_template(jp_entries, source_field="jp")
    assert template[0]["source_field"] == "jp"
    assert template[0]["sources"]["en"] == "...I need to go now."


def test_aokana_choice_group_splits_options_and_reinjects_english_slot():
    definition = parse_string(PARSER)
    source = '␅select 0:"Call out to her" 1:"Talk to her" OUTLINE:"I will..."␅select 0:"声をかける" 1:"話しかける" OUTLINE:"【選択肢】目の前の女の子に…"␅select 0:"打招呼" 1:"搭话" OUTLINE:"对眼前的女孩子……"␅select 0:"打招呼" 1:"搭話" OUTLINE:"對眼前的女孩子……"\n'
    entries = Matcher(definition).extract(source)
    choices = [entry for entry in entries if entry.type == "Choice"]
    assert [entry.context for entry in choices] == ["0", "1", "OUTLINE"]
    jp_choices = select_entries_source(choices, "jp")
    assert jp_choices[0].text == "声をかける"
    patched = inject_string(source, choices, {choices[0].id: "Chamar ela"})
    assert '0:"Chamar ela"' in patched
    assert '1:"Talk to her"' in patched
    assert '␅select 0:"声をかける"' in patched

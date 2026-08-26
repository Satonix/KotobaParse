from pathlib import Path

from kotobaparse import load_parser, extract_file, template_file, inject_file, inspect_file, trace_file
from kotobaparse.matcher import Matcher
from kotobaparse.parser import parse_string
from kotobaparse.reinject import inject_string, make_character_template


def test_public_api_extract_template_inject(tmp_path):
    parser_path = tmp_path / "simple.kotoba"
    script_path = tmp_path / "sample.txt"
    out_path = tmp_path / "patched.txt"
    parser_path.write_text(
        '''parser Simple\ntarget ".txt"\nencoding utf8\n\nrule dialogue\n    <speaker:word>: <text:quoted>\n    as Dialogue(text, speaker)\n    patch text\n    patch speaker speaker\n''',
        encoding="utf-8",
    )
    script_path.write_text('Yuki: "Hello"\n', encoding="utf-8")
    parser = load_parser(parser_path)
    entries = extract_file(parser, script_path)
    assert entries[0].text == "Hello"
    assert template_file(parser, script_path)[0]["translation"] == ""
    inject_file(parser, script_path, {entries[0].id: "Oi"}, out_path, characters={"Yuki": "Iuki"})
    assert out_path.read_text(encoding="utf-8") == 'Iuki: "Oi"\n'


def test_target_field_can_patch_multilingual_field():
    parser = parse_string(
        '''parser Multi\ntarget ".txt"\nencoding utf8\n\nrule narration\n    ␂<en:cell>␂<jp:cell>\n    as Narration(en, jp:jp)\n    patch en\n'''
    )
    source = "␂Hello␂こんにちは\n"
    entries = Matcher(parser).extract(source)
    patched = inject_string(source, entries, {entries[0].id: "やあ"}, target_field="jp")
    assert patched == "␂Hello␂やあ\n"


def test_choice_group_target_field_patches_selected_language():
    parser = parse_string(
        '''parser MultiChoice\ntarget ".txt"\nencoding utf8\n\nrule choice_group\n    ␅select <en:cell>␅select <jp:cell>\n    as ChoiceGroup(en, jp:jp)\n    patch en\n'''
    )
    source = '␅select 0:"Yes" 1:"No"␅select 0:"はい" 1:"いいえ"\n'
    entries = Matcher(parser).extract(source)
    yes = next(entry for entry in entries if entry.context == "0")
    patched = inject_string(source, entries, {yes.id: "うん"}, target_field="jp")
    assert patched == '␅select 0:"Yes" 1:"No"␅select 0:"うん" 1:"いいえ"\n'


def test_type_values_trim_and_context_memory():
    parser = parse_string(
        '''parser Context\ntarget ".txt"\nencoding utf8\n\ntype speaker_name\n    trim\n    values\n        Yuu\n    end\n\nrule speaker\n    <speaker:speaker_name>\n    remember speaker\n    skip\n\nrule dialogue\n    <text:quoted>\n    as Dialogue(text, speaker)\n    patch text\n'''
    )
    entries = Matcher(parser).extract(' Yuu\n"Hello"\n')
    assert entries[0].speaker == "Yuu"


def test_inspect_and_trace_api(tmp_path):
    parser_path = tmp_path / "simple.kotoba"
    script_path = tmp_path / "sample.txt"
    parser_path.write_text(
        '''parser Simple\ntarget ".txt"\nencoding utf8\n\nskip empty\n\nrule narration\n    <text:line>\n    as Narration(text)\n    patch text\n''',
        encoding="utf-8",
    )
    script_path.write_text('Hello\n\n', encoding="utf-8")
    report = inspect_file(parser_path, script_path)
    assert report.entries == 1
    assert report.global_skips == 1
    trace = trace_file(parser_path, script_path)
    assert trace[0].rule == "narration"
    assert trace[1].outcome == "global_skip"


def test_character_template_collects_sources():
    parser = parse_string(
        '''parser Speaker\ntarget ".txt"\nencoding utf8\n\nrule dialogue\n    【<speaker_en:word>】<en:cell>【<speaker_jp:word>】<jp:cell>\n    as Dialogue(en, speaker_en, jp:jp, speaker_jp:speaker_jp)\n    patch en\n    patch speaker speaker_en\n'''
    )
    entries = Matcher(parser).extract("【Girl】Hello【少女】こんにちは\n")
    chars = make_character_template(entries, source_field="jp")
    assert chars[0]["name"] == "Girl"
    assert chars[0]["sources"]["jp"] == "少女"

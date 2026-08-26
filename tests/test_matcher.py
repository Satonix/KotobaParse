from pathlib import Path

from kotobaparse.matcher import Matcher
from kotobaparse.parser import parse_string


SCRIPT = '''<0>
<1>Chapter One
<2>map005
<3>sp0003a
<4>5.632
<5>The sky was bright above the city.
<28>yuki_000001
<29>Alice
<30>"Hello."
<31>yuki_000002_a
<32>Alice
<33>"This line begins here
<34>ev0001_l
<35>yuki_000002_b
<36> and then continues here,
<37>ev0001_l
<38>yuki_000002_c
<39> then moves on,
<40>yuki_000002_d
<41> and then
<42>ev0001_l
<43> finally ends here."
'''


def test_extract_indexed_script():
    parser_source = Path("examples/indexed_vn.kotoba").read_text(encoding="utf-8")
    definition = parse_string(parser_source)
    entries = Matcher(definition).extract(SCRIPT)

    assert entries[0].type == "Narration"
    assert entries[0].text == "Chapter One"
    assert any(entry.type == "Dialogue" and entry.speaker == "Alice" and entry.text == "Hello." for entry in entries)
    fragmented = [entry for entry in entries if "finally ends" in entry.text][0]
    assert fragmented.type == "Dialogue"
    assert fragmented.context == "yuki_000002_a"
    assert len(fragmented.patch.fragments) == 5


def test_ef_message_dialogue():
    parser_source = Path("examples/ef_message.kotoba").read_text(encoding="utf-8")
    definition = parse_string(parser_source)
    entries = Matcher(definition).extract('.message 10 Haruka "こんにちは"\n')
    assert len(entries) == 1
    assert entries[0].type == "Dialogue"
    assert entries[0].speaker == "Haruka"
    assert entries[0].text == "こんにちは"

from pathlib import Path

from kotobaparse.matcher import Matcher
from kotobaparse.parser import parse_string
from kotobaparse.reinject import inject_string


SCRIPT = '''<31>yuki_000002_a
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


def test_patch_fragments_default_mode_is_keep_fragments():
    source = '''
parser Demo
target ".txt"
encoding utf8
line indexed "<" <line:number> ">"
type asset_id
    pattern "^(ev)[A-Za-z0-9_]*$"
type voice_id
    pattern "^[a-zA-Z]+_[0-9]{6}(_[a-z])?$"
rule dialogue
    <voice:voice_id>
    <speaker:name>
    collect quoted -> text
        allow asset_id
        allow voice_id
    end
    as Dialogue(text, speaker, ctx:voice)
    patch fragments text
'''
    definition = parse_string(source)
    entry = Matcher(definition).extract(SCRIPT)[0]
    assert entry.patch.mode == "keep_fragments"


def test_patch_fragments_first_fragment_mode():
    source = Path("examples/indexed_vn.kotoba").read_text(encoding="utf-8").replace(
        "patch fragments text mode keep_fragments", "patch fragments text mode first_fragment"
    )
    definition = parse_string(source)
    entries = Matcher(definition).extract(SCRIPT)
    entry = entries[0]
    patched = inject_string(SCRIPT, entries, {entry.id: "Texto traduzido inteiro."})
    assert '<33>"Texto traduzido inteiro.\n' in patched
    assert '<36>\n' in patched
    assert '<43>"\n' in patched

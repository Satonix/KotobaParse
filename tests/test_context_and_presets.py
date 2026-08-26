from kotobaparse.matcher import Matcher
from kotobaparse.parser import parse_string
from kotobaparse.reinject import inject_string, make_translation_template


def test_values_trim_remembered_speaker_and_custom_quotes():
    parser = parse_string('''
parser ContextQuotes
target ".txt"
encoding utf8

use preset vn_assets

quotes
    pair "Åg" "Åh"

type speaker_name
    trim
    values
        Alice
        Bob
    end

skip empty
skip asset

rule speaker
    <speaker:speaker_name>
    remember speaker
    skip

rule dialogue
    <text:quoted>
    as Dialogue(text, speaker)
    patch text
''')
    source = 'BG052\n Bob\nÅgHello@bworldÅh\n'
    matcher = Matcher(parser)
    entries = matcher.extract(source)

    assert len(entries) == 1
    assert entries[0].speaker == 'Bob'
    assert entries[0].text == 'Hello@bworld'
    assert matcher.stats['global_skips'] == 1
    assert matcher.stats['skipped_rules']['speaker'] == 1

    patched = inject_string(source, entries, {entries[0].id: 'Olá'})
    assert 'ÅgOláÅh' in patched


def test_template_includes_protected_preview():
    parser = parse_string('''
parser ProtectTemplate
target ".txt"
encoding utf8

rule narration
    <text:line>
    as Narration(text)
    patch text

protect
    @b
''')
    entries = Matcher(parser).extract('Hello@bworld')
    template = make_translation_template(entries, protect_rules=parser.protect)
    assert template[0]['protected_preview'] == 'Hello<KTP_0>world'
    assert template[0]['protected_tokens'][0]['value'] == '@b'


def test_forget_clears_remembered_speaker():
    parser = parse_string('''
parser ForgetContext
target ".txt"
encoding utf8

type speaker_name
    values Alice

rule speaker
    <speaker:speaker_name>
    remember speaker
    skip

rule scene
    ---
    forget speaker
    skip

rule dialogue
    <text:quoted>
    as Dialogue(text, speaker)
    patch text
''')
    entries = Matcher(parser).extract('Alice\n"one"\n---\n"two"\n')
    assert entries[0].speaker == 'Alice'
    assert entries[1].speaker is None

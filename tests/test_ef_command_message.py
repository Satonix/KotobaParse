from kotobaparse.matcher import Matcher
from kotobaparse.parser import parse_string
from kotobaparse.reinject import inject_string


PARSER = '''
parser EFCommandMessage
target ".sc"
encoding cp932

type voice_id
    pattern "^[A-Za-z]{3}-[A-Za-z0-9_]+-[0-9]{4}$"

skip empty
skip unless ".message"

rule at_voiced_dialogue
    .message <id:number> <voice:voice_id> @<speaker:name> <text:quoted>
    as Dialogue(text, speaker, ctx:voice)
    patch text

rule hash_voiced_dialogue
    .message <id:number> <voice:voice_id> #<speaker:name> <text:quoted>
    as Dialogue(text, speaker, ctx:voice)
    patch text

rule voiced_quoted
    .message <id:number> <voice:voice_id> <text:quoted>
    as Dialogue(text, ctx:voice)
    patch text

rule named_dialogue
    .message <id:number> <speaker:name> <text:quoted>
    as Dialogue(text, speaker, ctx:id)
    patch text

rule voiced_narration
    .message <id:number> <voice:voice_id> <text:rest>
    as Narration(text, ctx:voice)
    patch text

rule narration
    .message <id:number> <text:rest>
    as Narration(text, ctx:id)
    patch text
'''


def test_ef_command_message_detects_dialogue_and_narration():
    definition = parse_string(PARSER)
    source = '\n'.join([
        '.message 100   I walked through the night.',
        '.message 140   Hero “It is cold.”',
        '.message 230   abc-scene-0003 @Guide “Welcome.”',
        '.message 1970 def-scene-0013 #Friend “Wait.”',
        '.message 3540 ghi-scene-0003 @Caller ≪Hello?≫',
        '.message 1980 def-scene-1043  A soft sound came from outside.',
    ])
    entries = Matcher(definition).extract(source)

    assert [entry.type for entry in entries] == [
        "Narration",
        "Dialogue",
        "Dialogue",
        "Dialogue",
        "Dialogue",
        "Narration",
    ]
    assert entries[1].speaker == "Hero"
    assert entries[1].text == "It is cold."
    assert entries[2].speaker == "Guide"
    assert entries[2].text == "Welcome."
    assert entries[3].speaker == "Friend"
    assert entries[4].speaker == "Caller"
    assert entries[4].text == "Hello?"
    assert entries[5].text == "A soft sound came from outside."


def test_ef_command_message_reinjects_inside_curly_quotes():
    definition = parse_string(PARSER)
    source = '.message 140   Hero “It is cold.”\n'
    entries = Matcher(definition).extract(source)
    output = inject_string(source, entries, {entries[0].id: 'Esta frio.'})
    assert output == '.message 140   Hero “Esta frio.”\n'


def test_ef_command_message_reinjects_inside_guillemets():
    definition = parse_string(PARSER)
    source = '.message 3540 ghi-scene-0003 @Caller ≪Hello?≫\n'
    entries = Matcher(definition).extract(source)
    output = inject_string(source, entries, {entries[0].id: 'Alo?'})
    assert output == '.message 3540 ghi-scene-0003 @Caller ≪Alo?≫\n'

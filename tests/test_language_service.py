from pathlib import Path

from kotobaparse import diagnose_file, diagnose_source, summarize_source
from kotobaparse.cli import main


SIMPLE = '''parser Simple
target ".txt"
encoding utf8

rule dialogue
    <speaker:word>: <text:quoted>
    as Dialogue(text, speaker)
    patch text
    patch speaker speaker
'''


def test_diagnose_source_returns_summary_for_valid_parser():
    report = diagnose_source(SIMPLE)

    assert report.ok is True
    data = report.to_dict()
    assert data["summary"]["name"] == "Simple"
    assert data["summary"]["rules"][0]["name"] == "dialogue"
    assert data["summary"]["rules"][0]["entry_type"] == "Dialogue"
    assert {item["name"] for item in data["summary"]["rules"][0]["captures"]} == {"speaker", "text"}


def test_diagnose_source_converts_syntax_error_to_structured_diagnostic():
    report = diagnose_source('parser Bad\ntarget ".txt"\nrule broken\n')

    assert report.ok is False
    diagnostic = report.diagnostics[0]
    assert diagnostic.severity == "error"
    assert diagnostic.line == 3
    assert "no pattern" in diagnostic.message


def test_diagnose_source_reports_validation_errors_without_throwing():
    report = diagnose_source('''parser Bad
target ".txt"
rule broken
    <text:not_a_type>
    as Narration(text)
    patch text
''')

    assert report.ok is False
    assert any("unknown capture type" in diagnostic.message for diagnostic in report.diagnostics)
    assert report.summary is not None


def test_summarize_source_includes_rule_symbols():
    summary = summarize_source(SIMPLE)
    symbols = [symbol.to_dict() for symbol in summary.symbols]

    assert {symbol["kind"] for symbol in symbols} >= {"parser", "rule"}
    assert any(symbol["name"] == "dialogue" and symbol["line"] == 5 for symbol in symbols)


def test_cli_diagnose_and_spec_commands(tmp_path, capsys):
    parser_path = tmp_path / "simple.kotoba"
    parser_path.write_text(SIMPLE, encoding="utf-8")

    assert main(["diagnose", str(parser_path)]) == 0
    diagnose_out = capsys.readouterr().out
    assert '"ok": true' in diagnose_out

    assert main(["spec", str(parser_path)]) == 0
    spec_out = capsys.readouterr().out
    assert '"rules"' in spec_out
    assert '"dialogue"' in spec_out


def test_cli_check_json_returns_nonzero_on_invalid_parser(tmp_path, capsys):
    parser_path = tmp_path / "bad.kotoba"
    parser_path.write_text('parser Bad\ntarget ".txt"\nrule broken\n', encoding="utf-8")

    assert main(["check", str(parser_path), "--json"]) == 1
    out = capsys.readouterr().out
    assert '"ok": false' in out
    assert '"line": 3' in out

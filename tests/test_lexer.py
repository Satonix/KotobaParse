from kotobaparse.lexer import lex


def test_lexer_recognizes_keywords_and_strings():
    tokens = lex('parser Demo\ntarget ".txt"\n')
    values = [(token.kind, token.value) for token in tokens]
    assert ("KEYWORD", "parser") in values
    assert ("IDENT", "Demo") in values
    assert ("STRING", '".txt"') in values

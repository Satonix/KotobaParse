# Kotoba language syntax - v0.2.0-alpha.7

KotobaParse is intended to be a small domain-specific programming language for visual novel parser authors, not just a list of regex rules.

This alpha introduces the first Rust parser for the canonical program-style syntax while keeping the older line-oriented `.kotoba` format compatible.

## Program form

```kotoba
kotoba EFMessage {
    target ".sc";
    encoding cp932;

    skip empty;
    skip unless ".message";

    rule dialogue {
        match ".message <id:number> @<speaker:name> <voice:word> <text:quoted>";
        emit Dialogue(text, speaker, ctx:voice);
        patch text;
        patch speaker speaker;
    }
}
```

## Supported statements in alpha.7

Top-level:

```kotoba
kotoba Name { ... }
name "Readable Name";
id parser_id;
target ".ext";
extension ".ext";
extensions ".ks" ".sc";
encoding cp932;
skip empty;
skip prefix "//";
skip unless ".message";
skip asset;
line indexed "<" ">";
```

Types:

```kotoba
type speaker_name {
    pattern "^(Yamato|Momoyo)$";
    value "Narrator";
    values Yamato Momoyo;
    trim;
}
```

Rules:

```kotoba
rule voiced_dialogue {
    match "@<voice:word> <text:line>";
    emit Dialogue(text, speaker, ctx:voice);
    remember speaker;
    forget speaker;
    patch text;
    patch speaker speaker;
    skip;
}
```

Quotes/protection:

```kotoba
quotes {
    pair "「" "」";
}

protect {
    literal "@n";
    pattern "@[A-Za-z0-9_]+";
}
```

## Deliberately not implemented yet

The parser accepts the language block form, but this alpha still maps program statements to the existing runtime rule model. The following are reserved and currently produce diagnostics/errors if used inside a rule:

```kotoba
let x = ...;
set state.name = ...;
when condition { ... }
if condition { ... }
```

Those will be added only after the current extraction/rebuild model has enough regression coverage.

## Compatibility rule

The old syntax remains valid:

```kotoba
parser EFMessage
target ".sc"
encoding cp932
rule dialogue
    .message <id:number> <speaker:name> <text:quoted>
    as Dialogue(text, speaker)
    patch text
```

The new syntax is the direction for the editor and documentation. The old syntax is kept for existing parsers.

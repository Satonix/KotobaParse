# Syntax Notes

KotobaParse is line-oriented in v0.1.

A rule contains pattern lines and action lines:

```kotoba
rule narration
    <text:line>
    as Narration(text)
    patch text
```

Captures use:

```kotoba
<name:type>
```

Built-in capture types:

- `line`
- `rest`
- `quoted`
- `word`
- `name`
- `number`

Custom types are declared with `type` and `pattern`.

# Rules

A `rule` describes one script structure.

```kotoba
rule choice
    @choice text=<text:quoted> target=<target:word>
    as Choice(text, ctx:target)
    patch text
```

The pattern captures data from the script. The `as` action creates an entry. The `patch` action tells the reinjector which captured span receives the translation.

Rules are ordered. Specific rules must come before generic rules like `<text:line>`.

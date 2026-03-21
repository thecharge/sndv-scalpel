# Design Decisions

## Why YAML config and not Lua config

YAML was chosen for core runtime configuration because:

- It is easy to read for non-programmers.
- It maps directly to typed Rust structs.
- It is safer for base config loading than executable scripts.

Lua is still supported as a target language for symbol discovery, and it can be added in config like any other language.

If you want scriptable behavior later, we can add optional Lua runtime plugins on top of this stable config layer.

## Why parity and safety shipped together

Parity behavior is implemented with the same safety invariants:

1. Structural selection first (`find`, `view --outline`, `view --lines`)
2. Preview-first modifications (`diff`)
3. Transaction-protected writes (`patch --apply`)
4. Structured automation via JSON outputs for all operational commands

## How diff works

`diff` builds a unified diff using the `similar` crate:

1. Parse symbols and select the scoped target.
2. Apply scoped rename to proposed content.
3. Compare old and new text.
4. Print unified patch output.

No file is written in `diff` mode.

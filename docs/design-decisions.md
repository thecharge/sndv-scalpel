# Design Decisions

## Why YAML config and not Lua config

YAML was chosen for core runtime configuration because:

- It is easy to read for non-programmers.
- It maps directly to typed Rust structs.
- It is safer for base config loading than executable scripts.

Lua is still supported as a target language for symbol discovery, and it can be added in config like any other language.

If you want scriptable behavior later, we can add optional Lua runtime plugins on top of this stable config layer.

## Why not port every advanced feature in one shot

A full one-shot port increases risk. The current path shipped:

1. Safe core behavior first (diff, patch, rollback)
2. Dynamic language setup
3. Heavy and chaos tests
4. Packaging and automation

This keeps the tool usable while reducing regression risk.

## How diff works

`diff` builds a unified diff using the `similar` crate:

1. Parse symbols and select the scoped target.
2. Apply scoped rename to proposed content.
3. Compare old and new text.
4. Print unified patch output.

No file is written in `diff` mode.

# LLM Usage Guide

## Purpose

This project is designed to be LLM-friendly by exposing deterministic, parseable command outputs and safety-first write semantics.

## Recommendations for agents

- Prefer `find` to locate symbols before proposing edits.
- Use `diff` first and show the patch to users.
- Only invoke `patch --apply` when user intent is explicit.
- Use `--json` for structured downstream reasoning.

## Example agent flow

1. `scalpel --json find 'fn:calculate*' src --recursive`
2. Select exact target symbol.
3. `scalpel diff 'fn:calculate_total' src/lib.rs --rename sum=total`
4. Ask for confirmation.
5. `scalpel patch 'fn:calculate_total' src/lib.rs --rename sum=total --apply`

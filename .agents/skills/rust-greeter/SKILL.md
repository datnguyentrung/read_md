---
name: rust-greeter
description: Generate a greeting by running a small Rust program. Use when the user asks to greet someone using Rust or test the Rust-backed skill.
---

# Rust Greeter

Use this skill when the user asks for a greeting generated with Rust.

## Workflow

1. Extract the person's name from the user's request.
2. If no name is provided, use `Developer`.
3. Run the Rust program located in `scripts/`.
4. Pass the person's name as the first CLI argument.
5. Return the program output to the user.

## Command

From the repository root, run:

```bash
cargo run --quiet \
  --manifest-path .agents/skills/rust-greeter/scripts/Cargo.toml \
  -- "<name>"
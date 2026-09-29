# Webdesign Source Forge design

## Goal

Given a product brief and optional reference URLs, help an agent produce a coherent multi-page website using existing visual sources, animation components, and permitted reconstruction workflows. The result must keep design research, implementation, and browser acceptance distinct.

## Chosen shape

The first release is a dependency-free Codex plugin with three parts:

1. `sources/catalog.json` records the sources found in the Design Space and the most useful external references, including adapter type, URLs, capabilities, and license/terms status.
2. The Rust CLI in `src/main.rs` performs deterministic source search and emits a small plan object. It is intentionally a registry helper, not a new website builder.
3. `skills/webdesign/SKILL.md` teaches the agent the four passes: research, direction, build, verify. It writes a source ledger and design tokens before code and requires a real responsive preview before acceptance.

The agent can use enabled upstream MCP servers directly. The plugin does not proxy paid services, copy proprietary assets, or add a hosted backend in v0.1.

## Flow

```text
brief + copy + references
          │
          ▼
  registry search / MCP lookup
          │
          ▼
 source ledger + license gate
          │
          ▼
  visual direction + tokens
          │
          ▼
 owned implementation in target repo
          │
          ▼
 responsive browser acceptance
```

## Error handling and boundaries

The helper rejects malformed catalogs and never treats an unverified license as reusable code. A clone request must be permissioned; a public URL alone is not permission. A source count, generated plan, or passing unit test is evidence of the helper only, not evidence that the website is implemented, deployed, or visually accepted.

## Testing

The registry search and plan builder have a small Rust unit-test check. Manual verification should run the helper against the checked-in catalog and inspect the generated JSON. Website projects using the plugin must perform their own local preview, breakpoint, accessibility, and console checks.

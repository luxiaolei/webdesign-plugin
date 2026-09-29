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

## Source gate increment for issue #1

Keep the registry-helper architecture; no hosted builder, runtime dependency or model call is added. Search uses Unicode-aware lexical runs, bounded Han bigrams and optional aliases. Plans require known or explicitly specified target platforms and filter for the intersection before limiting results. No-match does not fall back to an arbitrary catalog subset.

The source catalog adds declared platforms, dependency-inventory status, fixed-revision slot and evidence references. Discovery results preserve metadata; incompatible sources are references_only. None of these metadata assertions produce copy permission. Exact item/revision/license and target/performance/reduced-motion evidence are verified in the consuming project's source ledger. Unknown compatibility fails closed for candidate selection; empty dependencies do not mean none.

Regression evidence covers lexical matching, no match, target intersection/unknowns, rank truncation, metadata non-authority and original four-pass workflow. It does not prove the recommended source compiles, a WeChat device renders correctly, or a website has passed visual or production acceptance.

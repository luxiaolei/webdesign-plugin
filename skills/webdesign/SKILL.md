---
name: webdesign
description: Build source-backed websites from a product brief by researching visual references, selecting licensed components, creating a coherent design direction, implementing the pages, and verifying the rendered result.
---

# Webdesign Source Forge

Use this workflow when the user gives a product idea, copy direction, or a reference site and wants a polished website. The plugin coordinates sources; it does not pretend that a search result or a successful generation status is a finished website.

## Start with one brief

Extract the audience, promise, conversion goal, pages, content hierarchy, target stack, and visual constraints. If the user gives only copy, infer the missing page structure and mark assumptions in `design/brief.md`.

Run the registry helper before choosing a component:

```bash
cargo run --manifest-path <plugin-root>/Cargo.toml -- search "animated hero dark SaaS"
cargo run --manifest-path <plugin-root>/Cargo.toml -- plan \
  --brief "<product brief>" \
  --stack "nextjs" \
  --pages "home,product,case-study"
```

## Four passes

1. **Research.** Use the returned sources and any enabled MCP servers to collect a small set of references. Record the URL, source, item name, license/terms status, and intended use in `design/source-ledger.json`. Keep inspiration, copied code, and copied assets as separate rows.
2. **Direction.** Write `design/brief.md` and `design/tokens.json` before implementation. Define typography, color roles, spacing, radii, surfaces, motion principles, responsive breakpoints, and the page-by-page story. Pick one direction and one fallback; do not mix random effects from every library.
3. **Build.** Reuse source code only when the ledger allows it. Prefer registry installs or copied source that the target project owns. Adapt tokens and interaction behavior to the selected direction. Build the smallest complete responsive route before expanding the site.
4. **Verify.** Start the real local preview, inspect the rendered page at desktop and mobile widths, and check keyboard focus, reduced motion, image loading, console errors, and major route links. Save screenshots or a short acceptance note under `design/acceptance.md`.

## Clone requests

For “复刻/clone” requests, first establish that the user owns the site or has permission to reproduce it. A permitted clone should capture layout, hierarchy, interaction notes, and a source ledger, then rebuild with owned code and replaceable assets. Do not copy proprietary source, brand assets, or private content merely because a URL is public.

## Deliverable contract

The implementation is ready for review only when the target project contains:

- `design/brief.md`
- `design/tokens.json`
- `design/source-ledger.json`
- `design/acceptance.md`
- a working responsive preview with the requested routes

Keep the contract honest: a plan, a screenshot, or a passing unit check does not prove visual acceptance or deployment.

## Ordering / Chinese multi-target briefs

Use explicit targets with `plan --platforms miniprogram,web` when both WeChat and employee Web are required. `search` is discovery only. Inspect `sources` (declared compatible candidates) separately from `references_only` (inspiration or incompatible/unknown support). NO_MATCH and NO_COMPATIBLE_SOURCE are real gaps; do not turn a web animation into a native component or substitute the first catalog entries.

The lexical matcher uses bounded Chinese bigrams and curated aliases, not semantic understanding. Bind the actual item, commit/package version, license, dependencies, target build, performance, and reduced-motion/static fallback in the project source ledger. Catalog claims and nonempty links are not verified evidence or copy permission; the plan intentionally never grants it.

Prefer existing Odoo/native configuration and accepted project capabilities, then a thin adapter. Reuse the ordering state/contract and components across customer/employee surfaces, but keep platform adapters and current permissions distinct. Do not rewrite mature modules solely for perfect source sharing. Preview three variants on the same task structure before selecting a direction, then verify both actual targets; do not start runtime deployment or install paid/global services as a side effect of source discovery.

Keep original cart/spec/order/payment/refund/invoice identities through theme/layout changes. AI/camera unavailable must retain ordinary menu/service paths; a generic fallback is never a successful transaction. The plugin is a build/design helper, not a production recommendation, identity, permission or hot-code platform.

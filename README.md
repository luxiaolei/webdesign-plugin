# Webdesign Source Forge

一个给 Codex / Claude Code / Cursor 使用的网页设计插件：把“产品文案 + 想表达的东西”变成一条可复用的工作流——先找视觉素材和动效，再定统一的视觉方向，随后用拥有明确来源的组件实现，最后用真实浏览器检查响应式和细节。

它不是又一个组件库，也不是把所有网站源码偷偷抓进来。它把 MotionSites、React Bits、Uiverse、Aceternity UI，以及截图转代码、JSON-first builder、设计系统 MCP 等来源编成一个可检索的注册表，并把来源、许可、tokens、实现和验收放进同一条交付链。

## 现在能做什么

- 从 brief、页面列表和技术栈生成确定性的来源选择计划。
- 把视觉研究、设计 tokens、源码复用和浏览器验收拆成四个步骤。
- 记录 MCP、GitHub、文档、registry 和许可/条款状态。
- 对“复刻网站”要求先区分获得授权的重建与仅凭公开 URL 的模仿。

## 使用

```bash
cargo run -- search "dark animated hero React"

cargo run -- plan \
  --brief "为商业孪生产品做一个中文、多页面、客户可读的官网" \
  --stack "nextjs" \
  --pages "home,product,case-study,contact"
```

在目标网站项目中启用本插件后，要求 Agent 先生成：

```text
design/brief.md
design/tokens.json
design/source-ledger.json
design/acceptance.md
```

再开始写页面。上游 MCP 需要单独按各自文档授权；插件不会把付费服务凭空变成免费服务。

## 来源

完整清单在 [`sources/catalog.json`](sources/catalog.json)，竞品定位在 [`docs/competitive-landscape.md`](docs/competitive-landscape.md)，设计说明在 [`docs/plans/2026-09-29-webdesign-plugin-design.md`](docs/plans/2026-09-29-webdesign-plugin-design.md)。

## 校验

```bash
cargo test
cargo run -- plan --brief "animated product landing page" --stack react --pages home,pricing
```

本仓库自己的代码采用 MIT；外部组件、提示词、模板和网站素材仍以各自来源的许可和使用条款为准。

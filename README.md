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

## 中文与跨端来源筛选（#1）

中文检索采用有界汉字二元词与来源别名，不声称理解全部中文语义。无匹配明确返回空/NO_MATCH，不再固定回退五个来源。`search`仅发现来源，不承诺目标适配；`plan`才按目标筛选。

```bash
cargo run --offline -- search "微信小程序共用点单购物袋"
cargo run --offline -- plan --brief "微信小程序与员工网页共用点单组件" --stack taro --pages menu,cart,order --platforms miniprogram,web
```

多目标必须全部在目录声明范围内；未知支持只进入references_only，不进目标组件候选。网页栈默认web，taro/uni-app默认web+miniprogram；未知栈需显式--platforms。build-time工具单独检索/计划，不冒充可嵌入小程序的运行组件。

CANDIDATES_REQUIRE_REVIEW只表示来源匹配。即使目录写MIT或给出证据链接，reuse_allowed_by_this_plan始终false：项目必须逐项核版本/许可、依赖、性能、reduced-motion及真实渲染。空依赖清单是UNKNOWN，不证明零依赖。目标不匹配返回NO_COMPATIBLE_SOURCE，保留灵感参考而不偷换实现。

HZOS先核Odoo原生/配置及现有组件，再薄适配，最后补缺口；Taro/NutUI/tokens仅候选，不要求全部引入。本插件在研发期使用，不进入扫码/支付，不生成线上任意可执行代码，也不提升用户/顾客权限。

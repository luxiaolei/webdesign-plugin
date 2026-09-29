use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Catalog {
    #[serde(default)]
    pub schema_version: u32,
    #[serde(default)]
    pub checked_at: String,
    pub sources: Vec<Source>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Source {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Declared build targets, not proof of rendering or source-code reuse.
    #[serde(default)]
    pub platforms: Vec<String>,
    #[serde(default)]
    pub runtime_dependencies: Vec<String>,
    #[serde(default)]
    pub source_revision: Option<String>,
    #[serde(default)]
    pub verification: Value,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub license: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct Plan {
    pub brief: String,
    pub stack: String,
    pub pages: Vec<String>,
    pub target_platforms: Vec<String>,
    pub status: String,
    pub gaps: Vec<String>,
    pub sources: Vec<PlanSource>,
    /// Discovery-only sources excluded from target-specific implementation.
    pub references_only: Vec<PlanSource>,
    pub workflow: Vec<WorkflowStep>,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlanSource {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub url: Option<String>,
    pub why: String,
    pub license: Value,
    pub platforms: Vec<String>,
    pub runtime_dependencies: Vec<String>,
    pub source_revision: Option<String>,
    pub verification: Value,
    /// A catalog search is never a per-item code-copy approval.
    pub reuse_status: String,
    pub reuse_allowed_by_this_plan: bool,
    pub verification_gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkflowStep {
    pub id: String,
    pub output: String,
}

const RULES: [&str; 7] = [
    "reuse source code only when the recorded license or terms allow it",
    "verify source license before copying code or assets",
    "treat visual inspiration and an exact clone as different deliverables",
    "run responsive, accessibility, and console checks before calling the page done",
    "declared target compatibility is not verified rendering or runtime compatibility",
    "this plan never grants code-copy permission; bind exact item, revision and evidence in the project source ledger",
    "no match stays explicit; do not silently substitute unrelated or web-only components",
];

pub fn load_catalog(path: &Path) -> Result<Catalog, String> {
    let contents = fs::read_to_string(path).map_err(|error| format!("read catalog: {error}"))?;
    let catalog: Catalog =
        serde_json::from_str(&contents).map_err(|error| format!("parse catalog: {error}"))?;
    let mut ids = BTreeSet::new();
    for source in &catalog.sources {
        if source.id.trim().is_empty() || source.name.trim().is_empty() {
            return Err("each source needs an id and name".into());
        }
        if !ids.insert(&source.id) {
            return Err(format!("duplicate source id: {}", source.id));
        }
        if source
            .platforms
            .iter()
            .any(|p| canonical_platform(p).is_none())
        {
            return Err(format!("unknown declared platform for {}", source.id));
        }
    }
    Ok(catalog)
}

fn is_han(character: char) -> bool {
    matches!(character as u32, 0x3400..=0x4DBF | 0x4E00..=0x9FFF | 0xF900..=0xFAFF | 0x20000..=0x2FA1F)
}

/// Deterministic lexical matching, not natural-language understanding.
/// Han bigrams allow an unspaced Chinese brief to match curated aliases.
fn tokens(value: &str) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let mut run = String::new();
    let mut han = false;
    fn flush(run: &mut String, han: bool, result: &mut BTreeSet<String>) {
        let chars: Vec<char> = run.chars().collect();
        if han {
            if chars.len() >= 2 {
                result.insert(run.clone());
                for pair in chars.windows(2) {
                    result.insert(pair.iter().collect());
                }
            }
        } else if chars.len() >= 2 {
            result.insert(run.to_lowercase());
        }
        run.clear();
    }
    for character in value.chars() {
        let next_han = is_han(character);
        if !character.is_alphanumeric() {
            flush(&mut run, han, &mut result);
        } else {
            if !run.is_empty() && next_han != han {
                flush(&mut run, han, &mut result);
            }
            han = next_han;
            run.push(character);
        }
    }
    flush(&mut run, han, &mut result);
    result
}

fn matches(query: &str, source: &Source) -> usize {
    let searchable = [
        source.id.as_str(),
        source.name.as_str(),
        source.kind.as_str(),
        source.description.as_str(),
    ]
    .into_iter()
    .chain(source.tags.iter().map(String::as_str))
    .chain(source.capabilities.iter().map(String::as_str))
    .chain(source.aliases.iter().map(String::as_str))
    .collect::<Vec<_>>()
    .join(" ");
    let source_tokens = tokens(&searchable);
    tokens(query)
        .iter()
        .filter(|query_token| {
            source_tokens.iter().any(|source_token| {
                *query_token == source_token
                    || (query_token.is_ascii()
                        && source_token.is_ascii()
                        && query_token.len() >= 5
                        && source_token.len() >= 5
                        && (query_token.starts_with(source_token)
                            || source_token.starts_with(query_token.as_str())))
            })
        })
        .count()
}

pub fn search_sources(catalog: &Catalog, query: &str, limit: usize) -> Vec<Source> {
    let mut ranked = catalog
        .sources
        .iter()
        .filter_map(|source| {
            let score = matches(query, source);
            (score > 0).then_some((score, source.clone()))
        })
        .collect::<Vec<_>>();
    ranked.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| left.1.id.cmp(&right.1.id))
    });
    ranked
        .into_iter()
        .take(limit)
        .map(|(_, source)| source)
        .collect()
}

fn canonical_platform(value: &str) -> Option<&'static str> {
    match value.trim().to_lowercase().as_str() {
        "web" | "h5" | "browser" | "网页" => Some("web"),
        "miniprogram" | "wechat" | "weapp" | "微信小程序" | "小程序" => Some("miniprogram"),
        "build-time" => Some("build-time"),
        "native" => Some("native"),
        _ => None,
    }
}

fn inferred_platforms(stack: &str) -> Vec<String> {
    match stack.trim().to_lowercase().as_str() {
        "taro" | "uni-app" => vec!["web".into(), "miniprogram".into()],
        "wechat" | "weapp" | "miniprogram" | "微信小程序" => vec!["miniprogram".into()],
        "react" | "nextjs" | "next.js" | "vue" | "vite" | "html" | "web" | "h5" => {
            vec!["web".into()]
        }
        _ => Vec::new(),
    }
}

fn has_evidence(source: &Source, field: &str) -> bool {
    source
        .verification
        .get(field)
        .and_then(Value::as_str)
        .is_some_and(|s| !s.trim().is_empty())
}

fn planned_source(source: Source) -> PlanSource {
    let mut gaps = Vec::new();
    if source
        .source_revision
        .as_deref()
        .is_none_or(|s| s.trim().is_empty())
    {
        gaps.push("EXACT_SOURCE_REVISION_NOT_RECORDED".into());
    }
    for (field, gap) in [
        ("license_evidence_ref", "ITEM_LICENSE_EVIDENCE_NOT_RECORDED"),
        (
            "target_test_evidence_ref",
            "TARGET_RENDER_TEST_NOT_RECORDED",
        ),
        ("performance_evidence_ref", "PERFORMANCE_NOT_RECORDED"),
        (
            "reduced_motion_evidence_ref",
            "REDUCED_MOTION_OR_STATIC_FALLBACK_NOT_RECORDED",
        ),
    ] {
        if !has_evidence(&source, field) {
            gaps.push(gap.into());
        }
    }
    // Even nonempty evidence references must be verified for the exact target item.
    gaps.push("PROJECT_ITEM_LEDGER_REVIEW_REQUIRED".into());
    PlanSource {
        id: source.id,
        name: source.name,
        kind: if source.kind.is_empty() {
            "reference".into()
        } else {
            source.kind
        },
        url: source.url,
        why: if source.description.is_empty() {
            "curated design source".into()
        } else {
            source.description
        },
        license: source.license,
        platforms: source.platforms,
        runtime_dependencies: source.runtime_dependencies,
        source_revision: source.source_revision,
        verification: source.verification,
        reuse_status: "REVIEW_REQUIRED_NOT_COPY_PERMISSION".into(),
        reuse_allowed_by_this_plan: false,
        verification_gaps: gaps,
    }
}

/// Backwards-compatible API; callers targeting multiple platforms should use the explicit form.
pub fn build_plan(catalog: &Catalog, brief: &str, stack: &str, pages: &[String]) -> Plan {
    build_plan_for_platforms(catalog, brief, stack, pages, &inferred_platforms(stack))
}

pub fn build_plan_for_platforms(
    catalog: &Catalog,
    brief: &str,
    stack: &str,
    pages: &[String],
    platforms: &[String],
) -> Plan {
    let mut gaps = Vec::new();
    let mut target_platforms = BTreeSet::new();
    for platform in platforms {
        match canonical_platform(platform) {
            Some(p) => {
                target_platforms.insert(p.to_string());
            }
            None => gaps.push(format!("UNKNOWN_TARGET_PLATFORM:{platform}")),
        }
    }
    if target_platforms.is_empty() {
        gaps.push("TARGET_PLATFORM_REQUIRED".into());
    }
    let query = std::iter::once(brief)
        .chain(std::iter::once(stack))
        .chain(pages.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(" ");
    // Apply compatibility before truncation, so lower-ranked compatible results survive.
    let found = if tokens(brief).is_empty() {
        Vec::new()
    } else {
        search_sources(catalog, &query, catalog.sources.len())
            .into_iter()
            .filter(|source| matches(brief, source) > 0)
            .collect()
    };
    let matched = !found.is_empty();
    let valid_target = gaps.is_empty();
    let mut selected = Vec::new();
    let mut references_only = Vec::new();
    for source in found {
        let declared: BTreeSet<&str> = source
            .platforms
            .iter()
            .filter_map(|p| canonical_platform(p))
            .collect();
        let compatible = valid_target
            && target_platforms
                .iter()
                .all(|p| declared.contains(p.as_str()));
        if compatible {
            if selected.len() < 8 {
                selected.push(planned_source(source));
            }
        } else if references_only.len() < 8 {
            references_only.push(planned_source(source));
        }
    }
    let status = if !valid_target {
        "TARGET_REQUIRED_OR_INVALID"
    } else if !matched {
        "NO_MATCH"
    } else if selected.is_empty() {
        "NO_COMPATIBLE_SOURCE"
    } else {
        "CANDIDATES_REQUIRE_REVIEW"
    };
    if status == "NO_MATCH" {
        gaps.push("NO_MATCH_DO_NOT_SUBSTITUTE".into());
    }
    if status == "NO_COMPATIBLE_SOURCE" {
        gaps.push("NO_DECLARED_COMPATIBLE_SOURCE".into());
    }
    Plan {
        brief: brief.trim().into(),
        stack: stack.trim().into(),
        pages: pages
            .iter()
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect(),
        target_platforms: target_platforms.into_iter().collect(),
        status: status.into(),
        gaps,
        sources: selected,
        references_only,
        workflow: vec![
            WorkflowStep {
                id: "research".into(),
                output: "source ledger and visual references".into(),
            },
            WorkflowStep {
                id: "direction".into(),
                output: "design brief and tokens before code".into(),
            },
            WorkflowStep {
                id: "build".into(),
                output: "responsive pages using owned source code".into(),
            },
            WorkflowStep {
                id: "verify".into(),
                output: "screenshots, accessibility, and runtime checks".into(),
            },
        ],
        rules: RULES.iter().map(|r| (*r).into()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fixture() -> Catalog {
        serde_json::from_value(json!({"sources":[
            {"id":"react-bits","name":"React Bits","kind":"component-library","tags":["react","animation","background"],"aliases":["网页动效"],"platforms":["web"],"license":{"status":"verify-per-item"}},
            {"id":"taro-ui","name":"Cross-platform components","tags":["taro","react"],"aliases":["微信小程序","点单组件","购物袋"],"platforms":["miniprogram","web"],"license":{"status":"verify-repo"}},
            {"id":"inspiration","name":"Unknown support","tags":["react"],"aliases":["点单组件"],"license":{"status":"mit"}}
        ]})).unwrap()
    }

    #[test]
    fn english_search_remains_deterministic() {
        let results = search_sources(&fixture(), "animated React background", 8);
        assert_eq!(results[0].id, "react-bits");
    }
    #[test]
    fn unspaced_chinese_matches_alias() {
        assert_eq!(
            search_sources(&fixture(), "微信小程序点单购物袋", 8)[0].id,
            "taro-ui"
        );
    }
    #[test]
    fn mixed_language_matches() {
        assert_eq!(
            search_sources(&fixture(), "Taro 购物袋组件", 8)[0].id,
            "taro-ui"
        );
    }
    #[test]
    fn empty_and_unrelated_queries_return_no_sources() {
        assert!(search_sources(&fixture(), "   ", 8).is_empty());
        assert!(search_sources(&fixture(), "quasarzzzz", 8).is_empty());
    }
    #[test]
    fn no_match_has_no_fixed_fallback() {
        let p = build_plan_for_platforms(&fixture(), "quasarzzzz", "custom", &[], &["web".into()]);
        assert_eq!(p.status, "NO_MATCH");
        assert!(p.sources.is_empty() && p.references_only.is_empty());
    }
    #[test]
    fn unrelated_brief_is_not_rescued_by_stack_matches() {
        let p = build_plan(&fixture(), "quasarzzzz", "react", &["home".into()]);
        assert_eq!(p.status, "NO_MATCH");
        assert!(p.sources.is_empty());
    }
    #[test]
    fn blank_brief_does_not_select_from_stack_alone() {
        assert_eq!(build_plan(&fixture(), " ", "react", &[]).status, "NO_MATCH");
    }
    #[test]
    fn web_only_is_not_a_miniprogram_component() {
        let p = build_plan_for_platforms(
            &fixture(),
            "background animation",
            "custom",
            &[],
            &["miniprogram".into()],
        );
        assert_eq!(p.status, "NO_COMPATIBLE_SOURCE");
        assert_eq!(p.references_only[0].id, "react-bits");
    }
    #[test]
    fn all_requested_platforms_must_be_declared() {
        let p = build_plan_for_platforms(
            &fixture(),
            "点单组件",
            "react",
            &[],
            &["miniprogram".into(), "h5".into()],
        );
        assert_eq!(p.sources.len(), 1);
        assert_eq!(p.sources[0].id, "taro-ui");
        assert!(p.references_only.iter().any(|s| s.id == "inspiration"));
    }
    #[test]
    fn unknown_platform_fails_closed() {
        let p = build_plan_for_platforms(
            &fixture(),
            "点单组件",
            "react",
            &[],
            &["web".into(), "guessed-platform".into()],
        );
        assert_eq!(p.status, "TARGET_REQUIRED_OR_INVALID");
        assert!(p.sources.is_empty());
    }
    #[test]
    fn unknown_stack_requires_explicit_platform() {
        assert_eq!(
            build_plan(&fixture(), "点单组件", "unknown-stack", &[]).status,
            "TARGET_REQUIRED_OR_INVALID"
        );
    }
    #[test]
    fn missing_evidence_is_not_copy_permission() {
        let p = build_plan(&fixture(), "点单组件", "taro", &[]);
        let s = &p.sources[0];
        assert!(!s.reuse_allowed_by_this_plan);
        assert!(s
            .verification_gaps
            .contains(&"ITEM_LICENSE_EVIDENCE_NOT_RECORDED".into()));
        assert!(s
            .verification_gaps
            .contains(&"REDUCED_MOTION_OR_STATIC_FALLBACK_NOT_RECORDED".into()));
    }
    #[test]
    fn catalog_assertions_cannot_self_approve_copy() {
        let mut c = fixture();
        c.sources[1].source_revision = Some("claimed-revision".into());
        c.sources[1].verification = json!({"license_evidence_ref":"claimed","target_test_evidence_ref":"claimed","performance_evidence_ref":"claimed","reduced_motion_evidence_ref":"claimed"});
        let p = build_plan(&c, "点单组件", "taro", &[]);
        assert!(!p.sources[0].reuse_allowed_by_this_plan);
        assert!(p.sources[0]
            .verification_gaps
            .contains(&"PROJECT_ITEM_LEDGER_REVIEW_REQUIRED".into()));
    }
    #[test]
    fn compatible_candidate_is_not_lost_after_top_eight() {
        let mut c = fixture();
        for i in 0..12 {
            let mut s = c.sources[0].clone();
            s.id = format!("web-only-{i}");
            s.aliases.push("点单组件购物袋".into());
            c.sources.push(s);
        }
        let p = build_plan(&c, "React 点单组件购物袋 background animation", "taro", &[]);
        assert_eq!(p.sources.len(), 1);
        assert_eq!(p.sources[0].id, "taro-ui");
    }
    #[test]
    fn repeated_query_tokens_do_not_change_order() {
        let c = fixture();
        let ids = |q| {
            search_sources(&c, q, 8)
                .into_iter()
                .map(|s| s.id)
                .collect::<Vec<_>>()
        };
        assert_eq!(ids("React 组件"), ids("React React 组件 组件"));
    }
    #[test]
    fn plan_keeps_original_workflow() {
        let p = build_plan(
            &fixture(),
            "animated landing page",
            "react",
            &["home".into()],
        );
        assert_eq!(
            p.workflow.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
            vec!["research", "direction", "build", "verify"]
        );
        assert!(p.rules.iter().any(|s| s.contains("verify source license")));
    }
    #[test]
    fn checked_in_catalog_has_unique_valid_sources() {
        let c = load_catalog(&Path::new(env!("CARGO_MANIFEST_DIR")).join("sources/catalog.json"))
            .unwrap();
        assert!(!c.sources.is_empty());
    }
}

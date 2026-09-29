use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    pub sources: Vec<PlanSource>,
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
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkflowStep {
    pub id: String,
    pub output: String,
}

const RULES: [&str; 4] = [
    "reuse source code only when the recorded license or terms allow it",
    "verify source license before copying code or assets",
    "treat visual inspiration and an exact clone as different deliverables",
    "run responsive, accessibility, and console checks before calling the page done",
];

pub fn load_catalog(path: &Path) -> Result<Catalog, String> {
    let contents = fs::read_to_string(path).map_err(|error| format!("read catalog: {error}"))?;
    let catalog: Catalog =
        serde_json::from_str(&contents).map_err(|error| format!("parse catalog: {error}"))?;
    if catalog
        .sources
        .iter()
        .any(|source| source.id.is_empty() || source.name.is_empty())
    {
        return Err("each source needs an id and name".into());
    }
    Ok(catalog)
}

fn tokens(value: &str) -> Vec<String> {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|token| token.len() > 1)
        .map(str::to_ascii_lowercase)
        .collect()
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
    .collect::<Vec<_>>()
    .join(" ");
    let source_tokens = tokens(&searchable);
    tokens(query)
        .into_iter()
        .filter(|query_token| {
            source_tokens.iter().any(|source_token| {
                query_token == source_token
                    || (query_token.len() >= 5
                        && source_token.len() >= 5
                        && (query_token.starts_with(source_token)
                            || source_token.starts_with(query_token)))
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

pub fn build_plan(catalog: &Catalog, brief: &str, stack: &str, pages: &[String]) -> Plan {
    let query = std::iter::once(brief)
        .chain(std::iter::once(stack))
        .chain(pages.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(" ");
    let mut selected = search_sources(catalog, &query, 8);
    if selected.is_empty() {
        selected = catalog.sources.clone();
        selected.sort_by(|left, right| left.id.cmp(&right.id));
        selected.truncate(5);
    }
    Plan {
        brief: brief.trim().into(),
        stack: stack.trim().into(),
        pages: pages
            .iter()
            .map(|page| page.trim().to_string())
            .filter(|page| !page.is_empty())
            .collect(),
        sources: selected
            .into_iter()
            .map(|source| PlanSource {
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
            })
            .collect(),
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
        rules: RULES.iter().map(|rule| (*rule).into()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::{build_plan, load_catalog, search_sources};
    use std::fs;
    use std::path::PathBuf;

    fn fixture() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "webdesign-source-forge-test-{}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).expect("temporary directory");
        fs::write(
            dir.join("catalog.json"),
            r#"{"sources":[
                {"id":"react-bits","name":"React Bits","kind":"component-library","tags":["react","animation","background"],"license":{"status":"verify-per-item"}},
                {"id":"motion-sites","name":"MotionSites","kind":"prompt-library","tags":["visual-direction","landing-page"],"license":{"status":"account-required"}}
            ]}"#,
        )
        .expect("fixture catalog");
        dir
    }

    #[test]
    fn search_scores_matching_tags_deterministically() {
        let dir = fixture();
        let catalog = load_catalog(&dir.join("catalog.json")).expect("catalog");
        let results = search_sources(&catalog, "animated React hero", 8);
        assert_eq!(results[0].id, "react-bits");
    }

    #[test]
    fn plan_keeps_license_gate_and_workflow() {
        let dir = fixture();
        let catalog = load_catalog(&dir.join("catalog.json")).expect("catalog");
        let plan = build_plan(
            &catalog,
            "A dark animated landing page for a design tool",
            "nextjs",
            &["home".into(), "pricing".into()],
        );
        assert_eq!(plan.stack, "nextjs");
        assert!(plan
            .rules
            .iter()
            .any(|rule| rule.contains("verify source license")));
        assert_eq!(
            plan.workflow
                .iter()
                .map(|step| step.id.as_str())
                .collect::<Vec<_>>(),
            vec!["research", "direction", "build", "verify"]
        );
    }
}

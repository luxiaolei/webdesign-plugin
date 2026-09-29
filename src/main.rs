use std::path::{Path, PathBuf};
use webdesign_source_forge::{build_plan, load_catalog, search_sources};

fn option(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}

fn catalog_path(args: &[String]) -> PathBuf {
    option(args, "--catalog")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("sources/catalog.json"))
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  webdesign-source-forge search <query> [--catalog PATH]\n  webdesign-source-forge plan --brief TEXT --stack STACK --pages home,pricing [--catalog PATH]"
    );
    std::process::exit(2);
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let Some(command) = args.first().map(String::as_str) else {
        usage();
    };
    let catalog = match load_catalog(&catalog_path(&args)) {
        Ok(catalog) => catalog,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };
    let output = match command {
        "search" => {
            let query = args
                .iter()
                .skip(1)
                .take_while(|arg| !arg.starts_with("--"))
                .cloned()
                .collect::<Vec<_>>()
                .join(" ");
            serde_json::to_string_pretty(&search_sources(&catalog, &query, 8)).expect("json")
        }
        "plan" => {
            let brief = option(&args, "--brief").unwrap_or_else(|| usage());
            let stack = option(&args, "--stack").unwrap_or_else(|| "react".into());
            let pages = option(&args, "--pages")
                .unwrap_or_else(|| "home".into())
                .split(',')
                .map(str::to_string)
                .collect::<Vec<_>>();
            serde_json::to_string_pretty(&build_plan(&catalog, &brief, &stack, &pages))
                .expect("json")
        }
        _ => usage(),
    };
    println!("{output}");
}

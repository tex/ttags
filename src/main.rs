use ttags::*;
use clap::Parser;
use easy_parallel::Parallel;
use std::collections::HashSet;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Find references
    #[clap(long, short)]
    reference: Option<String>,

    /// Find definitions
    #[clap(long, short)]
    definition: Option<String>,

    /// Complete symbols
    #[clap(short)]
    complete: Option<String>,

    /// Path to scan
    #[clap(long, short)]
    path: Option<String>,
}

fn main()  {
    let cli = Cli::parse();

    if let Some(symbol) = cli.reference.as_deref() {
        ttags_find(false, symbol);
    } else if let Some(symbol) = cli.definition.as_deref() {
        ttags_find(true, symbol);
    } else if let Some(symbol) = cli.complete.as_deref() {
        ttags_complete(symbol);
    } else {
        let path: &str = if let Some(p) = cli.path.as_deref() { p } else { "." };
        ttags_create(path);
    }
}

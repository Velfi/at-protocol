mod lexicon_doc;
mod writer;

use writer::operation;

use anyhow::{bail, Context};
use clap::Parser;
use lexicon_doc::LexiconDoc;
use rust_code_writer::{Crate, CrateMetadata};
use std::fs::File;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the lexicon to generate the client from. This should be a folder with a period-delimited
    /// name like "atproto.com" or "bsky.app" that contains JSON files describing operations for an
    /// XRPC data repository.
    #[arg(short, long)]
    lexicon: PathBuf,

    /// Path of the directory to output the generated operations to. A directory will be created if
    /// none exists
    #[arg(short, long)]
    output_dir: PathBuf,

    /// Name of the crate to generate. This will be used as the name of the crate and the name of the
    /// module that contains the generated code.
    #[arg(short, long)]
    crate_name: String,
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    match generate_client_from_lexicon(args) {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprint!("{e}");
            std::process::exit(1)
        }
    }
}

fn crawl_directory_for_lexicon_docs(lexicon_dir: &Path) -> anyhow::Result<Vec<LexiconDoc>> {
    let mut lexicon_docs = Vec::new();

    for res in lexicon_dir.read_dir()? {
        let dir_entry = res?;

        if dir_entry.file_type()?.is_dir() {
            lexicon_docs.append(&mut crawl_directory_for_lexicon_docs(&dir_entry.path())?);
            continue;
        }

        if !dir_entry.file_name().to_string_lossy().ends_with(".json") {
            // Skip non-JSON files in the directory
            debug!(
                "Skipping non-JSON file: {}",
                dir_entry.file_name().to_string_lossy()
            );
            continue;
        }
        let file = File::open(dir_entry.path()).context("failed to open lexicon dir")?;
        debug!("Parsing lexicon file: {:?}", dir_entry.path());
        let json = serde_json::from_reader::<_, serde_json::Value>(file)
            .context("couldn't read lexicon doc from JSON")?;
        debug!("Lexicon file parsed successfully, now converting to LexiconDoc");
        let lexicon_doc = LexiconDoc::from_json(&json)?;
        lexicon_docs.push(lexicon_doc);
    }

    Ok(lexicon_docs)
}

fn generate_client_from_lexicon(args: Args) -> anyhow::Result<()> {
    let lexicon_dir = &args.lexicon;
    let lexicon_docs = crawl_directory_for_lexicon_docs(lexicon_dir)?;

    if lexicon_docs.is_empty() {
        bail!("no lexicon docs found in the given directory. please check the path and try again")
    } else {
        info!(
            "loaded {} lexicon documents from {:?}",
            lexicon_docs.len(),
            &args.lexicon
        );
    }

    // let output_dir = std::fs::canonicalize(&args.output_dir).context("couldn't canonicalize output path")?;
    let output_dir = &args.output_dir;

    if output_dir.exists() {
        if output_dir.is_file() {
            bail!("given output dir exists but is a file. please choose another output dir and rerun the command")
        }
    } else {
        std::fs::create_dir_all(&output_dir).context("couldn't create output dir")?;
    }

    debug!("generated code output dir {output_dir:?}");

    let mut new_crate = Crate::new(CrateMetadata {
        name: args.crate_name,
        description: "Generated XRPC client".to_owned(),
        version: "0.1.0".to_owned(),
        authors: vec!["Zelda Hessler <zelda.hessler@pm.me>".to_owned()],
    });

    info!(
        "found {} lexicon docs, now generating operations",
        lexicon_docs.len()
    );

    for doc in lexicon_docs {
        info!("\t{}", doc.id());
        operation::write_to_module(&mut new_crate, &doc)?;
    }

    new_crate.finalize(output_dir)?;

    Ok(())
}

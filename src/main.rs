mod templates;

use std::{
    env::{args, current_dir},
    error::Error,
    fs::{create_dir, read_to_string, File},
    io::Write,
    path::PathBuf,
    rc::Rc,
};

use cargo::{
    core::{Package, SourceId, Workspace},
    ops::{
        cargo_add::{add, AddOptions, DepOp},
        compile, init, CompileOptions, NewOptions, VersionControl,
    },
    util::{command_prelude::CompileMode, toml::TomlManifest, toml_mut::manifest::DepTable},
    Config,
};
use clap::{Parser, Subcommand};
use templates::{render_day_module, render_main};

/// Cargo plugin for generating an empty AoC Frameowrk project
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(arg_required_else_help = true)]
    /// Initialize an empty AoC crate
    Init { name: String },
}

const FRAMEWORK_VERSION: &str = "0.7.0";
const FRAMEWORK_GIT_URL: &str = "https://github.com/Michal-Miko/aoc-framework.git";
const TASK_FILES: &[&str; 3] = &["example_in", "example_out", "in"];

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = args()
        .filter(|arg| arg != &env!("CARGO_BIN_NAME").replace("cargo-", ""))
        .collect();
    let clap = Args::parse_from(args);
    match clap.command {
        Command::Init { name } => init_aoc(name),
    }
}

fn init_aoc(name: String) -> Result<(), Box<dyn Error>> {
    let path = current_dir()?.join(&name);
    println!("Creating crate {name} at {path:#?}");
    let config = Config::default()?;
    let opts = NewOptions::new(
        Some(VersionControl::Git),
        true,
        false,
        path.clone(),
        None,
        None,
        None,
    )?;
    init(&opts, &config)?;

    let manifest_path = path.join("Cargo.toml");
    let workspace = Workspace::new(&manifest_path, &config)?;
    let toml_manifest = Rc::new(toml::from_str(&read_to_string(&manifest_path)?)?);
    let (manifest, _) = TomlManifest::to_real_manifest(
        &toml_manifest,
        false,
        SourceId::for_path(&path)?,
        &path,
        &config,
    )?;
    let add_options = AddOptions {
        config: &config,
        spec: &Package::new(manifest, &manifest_path),
        dependencies: default_aoc_deps(),
        section: DepTable::default(),
        dry_run: false,
        honor_rust_version: true,
    };
    add(&workspace, &add_options)?;

    init_tasks(path.clone())?;

    let workspace = Workspace::new(&manifest_path, &config)?;
    let compile_options = CompileOptions::new(&config, CompileMode::Build)?;
    compile(&workspace, &compile_options)?;

    println!("AoC Crate `{name}` has been created and compiled successfuly!");

    Ok(())
}

fn init_tasks(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let tasks_dir = path.join("tasks");
    let src_dir = path.join("src");
    create_dir(&tasks_dir)?;

    let mut day_names = vec![];
    for day in 1..=25 {
        let day_name = format!("day_{day:0>2}");
        let day_dir = tasks_dir.join(&day_name);
        create_dir(&day_dir)?;
        for file in TASK_FILES {
            File::create(day_dir.join(file))?;
        }

        let mut day_module = File::create(src_dir.join(format!("{day_name}.rs")))?;
        day_module.write_all(render_day_module(&day_name).as_bytes())?;
        day_names.push(day_name);
    }

    let mut main = File::create(src_dir.join("main.rs"))?;
    main.write_all(render_main(&day_names).as_bytes())?;
    Ok(())
}

fn default_aoc_deps() -> Vec<DepOp> {
    vec![
        DepOp {
            crate_spec: Some("color-eyre".into()),
            rename: None,
            features: None,
            default_features: None,
            optional: None,
            registry: None,
            path: None,
            git: None,
            branch: None,
            rev: None,
            tag: None,
        },
        DepOp {
            crate_spec: Some("itertools".into()),
            rename: None,
            features: None,
            default_features: None,
            optional: None,
            registry: None,
            path: None,
            git: None,
            branch: None,
            rev: None,
            tag: None,
        },
        DepOp {
            crate_spec: Some("regex".into()),
            rename: None,
            features: None,
            default_features: None,
            optional: None,
            registry: None,
            path: None,
            git: None,
            branch: None,
            rev: None,
            tag: None,
        },
        DepOp {
            crate_spec: Some("thiserror".into()),
            rename: None,
            features: None,
            default_features: None,
            optional: None,
            registry: None,
            path: None,
            git: None,
            branch: None,
            rev: None,
            tag: None,
        },
        DepOp {
            crate_spec: Some("aoc-framework".into()),
            rename: None,
            features: None,
            default_features: None,
            optional: None,
            registry: None,
            path: None,
            git: Some(FRAMEWORK_GIT_URL.into()),
            branch: None,
            rev: None,
            tag: Some(FRAMEWORK_VERSION.into()),
        },
    ]
}

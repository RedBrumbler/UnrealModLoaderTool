use std::os::windows::process::CommandExt;

use clap::Args;
use owo_colors::OwoColorize;

use crate::data::mods_config;

#[derive(Args, Debug, Clone)]
pub struct BuildArgs {
    /// The path to the .uproject file
    pub project_path: std::path::PathBuf,
    /// Copy over the files as defined in the mods.config.json file
    #[clap(long = "copy", short = 'c')]
    pub copy: bool,
}

fn execute_build(
    project_path: std::path::PathBuf,
    out_path: std::path::PathBuf,
    engine_path: std::path::PathBuf,
) {
    let mut b = std::process::Command::new("cmd");
    let raw = format!("\"\"{}/Engine/Build/BatchFiles/RunUAT.bat\" BuildCookRun -project=\"{}\" -noP4 -platform=Win64 -clientconfig=Development -serverconfig=Development -cook -allmaps -build -stage -pak -archive -archivedirectory=\"{}\"\"", engine_path.display(), project_path.display(), out_path.display());
    println!("Raw command: {}", raw.bright_yellow());
    b.arg("/C").raw_arg(raw);
    match b.output() {
        Ok(o) => {
            for out in String::from_utf8(o.stdout).iter() {
                println!("{}", out);
            }

            for out in String::from_utf8(o.stderr).iter() {
                println!("{}", out);
            }
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

pub fn build_game(b: BuildArgs) {
    if !b.project_path.exists() {
        panic!(
            "Path {} did not exist, please provide a valid file path!",
            b.project_path.display().bright_yellow()
        );
    }

    if let Some(ext) = b.project_path.extension() {
        if ext.to_str().unwrap() != "uproject" {
            panic!(
                "File {} was not a uproject file, please provide the path to the uproject file!",
                b.project_path.display().bright_yellow()
            );
        }
    } else {
        panic!(
            "File {} did not have an extension! please provide the path to the uproject file!",
            b.project_path.display().bright_yellow()
        );
    }

    let cfg = mods_config::read().unwrap();

    let project_path_base = b.project_path.canonicalize().unwrap();
    let project_path_str = project_path_base.as_os_str().to_str().unwrap()[4..].to_string();
    let project_path = std::path::Path::new(&project_path_str).to_path_buf();
    let output_path = cfg.build_path;
    let engine_path = cfg.engine_path;

    execute_build(project_path, output_path, engine_path);

    if b.copy {
        super::copy::copy_paks();
    }
}

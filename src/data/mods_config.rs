use serde::{Deserialize, Serialize};

use super::logic_mod::LogicMod;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModsConfig {
    /// where your engine is placed
    pub engine_path: std::path::PathBuf,
    /// where to push the files
    pub output_path: std::path::PathBuf,
    /// the path to the folder where the cooked program is going to be
    pub build_path: std::path::PathBuf,
    /// What are the numbers associated with the pak files?
    pub paks: Vec<LogicMod>,
}

pub fn read() -> Result<ModsConfig, String> {
    match std::fs::File::open("mods.config.json") {
        Ok(file) => match serde_json::from_reader(&file) {
            Ok(mods_config) => Ok(mods_config),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

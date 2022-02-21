use owo_colors::OwoColorize;

use crate::data::mods_config;

pub fn copy_paks() {
    let cfg = mods_config::read().unwrap();

    let from_base = std::path::Path::new(&format!(
        "{}/WindowsNoEditor/BillieBustUp/Content/Paks",
        cfg.build_path.display()
    ))
    .to_path_buf();

    let to_base = std::path::Path::new(&cfg.output_path);

    for pak in cfg.paks.iter() {
        let from = from_base.join(pak.original_name());
        if !from.exists() {
            println!(
                "{} did not exist, not able to copy package {}!",
                from.display().bright_yellow(),
                pak.destination_name().bright_green()
            );
            continue;
        }

        let to = to_base.join(pak.destination_name());

        std::fs::copy(from, to).expect("Failed to copy file");
    }
}

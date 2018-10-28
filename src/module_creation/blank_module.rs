use std::fs::DirBuilder;
use std::fs::File;
use std::io::prelude::*;

const MODULES: [&str; 28] = [
    "troops",
    "triggers",
    "tableau",
    "strings",
    "sounds",
    "skins",
    "skills",
    "simple_triggers",
    "scripts",
    "scenes",
    "scene_props",
    "quests",
    "presentations",
    "postfx",
    "party_templates",
    "parties",
    "particle_systems",
    "music",
    "mission_templates",
    "meshes",
    "map_icons",
    "info_pages",
    "game_menus",
    "factions",
    "dialog",
    "constants",
    "animations",
    "items",
];

pub fn init_blank_module(dir: &str) -> Result<(), ()> {
    let mut builder = DirBuilder::new();

    builder.recursive(true);

    for module in MODULES.iter() {
        let mut full = String::from(dir);
        full.push('/');
        full.push_str(module);
        if builder.create(&full).is_err() {
            return Err(());
        }
    }

    let module_info_content = format!("{{ \"name\" : \"{}\" }}", dir);

    let path = format!("{}/module_info.json", dir);

    let mut file = File::create(&path).map_err(|_| {})?;
    file.write_all(module_info_content.as_bytes())
        .map_err(|_| {})?;

    Ok(())
}

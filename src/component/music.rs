use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path;
use json;

lazy_static! {
    static ref MTF_FLAG_MAP : HashMap<&'static str, u32> = {
        let mut m = HashMap::new();

        m.insert("culture_1", 0x00000001);
        m.insert("culture_2", 0x00000002);
        m.insert("culture_3", 0x00000004);
        m.insert("culture_4", 0x00000008);
        m.insert("culture_5", 0x00000010);
        m.insert("culture_6", 0x00000020);

        m.insert("culture_all", 0x0000003F);

        m.insert("looping", 0x00000040);
        m.insert("start_immediately", 0x00000080);
        m.insert("persist_until_finished", 0x00000100);

        m.insert("sit_tavern", 0x00000200);
        m.insert("sit_fight",  0x00000400);
        m.insert("sit_multiplayer_fight", 0x00000800);
        m.insert("sit_ambushed", 0x00001000);
        m.insert("sit_town",     0x00002000);
        m.insert("sit_town_infiltrate", 0x00004000);
        m.insert("sit_killed", 0x00008000);
        m.insert("sit_travel", 0x00010000);
        m.insert("sit_arena", 0x00200000);
        m.insert("sit_siege", 0x00040000);
        m.insert("sit_night", 0x00080000);
        m.insert("sit_day", 0x01000000);
        m.insert("sit_encounter_hostile", 0x00200000);
        m.insert("sit_main_title", 0x00400000);
        m.insert("sit_victorious", 0x00800000);
        m.insert("sit_feast", 0x01000000);
        m.insert("module_track", 0x10000000);

        m
    };
}

pub struct MusicTrack {
    id : String,
    file : String,
    flags : u32,
    continue_flags : u32,
}

impl MusicTrack {

    pub fn from_file(path : &path::PathBuf) -> Result<MusicTrack, String> {
        let mut file = File::open(path).map_err(|_| "Failed to open music track json file.".to_string())?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|_| "Failed to load json from file.")?;
        let object = json::parse(&content).map_err(|e| format!("Failed to parse music track json: {:?}", e))?;

        let check = check_valid_definition(&object);

        if check.is_empty() {
            Ok(MusicTrack {
                id: object["id"].as_str().unwrap().to_owned(),
                file: object["file"].as_str().unwrap().to_owned(),
                flags: collect_flags(&object["flags"]),
                continue_flags: collect_flags(&object["continue_flags"])
            })
        }
        else {
            Err(format!("Errors in music json found: {:?}", check))
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_file_path(&self) -> &str {
        &self.file
    }

    pub fn get_flags(&self) -> u32 {
        self.flags
    }

    pub fn get_continue_flags(&self) -> u32 {
        self.continue_flags
    }
}

fn check_valid_definition(data : &json::JsonValue) -> Vec<String> {
    let mut errors = vec![];

    if !data["flags"].is_array() {
        errors.push("Value 'flags' must exist as an array.".to_owned());
    }
    else {
        for i in 0..data["flags"].len() {
            let flag = &data["flags"][i];

            if let Some(s) = flag.as_str() {
                if !MTF_FLAG_MAP.contains_key(s) {
                    errors.push(format!("{} is not a valid mtf flag.", s));
                }
            }
            else {
                errors.push("All values in 'flags' must be strings representing valid flags.".to_owned());
            }
        }
    }

    if !data["continue_flags"].is_array() {
        errors.push("Value 'continue_flags' must exist as an array.".to_owned());
    }
    else {
        for i in 0..data["continue_flags"].len() {
            let flag = &data["continue_flags"][i];

            if let Some(s) = flag.as_str() {
                if !MTF_FLAG_MAP.contains_key(s) {
                    errors.push(format!("{} is not a valid mtf flag.", s));
                }
            }
            else {
                errors.push("All values in 'continue_flags' must be strings representing valid flags.".to_owned());
            }
        }
    }

    errors
}

fn collect_flags(flags : &json::JsonValue) -> u32 {
    let mut val = 0u32;
    for i in 0..flags.len() {
        val |= MTF_FLAG_MAP[flags[i].as_str().unwrap()];
    }
    val
}

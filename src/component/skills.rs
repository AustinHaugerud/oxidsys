use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path;

use json;

lazy_static! {
    static ref SKL_FLAG_MAP: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();

        m.insert("base_att_1", 0x000);
        m.insert("base_att_2", 0x001);
        m.insert("base_att_3", 0x002);
        m.insert("base_att_4", 0x003);
        m.insert("effects_party", 0x010);
        m.insert("inactive", 0x100);

        m
    };
}

pub struct SkillDefinition {
    id: String,
    name: String,
    flags: u16,
    max_level: u8,
    description: String,
}

impl SkillDefinition {
    pub fn from_file(path: &path::PathBuf) -> Result<SkillDefinition, ()> {
        let mut file = File::open(path).map_err(|_| {})?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|_| {})?;
        let object = json::parse(&content).map_err(|_| {})?;

        if is_valid_definition(&object) {
            Ok(SkillDefinition {
                id: object["id"].as_str().unwrap().to_owned(),
                name: object["name"].as_str().unwrap().to_owned(),
                flags: collect_flags(&object["flags"]),
                max_level: object["max_level"].as_u8().unwrap(),
                description: object["description"].as_str().unwrap().to_owned(),
            })
        } else {
            Err(())
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_flags(&self) -> u16 {
        self.flags
    }

    pub fn get_max_level(&self) -> u8 {
        self.max_level
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

fn collect_flags(flags: &json::JsonValue) -> u16 {
    let mut val = 0u16;

    for i in 0..flags.len() {
        let flag = flags[i].as_str().unwrap();
        let mask = SKL_FLAG_MAP[flag];
        val |= mask;
    }

    val
}

fn is_valid_definition(data: &json::JsonValue) -> bool {
    let valid_flag_config = {
        data["flags"].is_array()
            && all_flags_valid(&data["flags"])
            && count_base_flags(&data["flags"]) == 1
    };

    let valid_max_level = {
        if let Some(val) = data["max_level"].as_u64() {
            (val >= 1) && (val <= 15)
        } else {
            false
        }
    };

    let valid_id = {
        if let Some(s) = data["id"].as_str() {
            is_valid_id(s)
        } else {
            false
        }
    };

    let name_description_defined = data["name"].is_string() && data["description"].is_string();

    valid_flag_config && valid_max_level && valid_id && name_description_defined
}

// Make sure the id doesn't contain whitespace and is all lowercase.
fn is_valid_id(id: &str) -> bool {
    let whitespace_split: Vec<_> = id.split_whitespace().collect();
    whitespace_split.len() == 1 && (id.to_lowercase() == id)
}

// This is used to catch if the user specifies multiple base
// attributes or none at all. A valid configuration
// should have a base attribute flag count such that
// count_base_flags(flags) == 1.
fn count_base_flags(flags: &json::JsonValue) -> u16 {
    let mut count = 0u16;

    for i in 0..flags.len() {
        if let Some(s) = flags[i].as_str() {
            // Use all_flags_valid before calling this so we know
            // any flag prefixed with base is really referencing a valid attribute
            if s.starts_with("base") {
                count += 1;
            }
        }
    }

    count
}

// Make sure that all elements in the flags array
// are valid strings and also not fictitious unrecognizable flags.
fn all_flags_valid(flags: &json::JsonValue) -> bool {
    for i in 0..flags.len() {
        if let Some(s) = flags[i].as_str() {
            if !SKL_FLAG_MAP.contains_key(s) {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

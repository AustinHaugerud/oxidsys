use std::fs;

use component::skills::SkillDefinition;

pub fn load_skills(skills_dir : &str) -> Result<Vec<SkillDefinition>, String> {
    let paths =
        fs::read_dir(skills_dir).map_err(|_| "Failed to open skills directory".to_owned())?;

    let mut definitions : Vec<SkillDefinition> = vec![];

    for path in paths {
        let entry = path.map_err(|e| format!("DirEntry error: {}", e))?;

        if entry.path().is_file() {
            let skill = SkillDefinition::from_file(&entry.path()).map_err(|_| format!("Error loading skill from {:?}.", entry.path()))?;
            definitions.push(skill);
        }
    }

    Ok(definitions)
}

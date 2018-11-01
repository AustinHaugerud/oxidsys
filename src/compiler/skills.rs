use compiler::Compile;
use loader::skills;
use compiler::common::replace_spaces;

use std::fs::File;
use std::fs;

pub struct SkillsCompiler {
    skills_dir : String,
}

impl SkillsCompiler {
    pub fn new(skills_dir : &str) -> SkillsCompiler {
        SkillsCompiler {
            skills_dir : skills_dir.to_owned()
        }
    }
}

impl Compile for SkillsCompiler {
    fn compile(&self) -> Result<(), String> {

        let skill_definitions = skills::load_skills(&self.skills_dir)?;

        let mut content = format!("{}\n", skill_definitions.len());

        for skill in skill_definitions {
            content += &format!("skl_{} {} ", skill.get_id(), replace_spaces(skill.get_name()));
            content += &format!("{} {} {}\n", skill.get_flags(), skill.get_max_level(), adjust_description(skill.get_description()));
        }

        fs::write("skills.txt", content).map_err(|_| "Failed to write to skills.txt".to_owned())?;

        Ok(())
    }
}

fn adjust_description(descr : &str) -> String {
    descr.replace(" ", "_")
}

mod common;
mod items;
mod skills;

pub trait Compile {
    fn compile(&self) -> Result<(), String>;
}

pub fn load_skills_compiler(dir: &str) -> Box<Compile> {
    Box::new(skills::SkillsCompiler::new(dir))
}

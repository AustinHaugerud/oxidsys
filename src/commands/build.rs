use std::collections::HashMap;
use compiler::*;

fn load_compiler(id : &str) -> Option<Box<Compile>> {
    match id {
        "skills" => Some(load_skills_compiler("skills/")),
        _ => None
    }
}

pub fn execute_compiler(id : &str) -> Result<(), String> {
    if let Some(compiler) = load_compiler(id) {
        compiler.compile()
    }
    else {
        Err("Invalid target.".to_owned())
    }
}
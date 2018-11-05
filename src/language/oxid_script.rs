use std::path;
use std::fs;


pub struct OperandCall {
    op_id : Vec<String>,
    params : Vec<String>,
}

pub struct Script {
    op_calls : Vec<OperandCall>,
}

impl Script {
    pub fn from_file(path : &path::PathBuf) -> Result<Script,String> {

        let file = fs::File::open(path).map_err(|_| format!("Failed to open {:?}", path))?;


        Err("".to_owned())
    }
}

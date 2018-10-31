use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StopAllSoundsOp;

const DOC : &str = "Stops all playing sounds. Version 1.153 options: 0 = stop only looping sounds, 1 = stop all sounds. Version 1.143 options: 0 = let current track finish, 1 = fade it out, 2 = stop it abruptly.";

pub const OP_CODE: u32 = 609;

pub const IDENT: &str = "stop_all_sounds";

impl Operation for StopAllSoundsOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<options>", "")],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OmitKeyOnceOp;

const DOC : &str = "Forces the game to ignore default bound action for the specified game key on current game frame.";

pub const OP_CODE: u32 = 77;

pub const IDENT: &str = "omit_key_once";

impl Operation for OmitKeyOnceOp {
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
            param_docs: vec![make_param_doc("<key_code>", "")],
        }
    }
}

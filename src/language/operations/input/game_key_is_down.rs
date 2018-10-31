use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GameKeyIsDownOp;

const DOC : &str = "Checks that the specified game key is currently pressed. See header_triggers.py for game key code reference.";

pub const OP_CODE: u32 = 72;

pub const IDENT: &str = "game_key_is_down";

impl Operation for GameKeyIsDownOp {
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
            param_docs: vec![make_param_doc("<game_key_code>", "")],
        }
    }
}

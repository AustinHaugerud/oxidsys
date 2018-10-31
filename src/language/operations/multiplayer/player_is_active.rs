use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerIsActiveOp;

const DOC: &str = "Checks that the specified player is active (i.e. connected to server).";

pub const OP_CODE: u32 = 401;

pub const IDENT: &str = "player_is_active";

impl Operation for PlayerIsActiveOp {
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
            param_docs: vec![make_param_doc("<player_id>", "")],
        }
    }
}

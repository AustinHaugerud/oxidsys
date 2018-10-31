use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerSetIsAdminOp;

const DOC: &str =
    "Server operation. Set the current player as admin (value = 1) or not (value = 0).";

pub const OP_CODE: u32 = 429;

pub const IDENT: &str = "player_set_is_admin";

impl Operation for PlayerSetIsAdminOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<player_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

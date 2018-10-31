use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerIsAdminOp;

const DOC: &str = "Checks that the specified player has administrative rights.";

pub const OP_CODE: u32 = 430;

pub const IDENT: &str = "player_is_admin";

impl Operation for PlayerIsAdminOp {
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

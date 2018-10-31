use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct BanPlayerOp;

const DOC : &str = "Official docs: set value = 1 for banning temporarily, assign 2nd player id as the administrator player id if banning is permanent";

pub const OP_CODE: u32 = 466;

pub const IDENT: &str = "ban_player";

impl Operation for BanPlayerOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<player_id>", ""),
                make_param_doc("<value>", ""),
                make_param_doc("<player_id>", ""),
            ],
        }
    }
}

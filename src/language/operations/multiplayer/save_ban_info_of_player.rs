use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SaveBanInfoOfPlayerOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 467;

pub const IDENT: &str = "save_ban_info_of_player";

impl Operation for SaveBanInfoOfPlayerOp {
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

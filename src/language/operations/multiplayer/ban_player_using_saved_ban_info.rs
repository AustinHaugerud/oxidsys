use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct BanPlayerUsingSavedBanInfoOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 468;

pub const IDENT: &str = "ban_player_using_saved_ban_info";

impl Operation for BanPlayerUsingSavedBanInfoOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}

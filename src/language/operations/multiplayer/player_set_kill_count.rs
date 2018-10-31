use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerSetKillCountOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 434;

pub const IDENT: &str = "player_set_kill_count";

impl Operation for PlayerSetKillCountOp {
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

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerSetIsMutedOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 440;

pub const IDENT: &str = "player_set_is_muted";

impl Operation for PlayerSetIsMutedOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<player_id>", ""),
                make_param_doc("<value>", ""),
                make_param_doc("[mute_for_everyone]", ""),
            ],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetAchievementStatOp;

const DOC : &str = "Retrieves the numeric value associated with an achievement. Used to keep track of player's results before finally unlocking it.";

pub const OP_CODE: u32 = 370;

pub const IDENT: &str = "get_achievement_stat";

impl Operation for GetAchievementStatOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<achievement_id>", ""),
                make_param_doc("<stat_index>", ""),
            ],
        }
    }
}

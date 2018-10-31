use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetAchievementStatOp;

const DOC : &str = "Sets the new value associated with an achievement. Used to keep track of player's results before finally unlocking it.";

pub const OP_CODE: u32 = 371;

pub const IDENT: &str = "set_achievement_stat";

impl Operation for SetAchievementStatOp {
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
                make_param_doc("<achievement_id>", ""),
                make_param_doc("<stat_index>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

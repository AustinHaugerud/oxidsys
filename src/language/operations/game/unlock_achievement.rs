use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct UnlockAchievementOp;

const DOC: &str = "Unlocks player's achievement. Apparently doesn't have any game effects.";

pub const OP_CODE: u32 = 372;

pub const IDENT: &str = "unlock_achievement";

impl Operation for UnlockAchievementOp {
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
            param_docs: vec![make_param_doc("<achievement_id>", "")],
        }
    }
}

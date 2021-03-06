use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopAddProficiencyPointsOp;

const DOC: &str =
    "Adds some proficiency points to a hero troop which can later be distributed by player.";

pub const OP_CODE: u32 = 1525;

pub const IDENT: &str = "troop_add_proficiency_points";

impl Operation for TroopAddProficiencyPointsOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

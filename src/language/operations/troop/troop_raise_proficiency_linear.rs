use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopRaiseProficiencyLinearOp;

const DOC : &str = "Same as (troop_raise_proficiency), but does not take Weapon Master skill into account (i.e. can increase proficiencies indefinitely).";

pub const OP_CODE: u32 = 1523;

pub const IDENT: &str = "troop_raise_proficiency_linear";

impl Operation for TroopRaiseProficiencyLinearOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<proficiency_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

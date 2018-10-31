use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopRaiseProficiencyOp;

const DOC : &str = "Increases troop weapon proficiency by the specified value. Value can be negative. Increase is subject to limits defined by Weapon Master skill. When used on non-hero troop, will affect all instances of that troop.";

pub const OP_CODE: u32 = 1522;

pub const IDENT: &str = "troop_raise_proficiency";

impl Operation for TroopRaiseProficiencyOp {
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

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopRaiseSkillOp;

const DOC : &str = "Increases troop skill by the specified value. Value can be negative. See header_skills.py for reference. When used on non-hero troop, will affect all instances of that troop.";

pub const OP_CODE: u32 = 1521;

pub const IDENT: &str = "troop_raise_skill";

impl Operation for TroopRaiseSkillOp {
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
                make_param_doc("<skill_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreSkillLevelOp;

const DOC: &str = "Stores current value of troop skill. See header_skills.py for reference.";

pub const OP_CODE: u32 = 2170;

pub const IDENT: &str = "store_skill_level";

impl Operation for StoreSkillLevelOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<skill_id>", ""),
                make_param_doc("[troop_id]", ""),
            ],
        }
    }
}

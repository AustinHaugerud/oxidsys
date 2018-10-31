use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreProficiencyLevelOp;

const DOC : &str = "Stores current value of troop weapon proficiency. See wpt_* constants in header_troops.py for reference.";

pub const OP_CODE: u32 = 2176;

pub const IDENT: &str = "store_proficiency_level";

impl Operation for StoreProficiencyLevelOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<attribute_id>", ""),
            ],
        }
    }
}

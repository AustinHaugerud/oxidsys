use language::operations::{Operation, ParamInfo};

pub struct EncounterAttackOp;

const DOC: &str = "Apparently starts the standard battle with the encountered party. 4research.";

pub const OP_CODE: u32 = 1302;

pub const IDENT: &str = "encounter_attack";

impl Operation for EncounterAttackOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}

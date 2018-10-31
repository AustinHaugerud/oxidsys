use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OptionsSetCombatAiOp;

const DOC: &str = "0 = good, 1 = average, 2 = poor";

pub const OP_CODE: u32 = 265;

pub const IDENT: &str = "options_set_combat_ai";

impl Operation for OptionsSetCombatAiOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}

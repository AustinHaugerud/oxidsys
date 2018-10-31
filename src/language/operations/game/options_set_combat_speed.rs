use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OptionsSetCombatSpeedOp;

const DOC: &str = "0 = slowest, 1 = slower, 2 = normal, 3 = faster, 4 = fastest";

pub const OP_CODE: u32 = 269;

pub const IDENT: &str = "options_set_combat_speed";

impl Operation for OptionsSetCombatSpeedOp {
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

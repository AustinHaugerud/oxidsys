use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OptionsGetCombatSpeedOp;

const DOC: &str = "0 = slowest, 1 = slower, 2 = normal, 3 = faster, 4 = fastest";

pub const OP_CODE: u32 = 268;

pub const IDENT: &str = "options_get_combat_speed";

impl Operation for OptionsGetCombatSpeedOp {
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
            param_docs: vec![make_param_doc("<destination>", "")],
        }
    }
}

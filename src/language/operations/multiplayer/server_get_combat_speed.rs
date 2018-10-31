use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ServerGetCombatSpeedOp;

const DOC: &str = "Official docs: 0-2";

pub const OP_CODE: u32 = 478;

pub const IDENT: &str = "server_get_combat_speed";

impl Operation for ServerGetCombatSpeedOp {
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

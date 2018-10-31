use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct MissionSetDuelModeOp;

const DOC: &str = "Sets duel mode for the multiplayer mission. Values: 0 = off, 1 = on.";

pub const OP_CODE: u32 = 2006;

pub const IDENT: &str = "mission_set_duel_mode";

impl Operation for MissionSetDuelModeOp {
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

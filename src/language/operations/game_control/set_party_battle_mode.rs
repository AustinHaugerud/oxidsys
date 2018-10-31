use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetPartyBattleModeOp;

const DOC : &str = "Used before or during the mission to start battle mode (and apparently make agents use appropriate AI).";

pub const OP_CODE: u32 = 1020;

pub const IDENT: &str = "set_party_battle_mode";

impl Operation for SetPartyBattleModeOp {
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

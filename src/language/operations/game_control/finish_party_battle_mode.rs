use language::operations::{Operation, ParamInfo};

pub struct FinishPartyBattleModeOp;

const DOC: &str = "Used during the mission to stop battle mode.";

pub const OP_CODE: u32 = 1019;

pub const IDENT: &str = "finish_party_battle_mode";

impl Operation for FinishPartyBattleModeOp {
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

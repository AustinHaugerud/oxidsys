use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct GetPlayerAgentNoOp;

const DOC: &str = "Retrieves the reference to the player-controlled agent. Singleplayer mode only.";

pub const OP_CODE: u32 = 1700;

pub const IDENT: &str = "get_player_agent_no";

impl Operation for GetPlayerAgentNoOp {
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

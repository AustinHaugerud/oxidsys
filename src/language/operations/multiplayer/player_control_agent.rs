use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerControlAgentOp;

const DOC : &str = "Server operation. Puts the agent under specified player's control. Operation will change agent's face code and banner to those of player.";

pub const OP_CODE: u32 = 421;

pub const IDENT: &str = "player_control_agent";

impl Operation for PlayerControlAgentOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<player_id>", ""),
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}

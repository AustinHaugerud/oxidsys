use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetDefendActionOp;

const DOC : &str = "Forces the agent to perform a defend action. Possible values: -2 = cancel any action (1.153+), 0 = defend_down, 1 = defend_right, 2 = defend_left, 3 = defend_up. Does time value determine delay, speed or duration? 4research.";

pub const OP_CODE: u32 = 1746;

pub const IDENT: &str = "agent_set_defend_action";

impl Operation for AgentSetDefendActionOp {
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
                make_param_doc("<agent_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

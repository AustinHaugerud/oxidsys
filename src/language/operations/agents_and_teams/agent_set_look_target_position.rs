use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetLookTargetPositionOp;

const DOC : &str = "Forces the agent to look at specified position (turn his head as necessary). Alarmed agents will ignore this.";

pub const OP_CODE: u32 = 1744;

pub const IDENT: &str = "agent_set_look_target_position";

impl Operation for AgentSetLookTargetPositionOp {
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
                make_param_doc("<position>", ""),
            ],
        }
    }
}

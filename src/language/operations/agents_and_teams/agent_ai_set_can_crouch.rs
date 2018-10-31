use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentAiSetCanCrouchOp;

const DOC: &str = "Version 1.153+. Allows or forbids the agent to crouch. 0 to forbid, 1 to allow.";

pub const OP_CODE: u32 = 2083;

pub const IDENT: &str = "agent_ai_set_can_crouch";

impl Operation for AgentAiSetCanCrouchOp {
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

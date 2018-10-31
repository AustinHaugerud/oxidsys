use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentIsActiveOp;

const DOC : &str = "Checks that the agent reference is active. This will succeed for dead or routed agents, for as long as the agent reference itself is valid.";

pub const OP_CODE: u32 = 1712;

pub const IDENT: &str = "agent_is_active";

impl Operation for AgentIsActiveOp {
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
            param_docs: vec![make_param_doc("<agent_id>", "")],
        }
    }
}

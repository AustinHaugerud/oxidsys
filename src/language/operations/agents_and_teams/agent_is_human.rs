use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentIsHumanOp;

const DOC: &str = "Checks that the agent is human (i.e. not horse).";

pub const OP_CODE: u32 = 1704;

pub const IDENT: &str = "agent_is_human";

impl Operation for AgentIsHumanOp {
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

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentClearRelationsWithAgentsOp;

const DOC: &str = "Clears any agent-to-agent relations for specified agent.";

pub const OP_CODE: u32 = 1802;

pub const IDENT: &str = "agent_clear_relations_with_agents";

impl Operation for AgentClearRelationsWithAgentsOp {
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

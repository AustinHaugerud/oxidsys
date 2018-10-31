use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentAddRelationWithAgentOp;

const DOC : &str = "Changes relations between two agents on the scene to enemy (value = -1), neutral (value = 0), ally (value = 1). Note that neutral agents are immune to friendly fire.";

pub const OP_CODE: u32 = 1803;

pub const IDENT: &str = "agent_add_relation_with_agent";

impl Operation for AgentAddRelationWithAgentOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<agent_id>", ""),
                make_param_doc("<agent_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

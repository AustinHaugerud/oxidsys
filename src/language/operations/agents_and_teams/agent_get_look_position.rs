use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetLookPositionOp;

const DOC: &str = "Retrieves the position that the agent is currently looking at.";

pub const OP_CODE: u32 = 1709;

pub const IDENT: &str = "agent_get_look_position";

impl Operation for AgentGetLookPositionOp {
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
                make_param_doc("<position>", ""),
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}

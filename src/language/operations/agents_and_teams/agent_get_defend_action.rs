use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetDefendActionOp;

const DOC : &str = "Retrieves agent's current defend action. Possible values: free = 0, parrying = 1, blocking = 2.";

pub const OP_CODE: u32 = 1764;

pub const IDENT: &str = "agent_get_defend_action";

impl Operation for AgentGetDefendActionOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<agent_id>", ""),
            ],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetKillCountOp;

const DOC : &str = "Retrieves the total number of kills by the specified agent during this battle. Call with non-zero <get_wounded> parameter to retrieve the total number of enemies the agent has knocked down.";

pub const OP_CODE: u32 = 1723;

pub const IDENT: &str = "agent_get_kill_count";

impl Operation for AgentGetKillCountOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<agent_id>", ""),
                make_param_doc("[get_wounded]", ""),
            ],
        }
    }
}

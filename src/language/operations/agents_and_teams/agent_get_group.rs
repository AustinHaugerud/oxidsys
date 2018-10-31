use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetGroupOp;

const DOC : &str = "Retrieves reference to player who is currently the leader of specified bot agent. Only works in multiplayer.";

pub const OP_CODE: u32 = 1765;

pub const IDENT: &str = "agent_get_group";

impl Operation for AgentGetGroupOp {
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

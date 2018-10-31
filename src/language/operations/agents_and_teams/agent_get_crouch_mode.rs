use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetCrouchModeOp;

const DOC: &str = "Version 1.153+. Retrieves agent's crouch status (1 = crouching, 0 = standing).";

pub const OP_CODE: u32 = 2097;

pub const IDENT: &str = "agent_get_crouch_mode";

impl Operation for AgentGetCrouchModeOp {
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

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetCrouchModeOp;

const DOC: &str = "Version 1.153+. Sets agent's crouch status (1 = crouch, 0 = stand up).";

pub const OP_CODE: u32 = 2098;

pub const IDENT: &str = "agent_set_crouch_mode";

impl Operation for AgentSetCrouchModeOp {
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

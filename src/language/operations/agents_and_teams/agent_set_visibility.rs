use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetVisibilityOp;

const DOC: &str = "Version 1.153+. Sets agent visibility. 0 for invisible, 1 for visible.";

pub const OP_CODE: u32 = 2096;

pub const IDENT: &str = "agent_set_visibility";

impl Operation for AgentSetVisibilityOp {
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

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetNoDynamicsOp;

const DOC : &str = "Makes the agent stand on the spot (value = 1) or move normally (value = 0). When frozen on the spot the agent can still turn around and fight if necessary. Used in Native for the wedding scene.";

pub const OP_CODE: u32 = 1762;

pub const IDENT: &str = "agent_set_no_dynamics";

impl Operation for AgentSetNoDynamicsOp {
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

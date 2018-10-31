use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentIsRoutedOp;

const DOC : &str = "Checks that the agent has fled from the map (i.e. reached the edge of the map in fleeing mode and then faded).";

pub const OP_CODE: u32 = 1699;

pub const IDENT: &str = "agent_is_routed";

impl Operation for AgentIsRoutedOp {
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

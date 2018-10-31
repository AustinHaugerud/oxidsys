use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentHasItemEquippedOp;

const DOC: &str = "Checks that the agent has a specific item equipped.";

pub const OP_CODE: u32 = 1729;

pub const IDENT: &str = "agent_has_item_equipped";

impl Operation for AgentHasItemEquippedOp {
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
                make_param_doc("<item_id>", ""),
            ],
        }
    }
}

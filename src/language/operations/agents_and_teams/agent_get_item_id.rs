use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetItemIdOp;

const DOC: &str = "Retrieves the item type of the specified horse agent. Returns -1 for humans.";

pub const OP_CODE: u32 = 1719;

pub const IDENT: &str = "agent_get_item_id";

impl Operation for AgentGetItemIdOp {
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
                make_param_doc("<horse_agent_id>", ""),
            ],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentSetWieldedItemOp;

const DOC : &str = "Forces the agent to wield the specified item. Agent must have that item in his equipment for this to work. Use item_id = -1 to unwield any currently wielded item.";

pub const OP_CODE: u32 = 1747;

pub const IDENT: &str = "agent_set_wielded_item";

impl Operation for AgentSetWieldedItemOp {
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

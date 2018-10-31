use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentUnequipItemOp;

const DOC : &str = "Removes the specified item from the agent. Optional parameter weapon_slot_no is in range 1..4 and determines what weapon slot to remove (item_id must still be set correctly).";

pub const OP_CODE: u32 = 1774;

pub const IDENT: &str = "agent_unequip_item";

impl Operation for AgentUnequipItemOp {
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
                make_param_doc("<agent_id>", ""),
                make_param_doc("<item_id>", ""),
                make_param_doc("[weapon_slot_no]", ""),
            ],
        }
    }
}

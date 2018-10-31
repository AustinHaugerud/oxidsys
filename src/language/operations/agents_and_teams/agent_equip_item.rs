use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentEquipItemOp;

const DOC : &str = "Adds the specified item to agent and forces him to equip it. Optional weapon_slot_no parameter is only used with weapons and will put the newly added item to that slot (range 1..4). If it is omitted with a weapon item, then the agent must have an empty weapon slot for the operation to succeed.";

pub const OP_CODE: u32 = 1779;

pub const IDENT: &str = "agent_equip_item";

impl Operation for AgentEquipItemOp {
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

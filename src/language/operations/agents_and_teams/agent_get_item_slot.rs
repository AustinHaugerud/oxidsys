use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetItemSlotOp;

const DOC : &str = "Retrieves item_id for specified agent's slot Possible slot values range in 0..7, order is weapon1, weapon2, weapon3, weapon4, head_armor, body_armor, leg_armor, hand_armor.";

pub const OP_CODE: u32 = 1804;

pub const IDENT: &str = "agent_get_item_slot";

impl Operation for AgentGetItemSlotOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<agent_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

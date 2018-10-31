use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AgentGetWieldedItemOp;

const DOC : &str = "Retrieves the item reference that the agent is currently wielding in his right hand (hand_no = 0) or left hand (hand_no = 1). Note that weapons are always wielded in right hand, and shield in left hand. When wielding a two-handed weapon (including bows and crossbows), this operation will return -1 for left hand.";

pub const OP_CODE: u32 = 1726;

pub const IDENT: &str = "agent_get_wielded_item";

impl Operation for AgentGetWieldedItemOp {
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
                make_param_doc("<hand_no>", ""),
            ],
        }
    }
}

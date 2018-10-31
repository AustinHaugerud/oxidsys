use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerItemSlotIsPickedUpOp;

const DOC : &str = "Checks that the specified player's equipment slot contains an item that the player has picked up from ground.";

pub const OP_CODE: u32 = 461;

pub const IDENT: &str = "player_item_slot_is_picked_up";

impl Operation for PlayerItemSlotIsPickedUpOp {
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
                make_param_doc("<player_id>", ""),
                make_param_doc("<item_slot_no>", ""),
            ],
        }
    }
}

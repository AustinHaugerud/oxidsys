use language::operations::Operation;

pub struct PlayerItemSlotIsPickedUpOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 461;

pub const IDENT: &str = "player_item_slot_is_picked_up";

impl Operation for PlayerItemSlotIsPickedUpOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}

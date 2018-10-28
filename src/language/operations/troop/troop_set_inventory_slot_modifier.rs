use language::operations::Operation;

pub struct TroopSetInventorySlotModifierOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1544;

pub const IDENT: &str = "troop_set_inventory_slot_modifier";

impl Operation for TroopSetInventorySlotModifierOp {
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

use language::operations::Operation;

pub struct TroopInventorySlotSetItemAmountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1534;

pub const IDENT: &str = "troop_inventory_slot_set_item_amount";

impl Operation for TroopInventorySlotSetItemAmountOp {
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

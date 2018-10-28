use language::operations::Operation;

pub struct TroopInventorySlotGetItemMaxAmountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1538;

pub const IDENT: &str = "troop_inventory_slot_get_item_max_amount";

impl Operation for TroopInventorySlotGetItemMaxAmountOp {
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

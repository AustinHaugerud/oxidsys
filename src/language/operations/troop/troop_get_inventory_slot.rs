use language::operations::Operation;

pub struct TroopGetInventorySlotOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1541;

pub const IDENT: &str = "troop_get_inventory_slot";

impl Operation for TroopGetInventorySlotOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}

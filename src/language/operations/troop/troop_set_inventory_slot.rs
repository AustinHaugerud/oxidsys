use language::operations::Operation;

pub struct TroopSetInventorySlotOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1543;

pub const IDENT: &str = "troop_set_inventory_slot";

impl Operation for TroopSetInventorySlotOp {
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

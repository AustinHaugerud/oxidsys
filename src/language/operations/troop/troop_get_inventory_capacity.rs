use language::operations::Operation;

pub struct TroopGetInventoryCapacityOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1540;

pub const IDENT: &str = "troop_get_inventory_capacity";

impl Operation for TroopGetInventoryCapacityOp {
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

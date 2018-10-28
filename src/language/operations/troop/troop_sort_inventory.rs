use language::operations::Operation;

pub struct TroopSortInventoryOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1511;

pub const IDENT: &str = "troop_sort_inventory";

impl Operation for TroopSortInventoryOp {
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

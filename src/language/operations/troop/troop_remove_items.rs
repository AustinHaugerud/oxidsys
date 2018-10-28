use language::operations::Operation;

pub struct TroopRemoveItemsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1536;

pub const IDENT: &str = "troop_remove_items";

impl Operation for TroopRemoveItemsOp {
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

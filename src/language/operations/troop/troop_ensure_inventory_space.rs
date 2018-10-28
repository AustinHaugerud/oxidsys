use language::operations::Operation;

pub struct TroopEnsureInventorySpaceOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 1510;

pub const IDENT: &str = "troop_ensure_inventory_space";

impl Operation for TroopEnsureInventorySpaceOp {
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

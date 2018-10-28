use language::operations::Operation;

pub struct StoreFreeInventoryCapacityOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2167;

pub const IDENT: &str = "store_free_inventory_capacity";

impl Operation for StoreFreeInventoryCapacityOp {
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

use language::operations::Operation;

pub struct StoreTroopHealthOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2175;

pub const IDENT: &str = "store_troop_health";

impl Operation for StoreTroopHealthOp {
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

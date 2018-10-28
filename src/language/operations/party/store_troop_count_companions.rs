use language::operations::Operation;

pub struct StoreTroopCountCompanionsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2160;

pub const IDENT: &str = "store_troop_count_companions";

impl Operation for StoreTroopCountCompanionsOp {
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

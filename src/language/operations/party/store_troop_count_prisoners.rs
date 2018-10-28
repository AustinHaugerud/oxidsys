use language::operations::Operation;

pub struct StoreTroopCountPrisonersOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2161;

pub const IDENT: &str = "store_troop_count_prisoners";

impl Operation for StoreTroopCountPrisonersOp {
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

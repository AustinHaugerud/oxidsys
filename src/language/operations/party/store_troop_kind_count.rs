use language::operations::Operation;

pub struct StoreTroopKindCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2158;

pub const IDENT: &str = "store_troop_kind_count";

impl Operation for StoreTroopKindCountOp {
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

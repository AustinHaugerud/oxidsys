use language::operations::Operation;

pub struct ItemHasCapabilityOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2724;

pub const IDENT: &str = "item_has_capability";

impl Operation for ItemHasCapabilityOp {
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

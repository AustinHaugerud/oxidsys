use language::operations::Operation;

pub struct ItemGetAccuracyOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2711;

pub const IDENT: &str = "item_get_accuracy";

impl Operation for ItemGetAccuracyOp {
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

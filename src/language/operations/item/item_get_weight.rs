use language::operations::Operation;

pub struct ItemGetWeightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2700;

pub const IDENT: &str = "item_get_weight";

impl Operation for ItemGetWeightOp {
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

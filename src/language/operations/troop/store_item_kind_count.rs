use language::operations::Operation;

pub struct StoreItemKindCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2165;

pub const IDENT: &str = "store_item_kind_count";

impl Operation for StoreItemKindCountOp {
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

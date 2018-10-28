use language::operations::Operation;

pub struct StrStoreItemNameByCountOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2327;

pub const IDENT: &str = "str_store_item_name_by_count";

impl Operation for StrStoreItemNameByCountOp {
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

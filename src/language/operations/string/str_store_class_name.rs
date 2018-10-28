use language::operations::Operation;

pub struct StrStoreClassNameOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2346;

pub const IDENT: &str = "str_store_class_name";

impl Operation for StrStoreClassNameOp {
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

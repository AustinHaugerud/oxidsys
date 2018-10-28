use language::operations::Operation;

pub struct StrStoreStringRegOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2321;

pub const IDENT: &str = "str_store_string_reg";

impl Operation for StrStoreStringRegOp {
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

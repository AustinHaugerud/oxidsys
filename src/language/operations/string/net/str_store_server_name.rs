use language::operations::Operation;

pub struct StrStoreServerNameOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2352;

pub const IDENT: &str = "str_store_server_name";

impl Operation for StrStoreServerNameOp {
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

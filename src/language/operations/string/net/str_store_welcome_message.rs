use language::operations::Operation;

pub struct StrStoreWelcomeMessageOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2353;

pub const IDENT: &str = "str_store_welcome_message";

impl Operation for StrStoreWelcomeMessageOp {
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

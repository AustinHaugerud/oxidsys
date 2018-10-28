use language::operations::Operation;

pub struct StrEncodeUrlOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2355;

pub const IDENT: &str = "str_encode_url";

impl Operation for StrEncodeUrlOp {
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

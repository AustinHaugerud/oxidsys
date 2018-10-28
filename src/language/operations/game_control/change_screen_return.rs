use language::operations::Operation;

pub struct ChangeScreenReturnOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2040;

pub const IDENT: &str = "change_screen_return";

impl Operation for ChangeScreenReturnOp {
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

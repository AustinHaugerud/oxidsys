use language::operations::Operation;

pub struct DisplayDebugMessageOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1104;

pub const IDENT: &str = "display_debug_message";

impl Operation for DisplayDebugMessageOp {
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

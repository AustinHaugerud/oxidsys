use language::operations::Operation;

pub struct TryBeginOp;

const DOC: &str = r#"
Opens a conditional block.
Format: try_begin;
"#;

pub const OP_CODE: u16 = 4;

pub const IDENT: &str = "try_begin";

impl Operation for TryBeginOp {
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

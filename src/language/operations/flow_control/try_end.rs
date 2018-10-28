use language::operations::Operation;

pub struct TryEndOp;

const DOC: &str = r#"
Concluded conditional branches.
Format: try_end;
"#;

pub const OP_CODE: u16 = 3;

const IDENT: &str = "try_end";

impl Operation for TryEndOp {
    fn op_code(&self) -> u16 {
        OP_CODE
    }

    fn documentation(&self) -> &str {
        DOC
    }

    fn identifier(&self) -> &str {
        IDENT
    }
}

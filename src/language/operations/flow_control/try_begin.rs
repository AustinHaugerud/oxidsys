use language::operations::Operation;

pub struct TryBeginOp;

const DOC: &str = r#"
Opens a conditional block.
Format: try_begin;
"#;

const IDENT: &str = "try_begin";

impl Operation for TryBeginOp {
    fn op_code(&self) -> u16 {
        unimplemented!()
    }

    fn documentation(&self) -> &str {
        unimplemented!()
    }

    fn identifier(&self) -> &str {
        IDENT
    }
}

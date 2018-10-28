use language::operations::Operation;

pub struct ElseTryOp;

const DOC: &str = r#"
Functions as else or else if in conditional flow.
Format: else_try;
"#;

pub const OP_CODE: u16 = 5;

pub const IDENT: &str = "else_try;";

impl Operation for ElseTryOp {
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

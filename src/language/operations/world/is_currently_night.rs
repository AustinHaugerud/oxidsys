use language::operations::Operation;

pub struct IsCurrentlyNightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2273;

pub const IDENT: &str = "is_currently_night";

impl Operation for IsCurrentlyNightOp {
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

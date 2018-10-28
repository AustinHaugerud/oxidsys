use language::operations::Operation;

pub struct ResetVisitorsOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1262;

pub const IDENT: &str = "reset_visitors";

impl Operation for ResetVisitorsOp {
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

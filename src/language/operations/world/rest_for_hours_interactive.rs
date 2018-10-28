use language::operations::Operation;

pub struct RestForHoursInteractiveOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1031;

pub const IDENT: &str = "rest_for_hours_interactive";

impl Operation for RestForHoursInteractiveOp {
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

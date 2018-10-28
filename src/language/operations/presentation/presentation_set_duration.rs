use language::operations::Operation;

pub struct PresentationSetDurationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 902;

pub const IDENT: &str = "presentation_set_duration";

impl Operation for PresentationSetDurationOp {
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

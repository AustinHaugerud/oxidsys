use language::operations::Operation;

pub struct StartBackgroundPresentationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 901;

pub const IDENT: &str = "start_background_presentation";

impl Operation for StartBackgroundPresentationOp {
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

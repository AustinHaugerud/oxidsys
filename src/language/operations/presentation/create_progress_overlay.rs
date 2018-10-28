use language::operations::Operation;

pub struct CreateProgressOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 915;

pub const IDENT: &str = "create_progress_overlay";

impl Operation for CreateProgressOverlayOp {
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

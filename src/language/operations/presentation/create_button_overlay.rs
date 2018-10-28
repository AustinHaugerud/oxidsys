use language::operations::Operation;

pub struct CreateButtonOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 912;

pub const IDENT: &str = "create_button_overlay";

impl Operation for CreateButtonOverlayOp {
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

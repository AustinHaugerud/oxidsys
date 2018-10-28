use language::operations::Operation;

pub struct CreateImageButtonOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 913;

pub const IDENT: &str = "create_image_button_overlay";

impl Operation for CreateImageButtonOverlayOp {
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

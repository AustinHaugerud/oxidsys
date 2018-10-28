use language::operations::Operation;

pub struct CreateSliderOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 914;

pub const IDENT: &str = "create_slider_overlay";

impl Operation for CreateSliderOverlayOp {
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

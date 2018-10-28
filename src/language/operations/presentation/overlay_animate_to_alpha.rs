use language::operations::Operation;

pub struct OverlayAnimateToAlphaOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 933;

pub const IDENT: &str = "overlay_animate_to_alpha";

impl Operation for OverlayAnimateToAlphaOp {
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

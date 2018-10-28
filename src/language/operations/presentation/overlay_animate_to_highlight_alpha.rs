use language::operations::Operation;

pub struct OverlayAnimateToHighlightAlphaOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 935;

pub const IDENT: &str = "overlay_animate_to_highlight_alpha";

impl Operation for OverlayAnimateToHighlightAlphaOp {
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

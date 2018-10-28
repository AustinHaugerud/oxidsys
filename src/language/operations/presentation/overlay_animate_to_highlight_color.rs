use language::operations::Operation;

pub struct OverlayAnimateToHighlightColorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 934;

pub const IDENT: &str = "overlay_animate_to_highlight_color";

impl Operation for OverlayAnimateToHighlightColorOp {
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

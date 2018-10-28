use language::operations::Operation;

pub struct OverlayAnimateToColorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 932;

pub const IDENT: &str = "overlay_animate_to_color";

impl Operation for OverlayAnimateToColorOp {
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

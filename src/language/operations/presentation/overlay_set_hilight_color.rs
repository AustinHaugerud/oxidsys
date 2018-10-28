use language::operations::Operation;

pub struct OverlaySetHilightColorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 923;

pub const IDENT: &str = "overlay_set_hilight_color";

impl Operation for OverlaySetHilightColorOp {
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

use language::operations::Operation;

pub struct OverlaySetHilightAlphaOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 924;

pub const IDENT: &str = "overlay_set_hilight_alpha";

impl Operation for OverlaySetHilightAlphaOp {
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

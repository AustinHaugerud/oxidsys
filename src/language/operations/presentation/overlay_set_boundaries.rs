use language::operations::Operation;

pub struct OverlaySetBoundariesOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 928;

pub const IDENT: &str = "overlay_set_boundaries";

impl Operation for OverlaySetBoundariesOp {
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

use language::operations::Operation;

pub struct OverlaySetDisplayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 947;

pub const IDENT: &str = "overlay_set_display";

impl Operation for OverlaySetDisplayOp {
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

use language::operations::Operation;

pub struct OverlaySetAlphaOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 922;

pub const IDENT: &str = "overlay_set_alpha";

impl Operation for OverlaySetAlphaOp {
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

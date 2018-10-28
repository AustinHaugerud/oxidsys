use language::operations::Operation;

pub struct OverlaySetTextOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 920;

pub const IDENT: &str = "overlay_set_text";

impl Operation for OverlaySetTextOp {
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

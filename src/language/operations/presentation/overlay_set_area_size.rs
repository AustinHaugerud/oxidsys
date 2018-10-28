use language::operations::Operation;

pub struct OverlaySetAreaSizeOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 929;

pub const IDENT: &str = "overlay_set_area_size";

impl Operation for OverlaySetAreaSizeOp {
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

use language::operations::Operation;

pub struct OverlaySetAdditionalRenderHeightOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 952;

pub const IDENT: &str = "overlay_set_additional_render_height";

impl Operation for OverlaySetAdditionalRenderHeightOp {
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

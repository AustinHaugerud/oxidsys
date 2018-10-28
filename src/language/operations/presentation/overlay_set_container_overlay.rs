use language::operations::Operation;

pub struct OverlaySetContainerOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 951;

pub const IDENT: &str = "overlay_set_container_overlay";

impl Operation for OverlaySetContainerOverlayOp {
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

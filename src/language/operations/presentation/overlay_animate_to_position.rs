use language::operations::Operation;

pub struct OverlayAnimateToPositionOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 937;

pub const IDENT: &str = "overlay_animate_to_position";

impl Operation for OverlayAnimateToPositionOp {
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

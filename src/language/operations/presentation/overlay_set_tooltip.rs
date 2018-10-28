use language::operations::Operation;

pub struct OverlaySetTooltipOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 950;

pub const IDENT: &str = "overlay_set_tooltip";

impl Operation for OverlaySetTooltipOp {
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

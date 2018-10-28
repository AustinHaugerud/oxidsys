use language::operations::Operation;

pub struct SetTooltipTextOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1130;

pub const IDENT: &str = "set_tooltip_text";

impl Operation for SetTooltipTextOp {
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

use language::operations::Operation;

pub struct CreateSimpleTextBoxOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 919;

pub const IDENT: &str = "create_simple_text_box_overlay";

impl Operation for CreateSimpleTextBoxOverlayOp {
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

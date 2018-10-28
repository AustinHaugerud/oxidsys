use language::operations::Operation;

pub struct CreateTextBoxOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 917;

pub const IDENT: &str = "create_text_box_overlay";

impl Operation for CreateTextBoxOverlayOp {
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

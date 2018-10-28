use language::operations::Operation;

pub struct CreateCheckBoxOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 918;

pub const IDENT: &str = "create_check_box_overlay";

impl Operation for CreateCheckBoxOverlayOp {
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

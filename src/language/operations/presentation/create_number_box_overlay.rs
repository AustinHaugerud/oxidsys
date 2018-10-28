use language::operations::Operation;

pub struct CreateNumberBoxOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 942;

pub const IDENT: &str = "create_number_box_overlay";

impl Operation for CreateNumberBoxOverlayOp {
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

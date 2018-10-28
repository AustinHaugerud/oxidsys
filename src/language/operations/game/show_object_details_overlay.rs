use language::operations::Operation;

pub struct ShowObjectDetailsOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 960;

pub const IDENT: &str = "show_object_details_overlay";

impl Operation for ShowObjectDetailsOverlayOp {
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

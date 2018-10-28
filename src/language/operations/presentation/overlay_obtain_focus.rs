use language::operations::Operation;

pub struct OverlayObtainFocusOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 949;

pub const IDENT: &str = "overlay_obtain_focus";

impl Operation for OverlayObtainFocusOp {
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

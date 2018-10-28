use language::operations::Operation;

pub struct IsZoomDisabledOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 2222;

pub const IDENT: &str = "is_zoom_disabled";

impl Operation for IsZoomDisabledOp {
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

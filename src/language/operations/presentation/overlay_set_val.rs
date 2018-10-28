use language::operations::Operation;

pub struct OverlaySetValOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 927;

pub const IDENT: &str = "overlay_set_val";

impl Operation for OverlaySetValOp {
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

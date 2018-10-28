use language::operations::Operation;

pub struct IsEditModeEnabledOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 255;

pub const IDENT: &str = "is_edit_mode_enabled";

impl Operation for IsEditModeEnabledOp {
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

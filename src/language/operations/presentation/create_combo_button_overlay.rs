use language::operations::Operation;

pub struct CreateComboButtonOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 916;

pub const IDENT: &str = "create_combo_button_overlay";

impl Operation for CreateComboButtonOverlayOp {
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

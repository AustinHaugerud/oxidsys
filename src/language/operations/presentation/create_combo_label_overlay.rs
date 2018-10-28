use language::operations::Operation;

pub struct CreateComboLabelOverlayOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 948;

pub const IDENT: &str = "create_combo_label_overlay";

impl Operation for CreateComboLabelOverlayOp {
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

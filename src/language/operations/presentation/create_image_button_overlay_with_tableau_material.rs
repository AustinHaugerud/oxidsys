use language::operations::Operation;

pub struct CreateImageButtonOverlayWithTableauMaterialOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 938;

pub const IDENT: &str = "create_image_button_overlay_with_tableau_material";

impl Operation for CreateImageButtonOverlayWithTableauMaterialOp {
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

use language::operations::Operation;

pub struct CreateMeshOverlayWithTableauMaterialOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 939;

pub const IDENT: &str = "create_mesh_overlay_with_tableau_material";

impl Operation for CreateMeshOverlayWithTableauMaterialOp {
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

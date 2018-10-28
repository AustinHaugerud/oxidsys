use language::operations::Operation;

pub struct CreateMeshOverlayWithItemIdOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 944;

pub const IDENT: &str = "create_mesh_overlay_with_item_id";

impl Operation for CreateMeshOverlayWithItemIdOp {
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

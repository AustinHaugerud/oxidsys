use language::operations::Operation;

pub struct OverlaySetMeshRotationOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 930;

pub const IDENT: &str = "overlay_set_mesh_rotation";

impl Operation for OverlaySetMeshRotationOp {
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

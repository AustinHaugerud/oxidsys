use language::operations::Operation;

pub struct AiMeshFaceGroupShowHideOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 1805;

pub const IDENT: &str = "ai_mesh_face_group_show_hide";

impl Operation for AiMeshFaceGroupShowHideOp {
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

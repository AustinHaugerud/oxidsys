use language::operations::Operation;

pub struct CurTableauAddMeshWithScaleAndVertexColorOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u16 = 2000;

pub const IDENT: &str = "cur_tableau_add_mesh_with_scale_and_vertex_color";

impl Operation for CurTableauAddMeshWithScaleAndVertexColorOp {
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

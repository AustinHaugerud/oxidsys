use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauAddMeshWithScaleAndVertexColorOp;

const DOC : &str = "Similar to (cur_tableau_add_mesh_with_vertex_color), but allows non-uniform scaling. Scale factors are stored as (x,y,z) position properties with fixed point values.";

pub const OP_CODE: u32 = 2000;

pub const IDENT: &str = "cur_tableau_add_mesh_with_scale_and_vertex_color";

impl Operation for CurTableauAddMeshWithScaleAndVertexColorOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 5,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<mesh_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("<scale_position>", ""),
                make_param_doc("<value_fixed_point>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

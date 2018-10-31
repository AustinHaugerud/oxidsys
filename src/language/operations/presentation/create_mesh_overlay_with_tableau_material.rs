use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CreateMeshOverlayWithTableauMaterialOp;

const DOC : &str = "Creates a mesh overlay, using the specified tableau_material. When mesh_id = -1, it is generated automatically. Value is passed as the parameter for tableau_material script. Returns overlay_id.";

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

    fn param_info(&self) -> ParamInfo {
        ParamInfo {
            num_required: 4,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<mesh_id>", ""),
                make_param_doc("<tableau_material_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

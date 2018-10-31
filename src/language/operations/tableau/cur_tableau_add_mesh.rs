use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauAddMeshOp;

const DOC : &str = "Adds a static mesh to the tableau with specified offset, scale and alpha. First value fixed point is the scale factor, second value fixed point is alpha. use 0 for default values.";

pub const OP_CODE: u32 = 1992;

pub const IDENT: &str = "cur_tableau_add_mesh";

impl Operation for CurTableauAddMeshOp {
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
                make_param_doc("<mesh_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("<value_fixed_point>", ""),
                make_param_doc("<value_fixed_point>", ""),
            ],
        }
    }
}

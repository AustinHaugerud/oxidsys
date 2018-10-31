use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauAddTableauMeshOp;

const DOC : &str = "Used in module_tableau_materials.py to add one tableau to another. Value parameter is passed to tableau_material as is.";

pub const OP_CODE: u32 = 1980;

pub const IDENT: &str = "cur_tableau_add_tableau_mesh";

impl Operation for CurTableauAddTableauMeshOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<tableau_material_id>", ""),
                make_param_doc("<value>", ""),
                make_param_doc("<position_register_no>", ""),
            ],
        }
    }
}

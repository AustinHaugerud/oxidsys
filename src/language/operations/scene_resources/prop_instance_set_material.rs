use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PropInstanceSetMaterialOp;

const DOC: &str = "Version 1.161+. 4research. give sub mesh as -1 to change all meshes' materials.";

pub const OP_CODE: u32 = 2617;

pub const IDENT: &str = "prop_instance_set_material";

impl Operation for PropInstanceSetMaterialOp {
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
                make_param_doc("<prop_instance_no>", ""),
                make_param_doc("<sub_mesh_no>", ""),
                make_param_doc("<string_register>", ""),
            ],
        }
    }
}

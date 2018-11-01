use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurItemAddMeshOp;

const DOC : &str = "Version 1.161+. Only call inside ti_on_init_item trigger. Adds another mesh to item, allowing the creation of combined items. Parameter <mesh_name_string> should contain mesh name itself, NOT a mesh reference. LOD values are optional. If <lod_end> is used, it will not be loaded.";

pub const OP_CODE: u32 = 1964;

pub const IDENT: &str = "cur_item_add_mesh";

impl Operation for CurItemAddMeshOp {
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
            num_required: 1,
            num_optional: 2,
            param_docs: vec![
                make_param_doc("<mesh_name_string>", ""),
                make_param_doc("[<lod_begin>]", ""),
                make_param_doc("[<lod_end>]", ""),
            ],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurItemSetMaterialOp;

const DOC : &str = "Version 1.161+. Only call inside ti_on_init_item trigger. Replaces material that will be used to render the item mesh. Use 0 for <sub_mesh_no> to replace material for base mesh. LOD values are optional. If <lod_end> is used, it will not be loaded.";

pub const OP_CODE: u32 = 1978;

pub const IDENT: &str = "cur_item_set_material";

impl Operation for CurItemSetMaterialOp {
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
            num_optional: 2,
            param_docs: vec![
                make_param_doc("<string_no>", ""),
                make_param_doc("<sub_mesh_no>", ""),
                make_param_doc("<lod_begin>", ""),
                make_param_doc("<lod_end>", ""),
                make_param_doc("[<lod_begin>]", ""),
                make_param_doc("[<lod_end>]", ""),
            ],
        }
    }
}

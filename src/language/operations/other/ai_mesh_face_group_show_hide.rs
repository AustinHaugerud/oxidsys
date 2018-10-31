use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AiMeshFaceGroupShowHideOp;

const DOC : &str = "auto_set_meta_mission_at_end_commited = 1305   (auto_set_meta_mission_at_end_commited), Not documented. Not used in Native. Was (simulate_battle, <value>) before.";

pub const OP_CODE: u32 = 1805;

pub const IDENT: &str = "ai_mesh_face_group_show_hide";

impl Operation for AiMeshFaceGroupShowHideOp {
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
            num_required: 2,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<group_no>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

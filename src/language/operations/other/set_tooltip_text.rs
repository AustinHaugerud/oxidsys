use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetTooltipTextOp;

const DOC : &str = "ai_mesh_face_group_show_hide = 1805    (ai_mesh_face_group_show_hide, <group_no>, <value>),  1 for enable, 0 for disable";

pub const OP_CODE: u32 = 1130;

pub const IDENT: &str = "set_tooltip_text";

impl Operation for SetTooltipTextOp {
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
            num_optional: 0,
            param_docs: vec![make_param_doc("<string_id>", "")],
        }
    }
}

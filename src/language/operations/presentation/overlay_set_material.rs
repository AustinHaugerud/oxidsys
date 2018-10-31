use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetMaterialOp;

const DOC: &str = "Version 1.161+. Replaces the material used for rendering specified overlay.";

pub const OP_CODE: u32 = 956;

pub const IDENT: &str = "overlay_set_material";

impl Operation for OverlaySetMaterialOp {
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
                make_param_doc("<overlay_id>", ""),
                make_param_doc("<string_no>", ""),
            ],
        }
    }
}

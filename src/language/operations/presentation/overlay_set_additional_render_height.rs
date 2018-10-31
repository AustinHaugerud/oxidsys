use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetAdditionalRenderHeightOp;

const DOC: &str = "Version 1.153+. Effects uncertain. 4research.";

pub const OP_CODE: u32 = 952;

pub const IDENT: &str = "overlay_set_additional_render_height";

impl Operation for OverlaySetAdditionalRenderHeightOp {
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
                make_param_doc("<height_adder>", ""),
            ],
        }
    }
}

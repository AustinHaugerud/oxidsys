use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetTooltipOp;

const DOC : &str = "Defines a text which will be displayed as a tooltip when mouse pointer will hover over the specified overlay. Unreliable, always test how it works.";

pub const OP_CODE: u32 = 950;

pub const IDENT: &str = "overlay_set_tooltip";

impl Operation for OverlaySetTooltipOp {
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
                make_param_doc("<string_id>", ""),
            ],
        }
    }
}

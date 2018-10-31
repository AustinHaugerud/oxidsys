use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct OverlaySetAlphaOp;

const DOC : &str = "Changes the overlay alpha (hexadecimal value in 0x00..0xFF range). May not work with some overlay types.";

pub const OP_CODE: u32 = 922;

pub const IDENT: &str = "overlay_set_alpha";

impl Operation for OverlaySetAlphaOp {
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
                make_param_doc("<alpha>", ""),
            ],
        }
    }
}

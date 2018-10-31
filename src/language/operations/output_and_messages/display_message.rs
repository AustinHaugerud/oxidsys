use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct DisplayMessageOp;

const DOC: &str = "Display a string message using provided color (hex-coded 0xRRGGBB).";

pub const OP_CODE: u32 = 1106;

pub const IDENT: &str = "display_message";

impl Operation for DisplayMessageOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<string_id>", ""),
                make_param_doc("[hex_colour_code]", ""),
            ],
        }
    }
}

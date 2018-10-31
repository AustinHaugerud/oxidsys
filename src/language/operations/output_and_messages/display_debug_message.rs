use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct DisplayDebugMessageOp;

const DOC : &str = "Displays a string message, but only in debug mode, using provided color (hex-coded 0xRRGGBB). The message is additionally written to rgl_log.txt file in both release and debug modes when edit mode is enabled.";

pub const OP_CODE: u32 = 1104;

pub const IDENT: &str = "display_debug_message";

impl Operation for DisplayDebugMessageOp {
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

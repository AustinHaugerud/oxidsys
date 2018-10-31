use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct DisplayLogMessageOp;

const DOC : &str = "Display a string message using provided color (hex-coded 0xRRGGBB). The message will also be written to game log (accessible through Notes / Game Log), and will persist between sessions (i.e. it will be stored as part of the savegame).";

pub const OP_CODE: u32 = 1105;

pub const IDENT: &str = "display_log_message";

impl Operation for DisplayLogMessageOp {
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

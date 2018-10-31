use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetCurrentColorOp;

const DOC: &str =
    "Sets color for subsequent calls to (add_point_light) etc. Color component ranges are 0..255.";

pub const OP_CODE: u32 = 1950;

pub const IDENT: &str = "set_current_color";

impl Operation for SetCurrentColorOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<red_value>", ""),
                make_param_doc("<green_value>", ""),
                make_param_doc("<blue_value>", ""),
            ],
        }
    }
}

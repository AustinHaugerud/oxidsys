use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TutorialMessageSetPositionOp;

const DOC: &str = "Defines screen position for the tutorial box. Assumes screen size is 1000*750.";

pub const OP_CODE: u32 = 1123;

pub const IDENT: &str = "tutorial_message_set_position";

impl Operation for TutorialMessageSetPositionOp {
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
                make_param_doc("<position_x>", ""),
                make_param_doc("<position_y>", ""),
            ],
        }
    }
}

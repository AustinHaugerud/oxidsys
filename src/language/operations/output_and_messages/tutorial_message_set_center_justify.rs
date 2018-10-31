use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TutorialMessageSetCenterJustifyOp;

const DOC : &str = "Sets tutorial box to be center justified (value = 1), or use positioning dictated by tutorial_message_set_position (value = 0).";

pub const OP_CODE: u32 = 1125;

pub const IDENT: &str = "tutorial_message_set_center_justify";

impl Operation for TutorialMessageSetCenterJustifyOp {
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
            param_docs: vec![make_param_doc("<val>", "")],
        }
    }
}

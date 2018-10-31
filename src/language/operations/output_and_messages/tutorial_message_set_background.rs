use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TutorialMessageSetBackgroundOp;

const DOC: &str =
    "Defines whether the tutorial box will have a background or not (1 or 0). Default is off.";

pub const OP_CODE: u32 = 1126;

pub const IDENT: &str = "tutorial_message_set_background";

impl Operation for TutorialMessageSetBackgroundOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TutorialMessageOp;

const DOC : &str = "Displays a popup window with tutorial text stored in referenced string or string register. Use -1 to close any currently open tutorial box. Optional parameters allow you to define text color and time period after which the tutorial box will close automatically.";

pub const OP_CODE: u32 = 1122;

pub const IDENT: &str = "tutorial_message";

impl Operation for TutorialMessageOp {
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
            num_optional: 2,
            param_docs: vec![
                make_param_doc("<string_id>", ""),
                make_param_doc("[color]", ""),
                make_param_doc("[auto_close_time]","")
            ],
        }
    }
}

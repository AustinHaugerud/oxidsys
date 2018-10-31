use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct QuestionBoxOp;

const DOC : &str = "Displays a popup window with the text of the question and two buttons (Yes and No by default, but can be overridden). When the player selects one of possible responses, a ti_on_question_answered trigger will be executed.";

pub const OP_CODE: u32 = 1121;

pub const IDENT: &str = "question_box";

impl Operation for QuestionBoxOp {
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
            num_optional: 2,
            param_docs: vec![
                make_param_doc("<string_id>", ""),
                make_param_doc("<yes_string_id>", ""),
                make_param_doc("<no_string_id>", ""),
                make_param_doc("[<yes_string_id>]", ""),
                make_param_doc("[<no_string_id>]", "")
            ],
        }
    }
}

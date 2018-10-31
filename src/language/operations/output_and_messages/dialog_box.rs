use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct DialogBoxOp;

const DOC: &str = "Displays a popup window with the text message and an optional caption.";

pub const OP_CODE: u32 = 1120;

pub const IDENT: &str = "dialog_box";

impl Operation for DialogBoxOp {
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
                make_param_doc("<text_string_id>", ""),
                make_param_doc("[title_string_id]", ""),
            ],
        }
    }
}

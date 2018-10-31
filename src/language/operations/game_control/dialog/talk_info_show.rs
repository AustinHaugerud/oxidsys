use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TalkInfoShowOp;

const DOC : &str = "Used in the dialogs code to display relations bar on opponent's portrait when mouse is hovering over it (value = 1) or disable this functionality (value = 0)";

pub const OP_CODE: u32 = 2020;

pub const IDENT: &str = "talk_info_show";

impl Operation for TalkInfoShowOp {
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
            param_docs: vec![make_param_doc("<hide_or_show>", "")],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TutorialBoxOp;

const DOC: &str = "This operation is deprecated but is still used in Native.";

pub const OP_CODE: u32 = 1120;

pub const IDENT: &str = "tutorial_box";

impl Operation for TutorialBoxOp {
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
                make_param_doc("<string_id>", ""),
                make_param_doc("<string_id>", ""),
            ],
        }
    }
}

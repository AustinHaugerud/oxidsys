use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TalkInfoSetLineOp;

const DOC : &str = "Sets the additional text information (usually troop name) to be displayed together with the relations bar.";

pub const OP_CODE: u32 = 2022;

pub const IDENT: &str = "talk_info_set_line";

impl Operation for TalkInfoSetLineOp {
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
                make_param_doc("<line_no>", ""),
                make_param_doc("<string_no>", ""),
            ],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreQuestNumberOp;

const DOC: &str = "Apparently deprecated.";

pub const OP_CODE: u32 = 2261;

pub const IDENT: &str = "store_quest_number";

impl Operation for StoreQuestNumberOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<quest_id>", ""),
            ],
        }
    }
}

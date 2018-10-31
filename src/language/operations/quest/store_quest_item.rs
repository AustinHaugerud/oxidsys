use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreQuestItemOp;

const DOC: &str =
    "Apparently deprecated. Native now uses quest slots to keep track of this information.";

pub const OP_CODE: u32 = 2262;

pub const IDENT: &str = "store_quest_item";

impl Operation for StoreQuestItemOp {
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
                make_param_doc("<item_id>", ""),
            ],
        }
    }
}

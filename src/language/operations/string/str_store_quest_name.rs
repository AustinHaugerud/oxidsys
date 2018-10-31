use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreQuestNameOp;

const DOC: &str =
    "Stores quest name (as defined in module_quests.py) in referenced string register.";

pub const OP_CODE: u32 = 2336;

pub const IDENT: &str = "str_store_quest_name";

impl Operation for StrStoreQuestNameOp {
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
                make_param_doc("<string_register>", ""),
                make_param_doc("<quest_id>", ""),
            ],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreInfoPageNameLinkOp;

const DOC : &str = "Stores info page title as an internal game link. Resulting string can be used in game notes, will be highlighted, and clicking on it will redirect the player to the details page of the referenced info page.";

pub const OP_CODE: u32 = 2345;

pub const IDENT: &str = "str_store_info_page_name_link";

impl Operation for StrStoreInfoPageNameLinkOp {
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
                make_param_doc("<info_page_id>", ""),
            ],
        }
    }
}

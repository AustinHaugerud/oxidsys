use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreInfoPageNameOp;

const DOC: &str =
    "Stores info page title (as defined in module_info_pages.py) in referenced string register.";

pub const OP_CODE: u32 = 2337;

pub const IDENT: &str = "str_store_info_page_name";

impl Operation for StrStoreInfoPageNameOp {
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

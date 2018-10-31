use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreItemNamePluralOp;

const DOC: &str = "Stores plural item name in referenced string register.";

pub const OP_CODE: u32 = 2326;

pub const IDENT: &str = "str_store_item_name_plural";

impl Operation for StrStoreItemNamePluralOp {
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
                make_param_doc("<item_id>", ""),
            ],
        }
    }
}

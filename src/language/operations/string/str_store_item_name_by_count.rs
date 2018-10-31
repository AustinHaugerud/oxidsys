use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StrStoreItemNameByCountOp;

const DOC : &str = "Stores singular or plural item name with number of items (\"11 Swords\", \"1 Bottle of Wine\").";

pub const OP_CODE: u32 = 2327;

pub const IDENT: &str = "str_store_item_name_by_count";

impl Operation for StrStoreItemNameByCountOp {
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

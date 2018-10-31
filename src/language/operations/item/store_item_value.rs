use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreItemValueOp;

const DOC : &str = "Stores item nominal price as listed in module_items.py. Does not take item modifier or quantity (for food items) into account.";

pub const OP_CODE: u32 = 2230;

pub const IDENT: &str = "store_item_value";

impl Operation for StoreItemValueOp {
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

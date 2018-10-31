use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemGetTypeOp;

const DOC: &str = "Returns item class (see header_items.py for itp_type_* constants).";

pub const OP_CODE: u32 = 1570;

pub const IDENT: &str = "item_get_type";

impl Operation for ItemGetTypeOp {
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

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemHasCapabilityOp;

const DOC : &str = "Version 1.161+. Checks that the item has specified capability flag set. See the list of itcf_* flags in header_items.py";

pub const OP_CODE: u32 = 2724;

pub const IDENT: &str = "item_has_capability";

impl Operation for ItemHasCapabilityOp {
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
                make_param_doc("<item_kind_no>", ""),
                make_param_doc("<capability>", ""),
            ],
        }
    }
}

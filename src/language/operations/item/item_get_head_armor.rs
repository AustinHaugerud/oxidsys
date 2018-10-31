use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemGetHeadArmorOp;

const DOC: &str = "Version 1.161+. Retrieves item head armor value.";

pub const OP_CODE: u32 = 2703;

pub const IDENT: &str = "item_get_head_armor";

impl Operation for ItemGetHeadArmorOp {
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
                make_param_doc("<item_kind_no>", ""),
            ],
        }
    }
}

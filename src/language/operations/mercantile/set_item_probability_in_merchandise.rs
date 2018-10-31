use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetItemProbabilityInMerchandiseOp;

const DOC: &str = "Sets item probability of being generated as merchandise to the provided value.";

pub const OP_CODE: u32 = 1493;

pub const IDENT: &str = "set_item_probability_in_merchandise";

impl Operation for SetItemProbabilityInMerchandiseOp {
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
                make_param_doc("<item_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

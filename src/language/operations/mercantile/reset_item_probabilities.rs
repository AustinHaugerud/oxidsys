use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ResetItemProbabilitiesOp;

const DOC : &str = "Sets all items probability of being generated as merchandise to the provided value. Use zero with subsequent calls to (set_item_probability_in_merchandise) to only allow generation of certain items.";

pub const OP_CODE: u32 = 1492;

pub const IDENT: &str = "reset_item_probabilities";

impl Operation for ResetItemProbabilitiesOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}

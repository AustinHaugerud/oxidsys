use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ResetPriceRatesOp;

const DOC: &str = "Resets customized price rates for merchants.";

pub const OP_CODE: u32 = 1170;

pub const IDENT: &str = "reset_price_rates";

impl Operation for ResetPriceRatesOp {
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
            num_required: 0,
            num_optional: 0,
            param_docs: vec![],
        }
    }
}

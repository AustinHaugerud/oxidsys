use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetMerchandiseMaxValueOp;

const DOC : &str = "Not used in Native. Apparently prevents items with price higher than listed from being generated as merchandise.";

pub const OP_CODE: u32 = 1491;

pub const IDENT: &str = "set_merchandise_max_value";

impl Operation for SetMerchandiseMaxValueOp {
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

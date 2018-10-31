use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreAcosOp;

const DOC: &str = "Assigns dest := ARCCOS (value)";

pub const OP_CODE: u32 = 2141;

pub const IDENT: &str = "store_acos";

impl Operation for StoreAcosOp {
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
                make_param_doc("<destination_fixed_point>", ""),
                make_param_doc("<value_fixed_point>", ""),
            ],
        }
    }
}

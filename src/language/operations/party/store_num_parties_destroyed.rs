use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreNumPartiesDestroyedOp;

const DOC: &str = "Stores the total number of destroyed parties of specified type.";

pub const OP_CODE: u32 = 2301;

pub const IDENT: &str = "store_num_parties_destroyed";

impl Operation for StoreNumPartiesDestroyedOp {
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
                make_param_doc("<party_template_id>", ""),
            ],
        }
    }
}

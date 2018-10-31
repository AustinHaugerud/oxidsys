use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreNumPartiesCreatedOp;

const DOC: &str =
    "Stores the total number of created parties of specified type. Not used in Native.";

pub const OP_CODE: u32 = 2300;

pub const IDENT: &str = "store_num_parties_created";

impl Operation for StoreNumPartiesCreatedOp {
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

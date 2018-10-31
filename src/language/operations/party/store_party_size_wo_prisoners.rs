use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StorePartySizeWoPrisonersOp;

const DOC : &str = "Stores total number of members in the party (without prisoners), duplicating (party_get_num_companions).";

pub const OP_CODE: u32 = 2157;

pub const IDENT: &str = "store_party_size_wo_prisoners";

impl Operation for StorePartySizeWoPrisonersOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("[party_id]", ""),
            ],
        }
    }
}

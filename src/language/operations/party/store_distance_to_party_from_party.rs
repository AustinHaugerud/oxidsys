use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreDistanceToPartyFromPartyOp;

const DOC: &str = "Retrieves distance between two parties on the global map.";

pub const OP_CODE: u32 = 2281;

pub const IDENT: &str = "store_distance_to_party_from_party";

impl Operation for StoreDistanceToPartyFromPartyOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<destination>", ""),
                make_param_doc("<party_id>", ""),
                make_param_doc("<party_id>", ""),
            ],
        }
    }
}

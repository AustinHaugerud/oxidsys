use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreFactionOfTroopOp;

const DOC: &str = "Alternative spelling of the above operation.";

pub const OP_CODE: u32 = 2173;

pub const IDENT: &str = "store_faction_of_troop";

impl Operation for StoreFactionOfTroopOp {
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
                make_param_doc("<troop_id>", ""),
            ],
        }
    }
}

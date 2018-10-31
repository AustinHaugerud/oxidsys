use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreTroopValueOp;

const DOC : &str = "Stores some value which is apparently related to troop's overall fighting value. Swadian infantry line troops from Native produced values 24, 47, 80, 133, 188. Calling on player produced 0.";

pub const OP_CODE: u32 = 2231;

pub const IDENT: &str = "store_troop_value";

impl Operation for StoreTroopValueOp {
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

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreRelationOp;

const DOC: &str = "Retrieves relation between two factions. Relation is in -100..100 range.";

pub const OP_CODE: u32 = 2190;

pub const IDENT: &str = "store_relation";

impl Operation for StoreRelationOp {
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
                make_param_doc("<faction_id_1>", ""),
                make_param_doc("<faction_id_2>", ""),
            ],
        }
    }
}

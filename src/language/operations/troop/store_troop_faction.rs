use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreTroopFactionOp;

const DOC: &str = "Retrieves current troop faction allegiance.";

pub const OP_CODE: u32 = 2173;

pub const IDENT: &str = "store_troop_faction";

impl Operation for StoreTroopFactionOp {
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

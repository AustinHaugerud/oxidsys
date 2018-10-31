use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopGetUpgradeTroopOp;

const DOC : &str = "Retrieves possible directions for non-hero troop upgrade. Use 0 to retrieve first upgrade path, and 1 to return second. Result of -1 means there's no such upgrade path for this troop.";

pub const OP_CODE: u32 = 1561;

pub const IDENT: &str = "troop_get_upgrade_troop";

impl Operation for TroopGetUpgradeTroopOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<upgrade_path>", ""),
            ],
        }
    }
}

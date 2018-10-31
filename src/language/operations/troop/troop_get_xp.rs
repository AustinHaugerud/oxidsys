use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopGetXpOp;

const DOC: &str = "Retrieves total amount of xp specified troop has.";

pub const OP_CODE: u32 = 1515;

pub const IDENT: &str = "troop_get_xp";

impl Operation for TroopGetXpOp {
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

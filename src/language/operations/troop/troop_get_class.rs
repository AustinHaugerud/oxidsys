use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopGetClassOp;

const DOC: &str = "Retrieves troop class. Returns values in range 0..8.";

pub const OP_CODE: u32 = 1516;

pub const IDENT: &str = "troop_get_class";

impl Operation for TroopGetClassOp {
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

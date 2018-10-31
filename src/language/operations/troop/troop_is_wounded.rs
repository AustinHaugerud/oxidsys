use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopIsWoundedOp;

const DOC: &str = "Checks that the troop is wounded. Only works for hero troops.";

pub const OP_CODE: u32 = 1508;

pub const IDENT: &str = "troop_is_wounded";

impl Operation for TroopIsWoundedOp {
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
            num_optional: 0,
            param_docs: vec![make_param_doc("<troop_id>", "")],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopIsMountedOp;

const DOC : &str = "Checks the troop for tf_mounted flag (see header_troops.py). Does NOT check that the troop has a horse.";

pub const OP_CODE: u32 = 152;

pub const IDENT: &str = "troop_is_mounted";

impl Operation for TroopIsMountedOp {
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

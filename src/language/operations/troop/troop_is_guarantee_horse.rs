use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopIsGuaranteeHorseOp;

const DOC : &str = "Checks the troop for tf_guarantee_horse flag (see header_troops.py). Does not check that troop actually has some horse.";

pub const OP_CODE: u32 = 154;

pub const IDENT: &str = "troop_is_guarantee_horse";

impl Operation for TroopIsGuaranteeHorseOp {
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

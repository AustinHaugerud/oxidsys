use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetAgeOp;

const DOC : &str = "Defines a new age for the troop (will be used by the game engine to generate appropriately aged face). Age is in range 0.100.";

pub const OP_CODE: u32 = 1555;

pub const IDENT: &str = "troop_set_age";

impl Operation for TroopSetAgeOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<age_slider_pos>", ""),
            ],
        }
    }
}

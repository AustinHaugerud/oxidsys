use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct RestForHoursOp;

const DOC : &str = "Forces the player party to rest for specified number of hours. Time can be accelerated and player can be made immune or subject to attacks.";

pub const OP_CODE: u32 = 1030;

pub const IDENT: &str = "rest_for_hours";

impl Operation for RestForHoursOp {
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
            num_optional: 2,
            param_docs: vec![
                make_param_doc("<rest_time_in_hours>", ""),
                make_param_doc("[time_speed_multiplier]", ""),
                make_param_doc("[remain_attackable]","")
            ],
        }
    }
}

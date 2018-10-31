use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct RestForHoursInteractiveOp;

const DOC : &str = "Forces the player party to rest for specified number of hours. Player can break the rest at any moment. Time can be accelerated and player can be made immune or subject to attacks.";

pub const OP_CODE: u32 = 1031;

pub const IDENT: &str = "rest_for_hours_interactive";

impl Operation for RestForHoursInteractiveOp {
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
                make_param_doc("[remain_attackable]", "")
            ],
        }
    }
}

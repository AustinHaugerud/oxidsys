use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PositionTransformPositionToLocalOp;

const DOC : &str = "The opposite to (position_transform_position_to_parent), this operation allows you to get source's *relative* position to your anchor. Suppose you want to run some decision making for your bot agent depending on player's position. In order to know where player is located relative to your bot you call (position_transform_position_to_local, <position_dest>, <bot_position>, <player_position>). Then we check position_dest's Y coordinate - if it's negative, then the player is behind our bot's back.";

pub const OP_CODE: u32 = 717;

pub const IDENT: &str = "position_transform_position_to_local";

impl Operation for PositionTransformPositionToLocalOp {
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
                make_param_doc("<position_dest>", ""),
                make_param_doc("<position_anchor>", ""),
                make_param_doc("<position_source>", ""),
            ],
        }
    }
}

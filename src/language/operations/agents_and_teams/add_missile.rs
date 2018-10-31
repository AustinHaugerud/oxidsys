use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct AddMissileOp;

const DOC : &str = "Version 1.153+. Creates a missile with specified parameters. Note that <starting_position> parameter also determines the direction in which missile flies.";

pub const OP_CODE: u32 = 1829;

pub const IDENT: &str = "add_missile";

impl Operation for AddMissileOp {
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
            num_required: 7,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<agent_id>", ""),
                make_param_doc("<starting_position>", ""),
                make_param_doc("<starting_speed_fixed_point>", ""),
                make_param_doc("<weapon_item_id>", ""),
                make_param_doc("<weapon_item_modifier>", ""),
                make_param_doc("<missile_item_id>", ""),
                make_param_doc("<missile_item_modifier>", ""),
            ],
        }
    }
}

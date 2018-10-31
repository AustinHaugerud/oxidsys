use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerAddSpawnItemOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 410;

pub const IDENT: &str = "player_add_spawn_item";

impl Operation for PlayerAddSpawnItemOp {
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
                make_param_doc("<player_id>", ""),
                make_param_doc("<item_slot_no>", ""),
                make_param_doc("<item_id>", ""),
            ],
        }
    }
}

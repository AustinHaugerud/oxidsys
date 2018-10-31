use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerSavePickedUpItemsForNextSpawnOp;

const DOC: &str = "";

pub const OP_CODE: u32 = 459;

pub const IDENT: &str = "player_save_picked_up_items_for_next_spawn";

impl Operation for PlayerSavePickedUpItemsForNextSpawnOp {
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
            param_docs: vec![make_param_doc("<player_id>", "")],
        }
    }
}

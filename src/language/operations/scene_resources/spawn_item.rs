use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SpawnItemOp;

const DOC : &str = "Spawns a new item, possibly with modifier, on the scene in the position specified by previous call to (set_spawn_position). Optional parameter determines time period (in second) after which the item will disappear. Using 0 will prevent the item from disappearing.";

pub const OP_CODE: u32 = 1971;

pub const IDENT: &str = "spawn_item";

impl Operation for SpawnItemOp {
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
            num_optional: 1,
            param_docs: vec![
                make_param_doc("<item_kind_id>", ""),
                make_param_doc("<item_modifier>", ""),
                make_param_doc("[seconds_before_pruning]", ""),
            ],
        }
    }
}

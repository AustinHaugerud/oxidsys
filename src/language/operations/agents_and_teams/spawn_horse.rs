use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SpawnHorseOp;

const DOC : &str = "Spawns a new horse (with any modifier) in the specified position and saves the reference to the new agent in reg0.";

pub const OP_CODE: u32 = 1973;

pub const IDENT: &str = "spawn_horse";

impl Operation for SpawnHorseOp {
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
                make_param_doc("<item_kind_id>", ""),
                make_param_doc("<item_modifier>", ""),
            ],
        }
    }
}

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SpawnItemWithoutRefillOp;

const DOC: &str =
    "Version 1.153+. UNTESTED. It is unclear how this is different from standard (spawn_item).";

pub const OP_CODE: u32 = 1976;

pub const IDENT: &str = "spawn_item_without_refill";

impl Operation for SpawnItemWithoutRefillOp {
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

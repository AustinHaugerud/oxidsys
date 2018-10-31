use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreItemKindCountOp;

const DOC : &str = "Calculates total number of items of specified type that the troop has. Default troop is player.";

pub const OP_CODE: u32 = 2165;

pub const IDENT: &str = "store_item_kind_count";

impl Operation for StoreItemKindCountOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<item_id>", ""),
                make_param_doc("[troop_id]", ""),
            ],
        }
    }
}

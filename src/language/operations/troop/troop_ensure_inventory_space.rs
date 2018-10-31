use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopEnsureInventorySpaceOp;

const DOC : &str = "Removes items from troop inventory until troop has specified number of free inventory slots. Will free inventory slots starting from the end (items at the bottom of inventory will be removed first if there's not enough free space).";

pub const OP_CODE: u32 = 1510;

pub const IDENT: &str = "troop_ensure_inventory_space";

impl Operation for TroopEnsureInventorySpaceOp {
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
                make_param_doc("<troop_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

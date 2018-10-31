use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct StoreNumPartiesDestroyedByPlayerOp;

const DOC: &str =
    "Stores the total number of parties of specified type which have been destroyed by player.";

pub const OP_CODE: u32 = 2302;

pub const IDENT: &str = "store_num_parties_destroyed_by_player";

impl Operation for StoreNumPartiesDestroyedByPlayerOp {
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
                make_param_doc("<destination>", ""),
                make_param_doc("<party_template_id>", ""),
            ],
        }
    }
}

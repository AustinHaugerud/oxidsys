use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetNameOp;

const DOC: &str = "Renames the troop, setting a new singular name for it.";

pub const OP_CODE: u32 = 1501;

pub const IDENT: &str = "troop_set_name";

impl Operation for TroopSetNameOp {
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
                make_param_doc("<string_no>", ""),
            ],
        }
    }
}

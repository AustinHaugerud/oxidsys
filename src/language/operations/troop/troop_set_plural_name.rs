use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopSetPluralNameOp;

const DOC: &str = "Renames the troop, setting a new plural name for it.";

pub const OP_CODE: u32 = 1502;

pub const IDENT: &str = "troop_set_plural_name";

impl Operation for TroopSetPluralNameOp {
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

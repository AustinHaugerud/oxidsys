use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopHasItemEquippedOp;

const DOC: &str = "Checks that the troop has this item equipped (worn or wielded).";

pub const OP_CODE: u32 = 151;

pub const IDENT: &str = "troop_has_item_equipped";

impl Operation for TroopHasItemEquippedOp {
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
                make_param_doc("<item_id>", ""),
            ],
        }
    }
}

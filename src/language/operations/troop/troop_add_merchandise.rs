use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct TroopAddMerchandiseOp;

const DOC : &str = "Adds a specified number of random items of certain type (see itp_type_* constants in header_items.py) to troop inventory. Only adds items with itp_merchandise flags.";

pub const OP_CODE: u32 = 1512;

pub const IDENT: &str = "troop_add_merchandise";

impl Operation for TroopAddMerchandiseOp {
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
            num_required: 3,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<troop_id>", ""),
                make_param_doc("<item_type_id>", ""),
                make_param_doc("<value>", ""),
            ],
        }
    }
}

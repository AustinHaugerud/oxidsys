use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemGetWeaponLengthOp;

const DOC : &str = "Version 1.161+. Retrieves item length (for weapons) or shield half-width (for shields). To get actual shield width, multiply this value by 2. Essentially, it is a distance from shield's \"center\" point to it's left, right and top edges (and bottom edge as well if shield height is not defined).";

pub const OP_CODE: u32 = 2707;

pub const IDENT: &str = "item_get_weapon_length";

impl Operation for ItemGetWeaponLengthOp {
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
                make_param_doc("<item_kind_no>", ""),
            ],
        }
    }
}

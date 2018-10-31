use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemGetShieldHeightOp;

const DOC : &str = "Version 1.161+. Retrieves distance from shield \"center\" to it's bottom edge as a fixed point number. Use (set_fixed_point_multiplier, 100), to retrieve the correct value with this operation. To get actual shield height, use shield_height + weapon_length if this operation returns a non-zero value, otherwise use 2 * weapon_length.";

pub const OP_CODE: u32 = 2712;

pub const IDENT: &str = "item_get_shield_height";

impl Operation for ItemGetShieldHeightOp {
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
                make_param_doc("<destination_fixed_point>", ""),
                make_param_doc("<item_kind_no>", ""),
            ],
        }
    }
}

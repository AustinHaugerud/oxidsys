use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ItemGetFoodQualityOp;

const DOC : &str = "Version 1.161+. Retrieves food quality coefficient (as of Warband 1.165, this coefficient is actually set for many food items, but never used in the code as there was no way to retrieve this coeff before 1.161 patch).";

pub const OP_CODE: u32 = 2716;

pub const IDENT: &str = "item_get_food_quality";

impl Operation for ItemGetFoodQualityOp {
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

use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct PlayerGetValueOfOriginalItemsOp;

const DOC : &str = "Undocumented. Official docs: this operation returns values of the items, but default troop items will be counted as zero (except horse)";

pub const OP_CODE: u32 = 460;

pub const IDENT: &str = "player_get_value_of_original_items";

impl Operation for PlayerGetValueOfOriginalItemsOp {
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
            num_required: 1,
            num_optional: 0,
            param_docs: vec![make_param_doc("<player_id>", "")],
        }
    }
}

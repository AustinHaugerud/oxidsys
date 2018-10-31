use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauAddOverrideItemOp;

const DOC : &str = "When creating a troop image for current tableau, the operation will add a new item to troop's equipment.";

pub const OP_CODE: u32 = 1999;

pub const IDENT: &str = "cur_tableau_add_override_item";

impl Operation for CurTableauAddOverrideItemOp {
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
            param_docs: vec![make_param_doc("<item_kind_id>", "")],
        }
    }
}

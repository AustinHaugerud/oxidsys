use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurItemSetTableauMaterialOp;

const DOC : &str = "Can only be used inside ti_on_init_item trigger in module_items.py. Assigns tableau to the item instance. Value of <instance_code> will be passed to tableau code. Commonly used for heraldic armors and shields.";

pub const OP_CODE: u32 = 1981;

pub const IDENT: &str = "cur_item_set_tableau_material";

impl Operation for CurItemSetTableauMaterialOp {
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
                make_param_doc("<tableau_material_id>", ""),
                make_param_doc("<instance_code>", ""),
            ],
        }
    }
}

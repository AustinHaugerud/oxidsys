use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauAddHorseOp;

const DOC: &str = "Adds a rendered image of a horse in a specified animation to current tableau.";

pub const OP_CODE: u32 = 1996;

pub const IDENT: &str = "cur_tableau_add_horse";

impl Operation for CurTableauAddHorseOp {
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
                make_param_doc("<item_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("<animation_id>", ""),
            ],
        }
    }
}

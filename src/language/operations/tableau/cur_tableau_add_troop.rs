use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauAddTroopOp;

const DOC : &str = "Adds a rendered image of the troop in a specified animation to current tableau. If instance_no is 0 or less, then the face is not generated randomly (important for heroes).";

pub const OP_CODE: u32 = 1995;

pub const IDENT: &str = "cur_tableau_add_troop";

impl Operation for CurTableauAddTroopOp {
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
            num_required: 4,
            num_optional: 0,
            param_docs: vec![
                make_param_doc("<troop_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("<animation_id>", ""),
                make_param_doc("<instance_no>", ""),
            ],
        }
    }
}

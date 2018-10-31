use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauAddMapIconOp;

const DOC : &str = "Adds a rendered image of a map icon to current tableau. Last parameter is the scale factor for the model.";

pub const OP_CODE: u32 = 1994;

pub const IDENT: &str = "cur_tableau_add_map_icon";

impl Operation for CurTableauAddMapIconOp {
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
                make_param_doc("<map_icon_id>", ""),
                make_param_doc("<position>", ""),
                make_param_doc("<value_fixed_point>", ""),
            ],
        }
    }
}

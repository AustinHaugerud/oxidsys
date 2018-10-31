use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct CurTableauSetBackgroundColorOp;

const DOC: &str = "Defines solid background color for the current tableau.";

pub const OP_CODE: u32 = 1985;

pub const IDENT: &str = "cur_tableau_set_background_color";

impl Operation for CurTableauSetBackgroundColorOp {
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
            param_docs: vec![make_param_doc("<value>", "")],
        }
    }
}

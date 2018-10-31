use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FactionGetColorOp;

const DOC: &str = "Gets the faction color value.";

pub const OP_CODE: u32 = 1277;

pub const IDENT: &str = "faction_get_color";

impl Operation for FactionGetColorOp {
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
                make_param_doc("<faction_id>", ""),
            ],
        }
    }
}

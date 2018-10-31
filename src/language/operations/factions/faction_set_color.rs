use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct FactionSetColorOp;

const DOC : &str = "Sets the faction color. All parties and centers belonging to this faction will be displayed with this color on global map.";

pub const OP_CODE: u32 = 1276;

pub const IDENT: &str = "faction_set_color";

impl Operation for FactionSetColorOp {
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
                make_param_doc("<faction_id>", ""),
                make_param_doc("<color_code>", ""),
            ],
        }
    }
}

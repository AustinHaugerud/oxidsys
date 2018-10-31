use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct EntryPointSetPositionOp;

const DOC: &str = "Moves the entry point to the specified position on the scene.";

pub const OP_CODE: u32 = 1781;

pub const IDENT: &str = "entry_point_set_position";

impl Operation for EntryPointSetPositionOp {
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
                make_param_doc("<entry_no>", ""),
                make_param_doc("<position>", ""),
            ],
        }
    }
}

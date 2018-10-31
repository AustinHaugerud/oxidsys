use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct EntryPointGetPositionOp;

const DOC: &str = "Retrieves the position of the entry point on the scene.";

pub const OP_CODE: u32 = 1780;

pub const IDENT: &str = "entry_point_get_position";

impl Operation for EntryPointGetPositionOp {
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
                make_param_doc("<position>", ""),
                make_param_doc("<entry_no>", ""),
            ],
        }
    }
}

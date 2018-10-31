use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetBackgroundMeshOp;

const DOC : &str = "Sets the specified mesh as the background for the current menu. Possibly can be used for dialogs or presentations, but was not tested.";

pub const OP_CODE: u32 = 2031;

pub const IDENT: &str = "set_background_mesh";

impl Operation for SetBackgroundMeshOp {
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
            param_docs: vec![make_param_doc("<mesh_id>", "")],
        }
    }
}

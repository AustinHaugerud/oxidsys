use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetSkyboxOp;

const DOC : &str = "Version 1.153+. Forces the scene to be rendered with specified skybox. Index of -1 will disable.";

pub const OP_CODE: u32 = 2389;

pub const IDENT: &str = "set_skybox";

impl Operation for SetSkyboxOp {
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
                make_param_doc("<non_hdr_skybox_index>", ""),
                make_param_doc("<hdr_skybox_index>", ""),
            ],
        }
    }
}

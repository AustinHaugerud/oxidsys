use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct SetShaderParamFloatOp;

const DOC : &str = "Version 1.153+. Allows direct manupulation of shader parameters. Operation scope is unknown, possibly global. Parameter is a float value.";

pub const OP_CODE: u32 = 2401;

pub const IDENT: &str = "set_shader_param_float";

impl Operation for SetShaderParamFloatOp {
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
                make_param_doc("<parameter_name>", ""),
                make_param_doc("<value_fixed_point>", ""),
            ],
        }
    }
}

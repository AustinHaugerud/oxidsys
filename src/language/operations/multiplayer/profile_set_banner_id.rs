use language::operations::{make_param_doc, Operation, ParamInfo};

pub struct ProfileSetBannerIdOp;

const DOC : &str = "Client operation. Assigns a new banner_id to be used for multiplayer. Note that in MP banners are enumerated starting from 0 (unlike single-player where they're enumeration depends on scene prop banners' reference range).";

pub const OP_CODE: u32 = 351;

pub const IDENT: &str = "profile_set_banner_id";

impl Operation for ProfileSetBannerIdOp {
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

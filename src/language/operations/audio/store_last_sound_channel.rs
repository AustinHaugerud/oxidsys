use language::operations::Operation;

pub struct StoreLastSoundChannelOp;

const DOC: &str = r#"
Please write me!
Format: Please write me!
"#;

pub const OP_CODE: u32 = 615;

pub const IDENT: &str = "store_last_sound_channel";

impl Operation for StoreLastSoundChannelOp {
    fn op_code(&self) -> u32 {
        OP_CODE
    }

    fn documentation(&self) -> &'static str {
        DOC
    }

    fn identifier(&self) -> &'static str {
        IDENT
    }
}

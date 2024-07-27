#[derive(Clone, Debug)]
pub struct Shape {
    pub bits: u128,
    pub attachments: &'static [u8],
    pub width: u8,
    pub height: u8,
}

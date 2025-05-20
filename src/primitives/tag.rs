#![allow(dead_code)]

#[derive(Debug)]
pub struct Tag {
    pub(crate) inner: u8,
}

impl Tag {
    fn new(b: u8) -> Self {
        Self { inner: b }
    }
}

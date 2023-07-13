#![allow(dead_code, unused)]

mod formatter_builder;

pub use formatter_builder::*;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct TextBuilder<F>
where
    F: Fn(&mut Formatter<'_>) -> FmtResult,
{
    build: F,
}
impl<F> Display for TextBuilder<F>
where
    F: Fn(&mut Formatter<'_>) -> FmtResult,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        (self.build)(f)
    }
}
impl<F> TextBuilder<F>
where
    F: Fn(&mut Formatter<'_>) -> FmtResult,
{
    pub fn build_string(build: F) -> String {
        let builder = TextBuilder { build };
        format!("{}", builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_build() {
        let text: String = TextBuilder::build_string(|f| Ok(()));
        assert_eq!(text.len(), 0);
        assert_eq!(text, String::from(""));
    }
}

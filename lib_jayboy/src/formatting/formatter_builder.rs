use std::fmt::{Debug, Display, Formatter, LowerHex, Result, UpperHex, Write};

pub trait FormatterBuilder {
    fn newline(&mut self) -> &mut Self;

    fn append<A: Appendable>(&mut self, value: A) -> &mut Self;
    fn debug<D: Debug>(&mut self, value: &D) -> &mut Self;
    fn display<D: Display>(&mut self, value: &D) -> &mut Self;
    fn lower_hex<H: LowerHex>(&mut self, value: &H) -> &mut Self;
    fn upper_hex<H: UpperHex>(&mut self, value: &H) -> &mut Self;

    fn write<V, A: Appendable>(&mut self, value: &V, transform: fn(&V) -> A) -> &mut Self;

    fn enumerate<I: Iterator>(
        &mut self,
        iter: I,
        per_item: fn(&mut Self, usize, I::Item),
    ) -> &mut Self;
    fn delimit<I: Iterator>(
        &mut self,
        delimit: fn(&mut Self),
        iter: I,
        per_item: fn(&mut Self, usize, I::Item),
    ) -> &mut Self;
}

impl FormatterBuilder for Formatter<'_> {
    fn newline(&mut self) -> &mut Self {
        self.write_char('\n').unwrap();
        self
    }

    fn append<A: Appendable>(&mut self, value: A) -> &mut Self {
        value.write_to(self).unwrap();
        self
    }

    fn debug<D: Debug>(&mut self, value: &D) -> &mut Self {
        Debug::fmt(value, self).unwrap();
        self
    }

    fn display<D: Display>(&mut self, value: &D) -> &mut Self {
        Display::fmt(value, self).unwrap();
        self
    }

    fn lower_hex<H: LowerHex>(&mut self, value: &H) -> &mut Self {
        LowerHex::fmt(value, self).unwrap();
        self
    }

    fn upper_hex<H: UpperHex>(&mut self, value: &H) -> &mut Self {
        UpperHex::fmt(value, self).unwrap();
        self
    }

    fn write<V, A: Appendable>(&mut self, value: &V, transform: fn(&V) -> A) -> &mut Self {
        let appendable = (transform)(value);
        self.append(appendable)
    }

    fn enumerate<I: Iterator>(
        &mut self,
        iter: I,
        per_item: fn(&mut Self, usize, I::Item),
    ) -> &mut Self {
        for (i, item) in iter.enumerate() {
            (per_item)(self, i, item);
        }
        self
    }

    fn delimit<I: Iterator>(
        &mut self,
        delimit: fn(&mut Self),
        iter: I,
        per_item: fn(&mut Self, usize, I::Item),
    ) -> &mut Self {
        for (i, item) in iter.enumerate() {
            if i > 0 {
                (delimit)(self);
            }
            (per_item)(self, i, item);
        }
        self
    }
}

pub trait Appendable {
    fn write_to(&self, formatter: &mut Formatter<'_>) -> Result;
}
impl Appendable for char {
    fn write_to(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_char(*self)
    }
}
impl Appendable for &str {
    fn write_to(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(self)
    }
}
impl Appendable for String {
    fn write_to(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(self)
    }
}

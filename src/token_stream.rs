use std::borrow::Cow;

pub struct TokenStream {
    inner: String,
}

impl TokenStream {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }

    /// Appends an identifier to the stream
    ///
    /// note: a space will be inserted before the identifier
    pub fn append(&mut self, ident: Ident) {
        self.inner.push(' ');
        self.inner.push_str(ident.as_str())
    }

    /// Appends another stream to the stream
    ///
    /// note: a space will be inserted before the other stream
    pub fn combine(&mut self, other: &TokenStream) {
        self.inner.push(' ');
        self.inner.push_str(&other.inner)
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub(crate) fn push(&mut self, c: char) {
        self.inner.push(c)
    }

    pub(crate) fn push_str(&mut self, str: &str) {
        self.inner.push_str(str)
    }
}

pub struct Ident {
    inner: Cow<'static, str>,
}

impl Ident {
    pub fn new<T: Into<Cow<'static, str>>>(str: T) -> Self {
        Self { inner: str.into() }
    }

    pub fn as_str(&self) -> &str {
        &*self.inner
    }
}

#[derive(Copy, Clone)]
pub enum Delimiter {
    Bracket,
    Brace,
    Parenthesis,
}

impl Delimiter {
    pub fn open(self) -> char {
        match self {
            Delimiter::Bracket => '[',
            Delimiter::Brace => '{',
            Delimiter::Parenthesis => '(',
        }
    }

    pub fn close(self) -> char {
        match self {
            Delimiter::Bracket => ']',
            Delimiter::Brace => '}',
            Delimiter::Parenthesis => ')',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accept_owned_and_borrowed() {
        let i = Ident::new("hello");
        let i = Ident::new(String::from("hello"));
    }
}

use std::borrow::Cow;

#[derive(Debug, Clone)]
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

    pub fn into_string(self) -> String {
        self.inner
    }

    pub(crate) fn push(&mut self, c: char) {
        self.inner.push(c)
    }

    pub(crate) fn push_str(&mut self, str: &str) {
        self.inner.push_str(str)
    }
}

impl std::iter::FromIterator<TokenStream> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenStream>>(iter: I) -> Self {
        iter.into_iter()
            .fold(None, |accum: Option<TokenStream>, n| {
                let mut ts = accum.unwrap_or_else(TokenStream::new);
                ts.combine(&n);
                Some(ts)
            })
            .unwrap_or(TokenStream::new())
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
impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &*self.inner)
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

pub struct Literal {
    inner: String,
}

macro_rules! unsuffixed {
    ($ty:ty => $name:ident) => {
        pub fn $name(n: $ty) -> Self {
            Self {
                inner: n.to_string(),
            }
        }
    };
}

impl Literal {
    unsuffixed!(u32 => u32_unsuffixed);
    unsuffixed!(u16 => u16_unsuffixed);
    unsuffixed!(u8 => u8_unsuffixed);

    pub fn as_str(&self) -> &str {
        &self.inner
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

use std::fmt::Display;

/// An option the user can choose
///
/// (Since the name "Option" is reserved for the well-known type representing
/// nullability, we are calling this one "Choice".)
pub trait Choice {
    /// User visible text
    type Text: Display;
    /// Internal value representing this choice
    type Value;

    /// Get a reference to the text
    fn text(&self) -> &Self::Text;

    /// Get a reference to the value of this choice
    fn value(&self) -> &Self::Value;
}

impl<'a> Choice for &'a str {
    type Text = &'a str;
    type Value = &'a str;

    fn text(&self) -> &Self::Text {
        self
    }

    fn value(&self) -> &Self::Value {
        self
    }
}

impl<'a, T, V> Choice for (T, V)
    where T: Display
{
    type Text = T;
    type Value = V;

    fn text(&self) -> &T {
        &self.0
    }

    fn value(&self) -> &V {
        &self.1
    }
}

use crate::lang;
use crate::num2words::Num2Err;
use crate::{Currency, Number};
use std::str::FromStr;

/// Defines what is a language
pub trait Language {
    fn to_cardinal(self, num: Number) -> Result<String, Num2Err>;
    fn to_ordinal(self, num: Number) -> Result<String, Num2Err>;
    fn to_ordinal_num(self, num: Number) -> Result<String, Num2Err>;
    fn to_year(self, num: Number) -> Result<String, Num2Err>;
    fn to_currency(self, num: Number, currency: Currency) -> Result<String, Num2Err>;
}

/// Languages available in `num2words`
pub enum Lang {
    /// ```
    /// use num2words::{Num2Words, Lang};
    /// assert_eq!(
    ///     Num2Words::new(42).lang(Lang::English).to_words(),
    ///     Ok(String::from("forty-two"))
    /// );
    /// ```
    English,
}

impl FromStr for Lang {
    type Err = ();

    /// Parses a string to return a value of this type
    ///
    ///
    /// | ISO 639-1 | Lang            | 42        |
    /// | --------- | --------------- | --------- |
    /// | `en`      | `Lang::English` | forty-two |
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "en" => Ok(Self::English),
            _ => Err(()),
        }
    }
}

pub fn to_language(lang: Lang) -> impl Language {
    match lang {
        Lang::English => lang::English {},
    }
}

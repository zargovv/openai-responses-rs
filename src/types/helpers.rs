use super::{ContentInput, ContentItem, Input, InputItem, InputListItem, InputMessage, Model};

macro_rules! string_variant {
    ($name:ident, $variant:ident) => {
        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::$variant(value)
            }
        }
        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::from(value.to_string())
            }
        }
    };
}

macro_rules! string_variant_var {
    ($name:ident, $variant:ident, $key:ident) => {
        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::$variant { $key: value }
            }
        }
        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::from(value.to_string())
            }
        }
    };
}

impl From<InputMessage> for InputListItem {
    fn from(value: InputMessage) -> Self {
        Self::Message(value)
    }
}
impl From<InputItem> for InputListItem {
    fn from(value: InputItem) -> Self {
        Self::Item(value)
    }
}

string_variant!(Input, Text);
string_variant!(ContentInput, Text);
string_variant_var!(ContentItem, Text, text);

impl From<String> for Model {
    fn from(s: String) -> Self {
        match s.as_str() {
            "o1" => Self::o1,
            "gpt-4" => Self::Gpt4,
            "gpt-4o" => Self::Gpt4o,
            "o1-mini" => Self::o1Mini,
            "o3-mini" => Self::o3Mini,
            "gpt-4o-mini" => Self::Gpt4oMini,
            "gpt-4o-turbo" => Self::Gpt4Turbo,
            "gpt-3.5-turbo" => Self::Gpt3_5Turbo,
            "gpt-4.5-preview" => Self::Gpt4_5Preview,
            "computer-use-preview" => Self::ComputerUsePreview,
            "gpt-4.1" => Self::Gpt4_1,
            "gpt-5-nano" => Self::Gpt5Nano,
            "gpt-5-mini" => Self::Gpt5Mini,
            "gpt-5" => Self::Gpt5,
            _ => Self::Other(s),
        }
    }
}

impl From<&str> for Model {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

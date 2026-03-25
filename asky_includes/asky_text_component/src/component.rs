use asky_nbt::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Component {
    #[serde(default)]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub bold: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub italic: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub underlined: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub strikethrough: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub obfuscated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub extra: Vec<Component>,
}

const fn is_false(b: &bool) -> bool {
    !*b
}

impl Component {
    pub fn new<S>(content: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            text: content.into(),
            ..Default::default()
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_default()
    }

    pub fn to_nbt(&self) -> Value {
        asky_nbt::to_value(self).unwrap()
    }

    pub fn to_legacy(&self) -> String {
        #[derive(Serialize)]
        struct TextComponent {
            #[serde(default)]
            text: String,
        }
        serde_json::to_string(&TextComponent {
            text: self.to_legacy_impl(true),
        })
        .unwrap_or_default()
    }

    fn to_legacy_impl(&self, is_root: bool) -> String {
        let mut s = String::new();

        if !is_root {
            s.push('§');
            s.push('r');
        }

        // See french docs: https://fr.minecraft.wiki/w/Codes_de_mise_en_forme
        if let Some(color) = &self.color {
            let color_letter = match color.as_str() {
                "black" => '0',
                "dark_blue" => '1',
                "dark_green" => '2',
                "dark_aqua" => '3',
                "dark_red" => '4',
                "dark_purple" => '5',
                "gold" => '6',
                "gray" => '7',
                "dark_gray" => '8',
                "blue" => '9',
                "green" => 'a',
                "aqua" => 'b',
                "red" => 'c',
                "light_purple" => 'd',
                "yellow" => 'e',
                "white" => 'f',
                _ => 'f',
            };
            s.push('§');
            s.push(color_letter);
        }

        if self.bold {
            s.push('§');
            s.push('l');
        }
        if self.italic {
            s.push('§');
            s.push('o');
        }
        if self.underlined {
            s.push('§');
            s.push('n');
        }
        if self.strikethrough {
            s.push('§');
            s.push('m');
        }
        if self.obfuscated {
            s.push('§');
            s.push('k');
        }

        s.push_str(&self.text);

        for extra in &self.extra {
            s.push_str(&extra.to_legacy_impl(false));
        }

        s
    }
}

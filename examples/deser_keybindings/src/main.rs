//! cd to the deser_keybindings repository then do `cargo run`
use {
    crokey::{
        *,
        crossterm::{
            event::{read, Event, KeyEvent},
            style::Stylize,
            terminal,
        },
    },
    serde::{Deserialize, Deserializer},
    std::collections::HashMap,
};

/// This is an example of a configuration structure which contains
/// a map from KeyEvent to String.
///
/// For this example, we use a specific deserializer function but
/// we could have used a map CroKey->String in which case no
/// special function would have been necessary.
#[derive(Deserialize)]
struct Config {
    #[serde(deserialize_with = "deser_keybindings")]
    keybindings: HashMap<KeyEvent, String>,
}

/// A special function to demonstrate how one could deserialize
/// into crokey then replace them with key events.
fn deser_keybindings<'de, D>(deserializer: D) -> Result<HashMap<KeyEvent, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = HashMap::<CroKey, String>::deserialize(deserializer)?;
    Ok(v.into_iter().map(|(ck, s)| (ck.into(), s)).collect())
}

/// An example of what could be a configuration file
static CONFIG_TOML: &str = r#"
[keybindings]
a = "aardvark"
shift-b = "babirussa"
ctrl-k = "koala"
alt-j = "jaguar"
h = "hexapode"
shift-h = "HEXAPODE"
- = "mandrill"
alt-- = "nasalis"
"#;

pub fn main() {
    print!("Application configuration:\n{}", CONFIG_TOML.blue());
    let config: Config = toml::from_str(CONFIG_TOML).unwrap();
    let fmt = KeyEventFormat::default();
    println!("\nType any key combination");
    loop {
        terminal::enable_raw_mode().unwrap();
        let e = read();
        terminal::disable_raw_mode().unwrap();
        match e {
            Ok(Event::Key(key_event)) => {
                if key_event == key!(ctrl-c) || key_event == key!(ctrl-q) {
                    println!("bye!");
                    break;
                }
                if let Some(word) = config.keybindings.get(&key_event) {
                    println!(
                        "You hit {} which is mapped to {}",
                        fmt.to_string(key_event).green(),
                        word.clone().yellow(),
                    );
                } else {
                    println!(
                        "You hit {} which isn't mapped",
                        fmt.to_string(key_event).red(),
                    );
                }
            }
            _ => {}
        }
    }
}

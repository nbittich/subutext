use rand::Rng;
use std::collections::{HashMap, HashSet};

const KEYS:&str = "😬😀😁😂😃😄😅😆😇😉😊🙂😋😍😘😗😙😚📡😜😝😜😎💾📲⌚😠😡😔😕😤👏🙏💪✋👌👁👀👥👴👕👞🎩👑🕶💼🐻🐼🐯🐌🐏🐐🐈🐿🐲🌗🍋🍉🏉🎯🎲🚞🚆🏤📸📹🎏💔🆘🆑🈲🈵⛔🚫❌✅🛃🚻🚾🔕💲🔠🎶⁉💯🔞🚱🚳📵❓📛🛐📐📗🗃🗳🗄📯🗺🛍🚽🔑🛋🛌🛏🔫💣🗡🔱";

#[derive(Debug)]
pub struct HomophonCypher {
    clear_text: String,
    keyspace: HashMap<char, Vec<char>>,
}

impl HomophonCypher {
    pub fn transform(&self) -> String {
        let cyphered_text: String = self
            .clear_text
            .chars()
            .map(|c| {
                if !self.keyspace.contains_key(&c) {
                    return c;
                }
                let homophons = self.keyspace.get(&c).unwrap();
                let random = rand::thread_rng().gen_range(0..homophons.len());
                homophons.get(random).map(|x| *x).unwrap()
            })
            .collect();
        cyphered_text
    }
    pub fn new(text: String) -> Self {
        let sanitized_text: String = text
            .to_uppercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c == &'\n')
            .collect();
        let unique_characters: HashSet<char> = sanitized_text.chars().collect();
        let mut available_keys: Vec<char> = KEYS.chars().collect();
        let keyspace: HashMap<char, Vec<char>> = unique_characters
            .iter()
            .filter(|c| c != &&'\n')
            .map(|c| {
                let vector: Vec<char> = (1..rand::thread_rng().gen_range(3..8))
                    .into_iter()
                    .map(|_i| {
                        (available_keys
                            .remove(rand::thread_rng().gen_range(0..available_keys.len())))
                    })
                    .collect();
                (*c, vector)
            })
            .collect();

        HomophonCypher {
            clear_text: sanitized_text,
            keyspace: keyspace,
        }
    }
}

#[cfg(test)]
mod test {
    use super::HomophonCypher;
    #[test]
    fn test_emoji() {
        let cypher = HomophonCypher::new(String::from("This is a text in frenglish"));
        println!("{:?}", cypher);
        let encoded = cypher.transform();
        println!("{}", encoded);
    }
}

use nlprule::{Tokenizer, tokenizer_filename};
use std::collections::HashSet;

pub struct Dictionary {
    tokenizer: Tokenizer,
}

impl Dictionary {
    pub fn new() -> Self {
        let mut tokenizer_bytes: &'static [u8] =
            include_bytes!(concat!(env!("OUT_DIR"), "/", tokenizer_filename!("en")));
        let tokenizer =
            Tokenizer::from_reader(&mut tokenizer_bytes).expect("tokenizer binary is valid");
        Self { tokenizer }
    }

    /// 英語の原型を取得する
    /// # Returns
    /// * `Some(String)` - 原型が見つかった場合
    /// * `Some(String)` - 単語が既に原型になっている場合
    /// * `None` - 原型が見つからなかった場合
    pub fn get_base_form<S: AsRef<str>>(&self, word: S) -> Option<String> {
        let word = word.as_ref();
        if word.is_empty() {
            return None;
        }

        let sentence = self.tokenizer.sentencize(word);
        assert_eq!(sentence.count(), 1);
        self.tokenizer.sentencize(word).next().and_then(|sentence| {
            let a = sentence
                .into_iter()
                .flat_map(|token| {
                    token
                        .word()
                        .tags()
                        .iter()
                        .map(|n| n.lemma().as_str().to_string())
                        .collect::<HashSet<_>>()
                })
                .collect::<HashSet<_>>();
            match a.len() {
                1 => a.into_iter().next(),              // 既に原型になっている
                2 => a.into_iter().find(|n| n != word), // 原型ではないので、原型を探す (単語自身も含まれるので find で探す)
                _ => None,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_base_form() {
        let word = Dictionary::new();
        assert_eq!(word.get_base_form("cats").unwrap(), "cat");
        assert_eq!(word.get_base_form("running").unwrap(), "run");
        assert_eq!(word.get_base_form("ran").unwrap(), "run");
        assert_eq!(word.get_base_form("are").unwrap(), "be");
        assert_eq!(word.get_base_form("was").unwrap(), "be");
        assert_eq!(word.get_base_form("had").unwrap(), "have");
        assert_eq!(word.get_base_form("children").unwrap(), "child");
        assert_eq!(word.get_base_form("word").unwrap(), "word");
        assert_eq!(
            word.get_base_form("state-of-the-art").unwrap(),
            "state-of-the-art"
        );
        // on-site
        // saving
    }
}

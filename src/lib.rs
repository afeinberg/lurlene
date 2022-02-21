use log::debug;
use serde::{Deserialize, Serialize};
use std::collections;
use std::convert::TryFrom;
use std::error;

type Documents = collections::BTreeMap<u32, String>;
type IndexMap = collections::BTreeMap<String, collections::BTreeMap<u32, u32>>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Index {
    count: u32,
    docs: Documents,
    index: IndexMap,
}

impl Default for Index {
    fn default() -> Self {
        Index {
            count: 0,
            docs: Documents::new(),
            index: IndexMap::new(),
        }
    }
}

impl TryFrom<&str> for Index {
    type Error = Box<dyn error::Error>;

    fn try_from(json_text: &str) -> Result<Self, Self::Error> {
        let res = serde_json::from_str::<Self>(json_text)?;
        debug!("{:?}", res);
        Ok(res)
    }
}

impl Index {
    pub fn add(&mut self, name: &str, content: &str) {
        self.count += 1;
        self.docs.insert(self.count, name.to_string());
        for word in content.split_whitespace() {
            let word = word.to_lowercase();
            match self.index.get_mut(&word) {
                Some(entry) => {
                    match entry.get_mut(&self.count) {
                        Some(n) => *n += 1,
                        None => {
                            entry.insert(self.count, 1);
                        }
                    };
                }
                None => {
                    let mut v = collections::BTreeMap::new();
                    v.insert(self.count, 1);
                    self.index.insert(word, v);
                }
            }
        }
    }

    pub fn search(&self, key: &str) -> Vec<(String, u32)> {
        let key = key.to_lowercase();
        match self.index.get(&key) {
            Some(freq_list) => {
                // collect doc ids from frequency list
                let mut doc_ids: Vec<u32> = freq_list.keys().copied().collect();

                doc_ids.sort_by(|doc_a_id, doc_b_id| {
                    // sort doc ids by frequency
                    let a = freq_list.get(doc_a_id).unwrap();
                    let b = freq_list.get(doc_b_id).unwrap();
                    a.cmp(b)
                });

                // collect ordered doc names with frequency
                let doc_names: Vec<(String, u32)> = doc_ids
                    .iter()
                    .map(|doc_id| {
                        (
                            self.docs.get(doc_id).unwrap().clone(),
                            *freq_list.get(doc_id).unwrap(),
                        )
                    })
                    .collect();
                doc_names
            }
            None => vec![],
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn sample_index() -> Index {
        let mut index = Index::default();
        index.add("foo", "i love emacs");
        index.add("bar", "all love emacs");
        index
    }

    #[test]
    fn test_search_and_add() {
        let index = sample_index();

        for key in ["emacs", "love"] {
            assert_eq!(
                index.search(key),
                vec![(String::from("foo"), 1u32), (String::from("bar"), 1u32)]
            );
        }
        for key in ["i", "all"] {
            assert_eq!(index.search(key).len(), 1);
        }
    }

    #[test]
    fn test_serde() {
        let index = sample_index();
        let index_json = index.serialize();
        let index_from_json = Index::try_from(index_json.as_str()).unwrap();
        assert_eq!(index, index_from_json);
    }
}

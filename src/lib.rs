use std::collections;

type Documents = collections::BTreeMap<u32, String>;
type IndexMap = collections::BTreeMap<String, collections::BTreeMap<u32, u32>>;

#[derive(Debug, Clone)]
pub struct Index {
    count: u32,
    docs: Documents,
    index: IndexMap,
}

impl Index {
    pub fn new() -> Self {
        Index {
            count: 0,
            docs: Documents::new(),
            index: IndexMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, content: &str) {
        self.count += 1;
        self.docs.insert(self.count, name.to_string());
        for word in content.split_whitespace() {
            let word = word.to_lowercase();
            match self.index.get_mut(&word) {
                Some(entry) => {
                    match entry.get_mut(&self.count) {
                        Some(n) => *n = *n + 1,
                        None => {
                            entry.insert(self.count, 1);
                        }
                    };
                }
                None => {
                    let mut v = std::collections::BTreeMap::new();
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
                let mut doc_ids: Vec<u32> = freq_list.keys().map(|doc_id| *doc_id).collect();

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
                            self.docs.get(&doc_id).unwrap().clone(),
                            *freq_list.get(&doc_id).unwrap(),
                        )
                    })
                    .collect();
                doc_names
            }
            None => vec![],
        }
    }
}

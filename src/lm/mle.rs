use std::collections::BTreeMap;

struct Vocabulary<'a> {
    cutoff: usize,
    counter: Counter<'a>,
    unk_label: &'a str,
}

impl <'a>Vocabulary<'a> {
    pub(crate) fn new(cutoff: usize) -> Self {
        Self {
            cutoff,
            counter: Counter::new(),
            unk_label: "<UNK>",
        }
    }

    pub fn update_word(&mut self, word: &'a str) {
        self.counter.update_word(word);
    }

    pub fn update_sentence(&mut self, sentence: impl Iterator<Item=&'a &'a str>) {
        self.counter.update_sentence(sentence);
    }

    pub fn lookup_word(&self, word: &'a str) -> &str {
        return if self.counter.get(word) > self.cutoff {
            word
        } else {
            self.unk_label
        };
    }

    pub(crate) fn lookup_sentence(&self, sentence: impl Iterator<Item=&'a &'a  str> + 'a) -> impl Iterator<Item=&'a str> + '_{
        sentence.map(|word| if self.counter.get(word) > self.cutoff {
            word
        } else {
            self.unk_label
        })
    }
}

struct Counter<'a> {
    counts: BTreeMap<&'a str, usize>, //may just need hashmap, not sure yet, do we need ordered keys?
}

impl<'a> Counter<'a> {
    pub(crate) fn new() -> Self {
        Self {
            counts: BTreeMap::new()
        }
    }

    pub(crate) fn update_word(&mut self, word: &'a str) {
        let count = self.counts.entry(word).or_insert(0);
        *count += 1;
    }

    pub(crate) fn update_sentence(&mut self, sentence: impl Iterator<Item=&'a &'a str>) {
        sentence.for_each(|word| self.update_word(word));
    }

    pub(crate) fn get(&self, word: &str) -> usize {
        *self.counts.get(word).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        let mut vocab = Vocabulary::new(1);
        vocab.update_sentence(["a", "b", "c", "a", "b", "c"].iter());
        let looked_up: Vec<&str> = vocab.lookup_sentence(["a", "b", "c"].iter()).collect();
        assert_eq!(looked_up, vec!["a", "b", "c"]);
        let looked_up: Vec<&str> = vocab.lookup_sentence(["Aliens", "from", "Mars"].iter()).collect();
        assert_eq!(looked_up, vec!["<UNK>", "<UNK>", "<UNK>"]);
    }

    #[test]
    fn test_lookup_below_cutoff() {
        let mut vocab = Vocabulary::new(1);
        vocab.update_sentence(["a", "b", "c"].iter());
        let looked_up: Vec<&str> = vocab.lookup_sentence(["a", "b", "c"].iter()).collect();
        assert_eq!(looked_up, vec!["<UNK>", "<UNK>", "<UNK>"]);
    }

    #[test]
    fn test_count_words() {
        let mut counter = Counter::new();
        counter.update_word("a");

        assert_eq!(counter.get("a"), 1);
    }

    #[test]
    fn test_count_sentence() {
        let mut counter = Counter::new();
        counter.update_sentence(["a", "b", "a"].iter());

        assert_eq!(counter.get("a"), 2);
        assert_eq!(counter.get("b"), 1);
    }
}
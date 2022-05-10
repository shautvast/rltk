pub struct NGramSequenceIter<'a> {
    sequence: Box<dyn Iterator<Item=&'a &'a str> + 'a>,
    n: usize,
    current_ngram: Vec<&'a &'a str>,
}

impl<'a> NGramSequenceIter<'a> {
    pub(crate) fn new(sequence: impl Iterator<Item=&'a &'a str> + 'a, n: usize) -> Self {
        Self {
            sequence: Box::new(sequence),
            n,
            current_ngram: Vec::new(),
        }
    }
}

impl<'a> Iterator for NGramSequenceIter<'a> {
    type Item = Box<dyn Iterator<Item=&'a &'a str> + 'a>;

    fn next(&mut self) -> Option<Self::Item> {
        return if self.current_ngram.len() == 0 {
            for _ in 0..self.n {
                if let Some(item) = self.sequence.next() {
                    self.current_ngram.push(item);
                } else {
                    return None; // n > len
                }
            }

            Some(Box::new(self.current_ngram.clone().into_iter()))
        } else {
            self.current_ngram.remove(0);
            let maybe_next = self.sequence.next();
            if maybe_next.is_some() {
                self.current_ngram.push(&maybe_next.unwrap());
                Some(Box::new(self.current_ngram.clone().into_iter()))
            } else {
                None
            }
        };
    }
}

pub struct EveryGramSequenceIter<'a> {
    sequence: Box<dyn Iterator<Item=&'a &'a str> + 'a>,
    n: usize,
    current_ngram: Vec<&'a &'a str>,
    current_size: usize,
}

impl<'a> EveryGramSequenceIter<'a> {
    pub(crate) fn everygrams(sequence: impl Iterator<Item=&'a &'a str> + 'a, n: usize) -> Self {
        Self {
            sequence: Box::new(sequence),
            n,
            current_ngram: Vec::new(),
            current_size: 0,
        }
    }
}

impl<'a> Iterator for EveryGramSequenceIter<'a> {
    type Item = Box<dyn Iterator<Item=&'a &'a str> + 'a>;

    //noinspection DuplicatedCode, hard to deduplicate because of early return
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_ngram.len() == 0 {
            for _ in 0..self.n {
                if let Some(item) = self.sequence.next() {
                    self.current_ngram.push(item);
                } else {
                    return None; // n > len
                }
            }
        }

        self.current_size += 1;

        if self.current_size > self.n {
            self.current_size = 1;
            self.current_ngram.remove(0);
            let maybe_next = self.sequence.next();
            if maybe_next.is_some() {
                self.current_ngram.push(&maybe_next.unwrap());
            } else {
                self.n = 0; // not pretty, but ensures that the following next will be the last
                if self.current_ngram.len() == 0 {
                    return None;
                }
            }
        }

        return Some(Box::new(self.current_ngram.clone().into_iter().take(self.current_size)));
    }
}

pub struct FlatteningIter<'a> {
    ngrams: Box<dyn Iterator<Item=Box<dyn Iterator<Item=&'a &'a str> + 'a>> + 'a>,
    current_ngram: Option<Box<dyn Iterator<Item=&'a &'a str> + 'a>>,
}

impl<'a> FlatteningIter<'a> {
    pub(crate) fn new(ngrams: impl Iterator<Item=Box<dyn Iterator<Item=&'a &'a str> + 'a>> + 'a) -> Self {
        Self {
            ngrams: Box::new(ngrams),
            current_ngram: None,
        }
    }
}

impl<'a> Iterator for FlatteningIter<'a> {
    type Item = &'a &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_ngram.is_none() {
            self.current_ngram = self.ngrams.next();
        }

        while let Some(ref mut current_ngram) = self.current_ngram {
            let current_item = current_ngram.next();
            if current_item.is_some() {
                return current_item;
            } else {
                self.current_ngram = self.ngrams.next();
            }
        }

        None
    }
}
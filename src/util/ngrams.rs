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
    max_order: usize,
    current_ngram: Vec<&'a &'a str>,
    current_order: usize,
}

impl<'a> EveryGramSequenceIter<'a> {
    pub(crate) fn everygrams(sequence: impl Iterator<Item=&'a &'a str> + 'a, max_order: usize) -> Self {
        Self {
            sequence: Box::new(sequence),
            max_order,
            current_ngram: Vec::new(),
            current_order: 0,
        }
    }
}

impl<'a> Iterator for EveryGramSequenceIter<'a> {
    type Item = Box<dyn Iterator<Item=&'a &'a str> + 'a>;

    //noinspection DuplicatedCode, hard to deduplicate because of early return
    fn next(&mut self) -> Option<Self::Item> {
        // initiate a temp buffer (current_ngram) from which
        if self.current_ngram.len() == 0 {
            for _ in 0..self.max_order {
                if let Some(item) = self.sequence.next() {
                    self.current_ngram.push(item);
                } else {
                    return None; // n > len
                }
            }
        }

        self.current_order += 1;

        // slide window to the right in the sentence, if all ngrams of desired max order have been iterated
        // and accomodate for end of sentence
        if self.current_order > self.max_order { // last item of current ngram reached
            self.current_order = 1; // start again with 1
            self.current_ngram.remove(0); // first item is not part of any coming ngrams, and can be removed
            let maybe_next = self.sequence.next(); // next item in source
            if maybe_next.is_some() {
                self.current_ngram.push(&maybe_next.unwrap());
            } else {
                self.max_order -= 1; // the desired max ngram length gets shorter at the end where there are no more new items in the iterator
                // theoretically it would be better if we do not mutate max_order and a use a new variable "desired_max_order" oder so etwas.
                if self.current_ngram.len() == 0 { // all items have been removed and no new have been added, we're at the end
                    return None;
                }
            }
        }
        // take n items from the ngram where n (current_order) is incremented (unigram, bigram, trigram etc)
        return Some(Box::new(self.current_ngram.clone().into_iter().take(self.current_order)));
    }
}

/// like flatmap fn
pub struct FlatteningIter<'a> {
    list_of_lists: Box<dyn Iterator<Item=Box<dyn Iterator<Item=&'a &'a str> + 'a>> + 'a>,
    current: Option<Box<dyn Iterator<Item=&'a &'a str> + 'a>>,
}

impl<'a> FlatteningIter<'a> {
    pub(crate) fn new(ngrams: impl Iterator<Item=Box<dyn Iterator<Item=&'a &'a str> + 'a>> + 'a) -> Self {
        Self {
            list_of_lists: Box::new(ngrams),
            current: None,
        }
    }
}

impl<'a> Iterator for FlatteningIter<'a> {
    type Item = &'a &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            self.current = self.list_of_lists.next();
        }

        while let Some(ref mut current_ngram) = self.current {
            let current_item = current_ngram.next();
            if current_item.is_some() {
                return current_item;
            } else {
                self.current = self.list_of_lists.next();
            }
        }

        None
    }
}
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug, Eq, Ord)]
pub struct NGram(Vec<&'static str>);

impl NGram {
    pub fn new(elements: Vec<&'static str>) -> Self {
        Self {
            0: elements
        }
    }

    pub fn new_bigram(element1: &'static str, element2: &'static str) -> Self {
        Self {
            0: vec![element1, element2]
        }
    }

    pub fn new_trigram(element1: &'static str, element2: &'static str, element3: &'static str) -> Self {
        Self {
            0: vec![element1, element2, element3]
        }
    }

    pub fn tail(&self) -> Self {
        Self {
            0: self.0[1..].to_vec()
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> &'static str {
        unsafe {
            self.0.get_unchecked(index)
        }
    }
}

impl PartialEq for NGram {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        } else {
            for (i, element) in self.0.iter().enumerate() {
                if *element != other.get(i) {
                    return false;
                }
            }
        }

        true
    }
}

impl PartialOrd for NGram {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else {
            for (i, element) in self.0.iter().enumerate() {
                if let Some(ordering) = element.partial_cmp(&other.get(i)) {
                    if ordering != Ordering::Equal {
                        return Some(ordering);
                    }
                }
            }
            Some(Ordering::Equal)
        }
    }
}

pub struct Model {
    word_counts: BTreeMap<&'static str, u32>,
    ngram_counts: BTreeMap<NGram, u32>,
}

impl Model {
    pub fn calc_digrams(corpus: Vec<Vec<&'static str>>) -> Self {
        let mut word_counts = BTreeMap::new();
        let mut ngram_counts = BTreeMap::new();
        for sentence in corpus {
            for word in sentence.iter() {
                let count = word_counts.entry(*word).or_insert(0);
                *count += 1;
            }

            for i in 0..sentence.len() - 1 {
                let ngram = NGram::new(vec![sentence[i], sentence[i + 1]]);
                let count = ngram_counts.entry(ngram).or_insert(0);
                *count += 1;
            }
        }
        Self {
            ngram_counts,
            word_counts,
        }
    }

    // only tested for 2-grams, and that's only happy cases
    pub fn p(&self, ngram: NGram) -> Option<f64> {
        // let mut probability = (*self.word_counts.get(ngram.get(0)).unwrap() as f64) / self.word_counts.len() as f64;
        //
        //
        // for index in 0..ngram.len() - 1 {
        //     println!("{}", probability);
        //
        //
        //
        //     println!("{}", ng_p);
        //     probability = ng_p * probability;
        // }
        // println!("{}", probability);
        // Some(probability)
        None
    }

    fn p_ngram(&self, ngram: NGram, intermediate: f64) -> f64 {
        // for index in 0..ngram.len() - 1 {
        //     self.ngram_counts.get(&ngram)
        //         .map(|count| (*count as f64) / (self.word_counts[ngram.0.get(0).unwrap()] as f64)).unwrap()
        // }
        //
        // if ngram.len() > 2 {
        //     println!("{}", intermediate);
        //     intermediate * self.pp(ngram.tail(), intermediate) //TODO
        // } else {
        //     self.ngram_counts.get(&ngram)
        //         .map(|count| (*count as f64) / (self.word_counts[ngram.0.get(0).unwrap()] as f64)).unwrap() //TODO
        // }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_model() {
        let corpus = vec![
            vec!["<s>", "I", "am", "Sam", "</s>"],
            vec!["<s>", "Sam", "I", "am", "</s>"],
            vec!["<s>", "I", "do", "not", "like", "eggs", "and", "ham", "</s>"],
        ];

        let model = Model::calc_digrams(corpus);

        // assert_eq!(model.p(NGram::new(vec!["<s>", "I"])), Some(0.6666666666666666_f64));
        // assert_eq!(model.p(NGram::new(vec!["Sam", "</s>"])), Some(0.5_f64));
        // assert_eq!(model.p(NGram::new(vec!["<s>", "Sam"])), Some(0.33333333333333333_f64));
        // assert_eq!(model.p(NGram::new(vec!["am", "Sam"])), Some(0.5_f64));
        // assert_eq!(model.p(NGram::new(vec!["I", "am"])), Some(0.6666666666666666_f64));

        println!("{:?}", model.p(NGram::new(vec!["I", "am", "Sam"])));
    }

    #[test]
    fn test_ngram_eq() {
        let n1 = NGram::new(vec!["1", "2"]);
        let n2 = NGram::new(vec!["1", "2"]);
        let n3 = NGram::new(vec!["3", "4"]);

        assert_eq!(n1, n2);
        assert_ne!(n1, n3);
        assert_ne!(n2, n3);
    }

    #[test]
    fn test_ngram_tail() {
        let n1 = NGram::new(vec!["1", "2", "3"]);
        let n2 = NGram::new(vec!["2", "3"]);
        assert_eq!(n1.tail(), n2);
    }
}
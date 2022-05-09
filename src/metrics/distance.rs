// sandbox, to be removed

use unicode_segmentation::UnicodeSegmentation;

pub(crate) struct Element {
    pub(crate) value: usize,
}

impl Element {
    fn new() -> Self {
        Self {
            value: 0,
        }
    }
}



// non recursive implementation requires a table
// my guess is that this is more efficient (should check)
pub(crate) fn get_edit_distance_table(word1: &str, word2: &str) -> Vec<Vec<Element>> {
    // create table
    let mut table = Vec::new();
    for _ in 0..=word1.len() {
        let mut row = Vec::new();
        for _ in 0..=word2.len() {
            row.push(Element::new())
        }
        table.push(row);
    }

    // set the boundaries
    for i in 0..=word1.len() {
        table[i][0].value = i;
    }
    for i in 1..=word2.len() {
        table[0][i].value = i;
    }

    for (i1, g1) in word1.graphemes(true).enumerate() {
        for (i2, g2) in word2.graphemes(true).enumerate() {
            let d_del = table[i1][i2 + 1].value + 1; //deletion
            let d_ins = table[i1 + 1][i2].value + 1; //insertion
            let d_sub = table[i1][i2].value + (if g1 == g2 { 0 } else { 2 }); // substitution
            let min = usize::min(d_del, usize::min(d_ins, d_sub));
            let element = table[i1 + 1].get_mut(i2 + 1).unwrap();
            element.value = min;
        }
    }
    table
}


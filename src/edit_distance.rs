use unicode_segmentation::UnicodeSegmentation;

//could also be powers of 2 that are combined using bitwise-or
// enum Backtrace {
//     LEFT,
//     DOWN,
//     DIAGONAL,
// }

struct Element {
    value: usize,
    // backtraces: Vec<Backtrace>,
}

impl Element {
    fn new() -> Self {
        Self {
            value: 0,
            // backtraces: Vec::new(),
        }
    }
}

pub fn get_levenshtein_distance(word1: &str, word2: &str) -> usize {
    get_edit_distance_table(word1, word2)[word1.len()][word2.len()].value
}

// non recursive implementation requires a table
// my guess is that this is more efficient (should check)
fn get_edit_distance_table(word1: &str, word2: &str) -> Vec<Vec<Element>> {
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
            // if d_del == min {
            //     element.backtraces.push(Backtrace::DOWN);
            // }
            // if d_ins == min {
            //     element.backtraces.push(Backtrace::LEFT);
            // }
            // if d_sub == min {
            //     element.backtraces.push(Backtrace::DIAGONAL);
            // }
        }
    }
    table
}

#[cfg(test)]
mod tests {
    use super::{get_edit_distance_table, get_levenshtein_distance};

    #[test]
    fn test_get_levenshtein_distance() {
        assert_eq!(get_levenshtein_distance("intention", "execution"), 8);
    }

    #[test]
    fn test_get_edit_distance_table() {
        // example from Stanford NLP course: https://youtu.be/kgcEaoM_QJA
        let word1 = "intention";
        let word2 = "execution";

        let outcome: [[usize; 10]; 10] = [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 5, 6, 7, 6, 7, 8], [2, 3, 4, 5, 6, 7, 8, 7, 8, 7], [3, 4, 5, 6, 7, 8, 7, 8, 9, 8], [4, 3, 4, 5, 6, 7, 8, 9, 10, 9],
            [5, 4, 5, 6, 7, 8, 9, 10, 11, 10], [6, 5, 6, 7, 8, 9, 8, 9, 10, 11], [7, 6, 7, 8, 9, 10, 9, 8, 9, 10], [8, 7, 8, 9, 10, 11, 10, 9, 8, 9], [9, 8, 9, 10, 11, 12, 11, 10, 9, 8]];

        let tab = get_edit_distance_table(word1, word2);

        for (rowindex, row) in tab.iter().enumerate() {
            for (colindex, element) in row.iter().enumerate() {
                assert_eq!(outcome[rowindex][colindex], element.value);
            }
        }
    }
}
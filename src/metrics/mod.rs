pub mod distance;

pub fn edit_distance(s1: &str, s2: &str) -> usize {
    distance::get_edit_distance_table(s1, s2)[s1.len()][s2.len()].value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_levenshtein_distance() {
        assert_eq!(edit_distance("intention", "execution"), 8);
    }

    #[test]
    fn test_get_edit_distance_table() {
        // example from Stanford NLP course: https://youtu.be/kgcEaoM_QJA
        let word1 = "intention";
        let word2 = "execution";

        let outcome: [[usize; 10]; 10] = [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 5, 6, 7, 6, 7, 8], [2, 3, 4, 5, 6, 7, 8, 7, 8, 7], [3, 4, 5, 6, 7, 8, 7, 8, 9, 8], [4, 3, 4, 5, 6, 7, 8, 9, 10, 9],
            [5, 4, 5, 6, 7, 8, 9, 10, 11, 10], [6, 5, 6, 7, 8, 9, 8, 9, 10, 11], [7, 6, 7, 8, 9, 10, 9, 8, 9, 10], [8, 7, 8, 9, 10, 11, 10, 9, 8, 9], [9, 8, 9, 10, 11, 12, 11, 10, 9, 8]];

        let tab = distance::get_edit_distance_table(word1, word2);

        for (rowindex, row) in tab.iter().enumerate() {
            for (colindex, element) in row.iter().enumerate() {
                assert_eq!(outcome[rowindex][colindex], element.value);
            }
        }
    }
}
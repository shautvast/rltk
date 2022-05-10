use std::slice::Iter;

pub fn should_be_equal_lists<'a>(left: impl Iterator<Item=&'a &'a str>, right: Vec<&'a str>) {
    for (left, right) in left.zip(right.into_iter()) {
        assert_eq!(*left, right);
    }
}

pub fn should_be_equal_list_of_lists<'a>(actual: &mut impl Iterator<Item=impl Iterator<Item=&'a &'a str>>, expected: Vec<Iter<&'a str>>) {
    let mut actual_vec=Vec::new();
    actual_vec.extend(actual);
    assert_eq!(actual_vec.len(), expected.len());
    for (actual_outer, expected_outer) in actual_vec.into_iter().zip(expected.into_iter()) {
        for (actual_inner, expected_inner) in actual_outer.zip(expected_outer) {
            assert_eq!(actual_inner, expected_inner);
        }
    }
}
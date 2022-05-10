use std::slice::Iter;

pub fn should_be_equal_lists<'a>(left: impl Iterator<Item=&'a &'a str>, right: Vec<&'a str>) {
    for (left, right) in left.zip(right.into_iter()) {
        assert_eq!(*left, right);
    }
}

pub fn should_be_equal_list_of_lists<'a>(actual: &mut impl Iterator<Item=impl Iterator<Item=&'a &'a str>>, expected: Vec<Iter<&'a str>>) {
    let actual = collect(actual);
    assert_eq!(actual.len(), expected.len());
    for (actual_outer, expected_outer) in actual.into_iter().zip(expected.into_iter()) {
        let actual_outer: Vec<&&str> = actual_outer.collect();
        let expected_outer: Vec<&&str> = expected_outer.collect();
        assert_eq!(actual_outer.len(), expected_outer.len());
        for (actual_inner, expected_inner) in actual_outer.iter().zip(expected_outer.iter()) {
            assert_eq!(actual_inner, expected_inner);
        }
    }
}

fn collect<'a>(iter: &mut impl Iterator<Item=impl Iterator<Item=&'a &'a str>>) -> Vec<impl Iterator<Item=&'a &'a str>> {
    let mut vec = Vec::new();
    vec.extend(iter);
    vec
}
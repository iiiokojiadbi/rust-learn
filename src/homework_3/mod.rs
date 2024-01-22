fn satisfying_index_check(elements: &[u32], index: usize) -> bool {
    elements.is_empty() || !(index <= elements.len() && index > elements.len())
}

pub fn get_tuple_element(elements: &mut (u32, u32), first: bool) -> &mut u32 {
    if first {
        &mut elements.1
    } else {
        &mut elements.0
    }
}

pub fn get_element_by_index(elements: &mut [u32], index: usize) -> Option<&mut u32> {
    if !satisfying_index_check(elements, index) {
        None
    } else {
        Some(&mut elements[index])
    }
}

pub fn get_last_element_by_index(elements: &[u32], index: usize) -> Option<&u32> {
    Some(&elements[elements.len() - 1 - index])
}

pub fn get_splitted_slice(elements: &[u32], index: usize) -> Option<(&[u32], &[u32])> {
    if !satisfying_index_check(elements, index) {
        None
    } else {
        Some((&elements[..index], &elements[index..]))
    }
}

pub fn get_splitted_slice_into_parts(elements: &[u32], parts: usize) -> Vec<&[u32]> {
    let mut slices = vec![];
    let parts = (elements.len() as f32 / parts as f32).ceil() as usize;

    for index in 0..=(elements.len() / parts) {
        let start = index * parts;
        let last = if start + parts > elements.len() {
            elements.len()
        } else {
            start + parts
        };

        let slice = &elements[start..last];

        if !slice.is_empty() {
            slices.push(slice);
        }
    }

    slices
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fmt::Debug;

    fn match_option_result_eq<T: PartialEq<F> + Debug, F: Debug>(result: Option<T>, expected: F) {
        match result {
            Some(value) => {
                assert_eq!(value, expected);
            }
            None => {
                panic!("Элемент по индексу не найден");
            }
        }
    }

    #[test]
    fn test_get_tuple_element() {
        let mut elements = (11, 22);

        let expected = &mut 11;
        assert_eq!(get_tuple_element(&mut elements, false), expected);

        let expected = &mut 22;
        assert_eq!(get_tuple_element(&mut elements, true), expected);
    }

    #[test]
    fn test_get_element_by_index() {
        let mut elements = [1, 2, 3, 4, 5];

        let expected = &mut 1;
        match_option_result_eq(get_element_by_index(&mut elements, 0), expected);

        let expected = &mut 5;
        match_option_result_eq(get_element_by_index(&mut elements, 4), expected);
    }

    #[test]
    fn test_get_last_element_by_index() {
        let elements = [1, 2, 3];

        let expected = &3;
        match_option_result_eq(get_last_element_by_index(&elements, 0), expected);

        let expected = &1;
        match_option_result_eq(get_last_element_by_index(&elements, 2), expected);
    }

    #[test]
    fn test_get_splitted_slice() {
        let elements = [1, 2, 3, 4, 5, 6];

        let expected: (&[u32], &[u32]) = (&[], &elements);
        match_option_result_eq(get_splitted_slice(&elements, 0), expected);

        let expected: (&[u32], &[u32]) = (&[1, 2], &[3, 4, 5, 6]);
        match_option_result_eq(get_splitted_slice(&elements, 2), expected);

        let expected: (&[u32], &[u32]) = (&elements, &[]);
        match_option_result_eq(get_splitted_slice(&elements, elements.len()), expected);
    }

    #[test]
    fn test_get_splitted_slice_into_parts() {
        let elements = [1, 2, 3, 4, 5, 6, 7, 8];

        let expected: Vec<&[u32]> = vec![&[1, 2], &[3, 4], &[5, 6], &[7, 8]];
        assert_eq!(get_splitted_slice_into_parts(&elements, 4), expected);

        let expected: Vec<&[u32]> = vec![&[1, 2, 3], &[4, 5, 6], &[7, 8]];
        assert_eq!(get_splitted_slice_into_parts(&elements, 3), expected);

        let expected: Vec<&[u32]> = vec![&[1, 2, 3, 4, 5, 6, 7, 8]];
        assert_eq!(get_splitted_slice_into_parts(&elements, 1), expected);

        let expected: Vec<&[u32]> = vec![&[1], &[2], &[3], &[4], &[5], &[6], &[7], &[8]];
        assert_eq!(get_splitted_slice_into_parts(&elements, 8), expected);

        let expected: Vec<&[u32]> = vec![&[1], &[2], &[3], &[4], &[5], &[6], &[7], &[8]];
        assert_eq!(get_splitted_slice_into_parts(&elements, 200), expected);
    }
}

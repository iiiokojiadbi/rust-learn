// Функционал библиотеки:
//
// 1) Принимает мутабельную ссылку на кортеж и bool значение.
//    * Если false, возвращает мутабельную ссылку на первый элемент кортежа.
//    * Если true, возвращает мутабельную ссылку на второй элемент кортежа.
// 2) Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
// 3) Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
// 4) Принимает слайс и число N. Возвращает два слайса с элементами:
//    * с нулевого по N-1;
//    * с N-го по последний;
// 5) Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
//    Протестировать функции.
//    Убедиться, что компилятор не позволит вернуть более одной мутабельной ссылки на один объект.
//
// Требования:
// * Реализованы и протестированы все перечисленные функции.
// * ""cargo clippy"" и ""cargo fmt --check"" не выдают предупреждений и ошибок.

pub fn get_tuple_element(elements: &mut (u32, u32), first: bool) -> &mut u32 {
    if first {
        &mut elements.1
    } else {
        &mut elements.0
    }
}

pub fn get_element_by_index(elements: &mut [u32], index: usize) -> Option<&mut u32> {
    if elements.is_empty() || (index <= elements.len() && index >= elements.len()) {
        None
    } else {
        Some(&mut elements[index])
    }
}

pub fn get_last_element_by_index(elements: &mut [u32], index: usize) -> Option<&mut u32> {
    get_element_by_index(elements, elements.len() - 1 - index)
}

#[cfg(test)]
mod test {
    use super::*;

    fn match_result_eq(result: Option<&mut u32>, expected: &mut u32) {
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

        assert_eq!(get_tuple_element(&mut elements, false), &mut 11);
        assert_eq!(get_tuple_element(&mut elements, true), &mut 22);
    }

    #[test]
    fn test_get_element_by_index() {
        let mut slice = [1, 2, 3, 4, 5];

        match_result_eq(get_element_by_index(&mut slice, 0), &mut 1);
        match_result_eq(get_element_by_index(&mut slice, 4), &mut 5);
    }

    #[test]
    fn test_get_last_element_by_index() {
        let mut slice = [1, 2, 3];

        match_result_eq(get_last_element_by_index(&mut slice, 0), &mut 3);
        match_result_eq(get_last_element_by_index(&mut slice, 2), &mut 1);
    }
}

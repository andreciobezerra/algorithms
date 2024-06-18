pub fn selection_sort(list: &Vec<isize>) -> Vec<isize> {
    let mut list_copy = list.clone();
    let mut result: Vec<isize> = Vec::with_capacity(list.len());

    while list_copy.len() > 0 {
        let index = list_copy
            .iter()
            .position(|elem| elem == list_copy.iter().min().unwrap())
            .unwrap();
        result.push(list_copy.remove(index));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let list = vec![3, 2, 1];
        let result = selection_sort(&list);
        assert_eq!(result, [1, 2, 3]);
    }

    #[test]
    fn test_2() {
        let list = vec![3, 1, 2, 4];
        let result = selection_sort(&list);
        assert_eq!(result, [1, 2, 3, 4]);
    }

    #[test]
    fn test_3() {
        let list = vec![-3, 2, 1];
        let result = selection_sort(&list);
        assert_eq!(result, [-3, 1, 2]);
    }

    #[test]
    fn test_4() {
        let list = vec![666];
        let result = selection_sort(&list);
        assert_eq!(result, [666]);
    }

    #[test]
    fn test_5() {
        let list = vec![];
        let result = selection_sort(&list);
        assert_eq!(result, []);
    }
}

pub fn quicksort<T: Ord + Copy>(arr: &Vec<T>) -> Vec<T> {
    if arr.len() < 2 {
        return arr.clone();
    }

    let pivot = arr[0];
    let less: Vec<T> = arr.iter().filter(|&x| x < &pivot).copied().collect();
    let greater: Vec<T> = arr.iter().filter(|&x| x > &pivot).copied().collect();

    [
        quicksort(less.as_ref()),
        vec![pivot],
        quicksort(greater.as_ref()),
    ]
    .concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let list = vec![3, 2, 1];
        let result = quicksort(&list);
        assert_eq!(result, [1, 2, 3]);
    }

    #[test]
    fn test_2() {
        let list = vec![3, 1, 2, 4];
        let result = quicksort(&list);
        assert_eq!(result, [1, 2, 3, 4]);
    }

    #[test]
    fn test_3() {
        let list = vec![-3, 2, 1];
        let result = quicksort(&list);
        assert_eq!(result, [-3, 1, 2]);
    }

    #[test]
    fn test_4() {
        let list = vec![666];
        let result = quicksort(&list);
        assert_eq!(result, [666]);
    }

    #[test]
    fn test_5() {
        let list: Vec<usize> = Vec::new();
        let result = quicksort(&list);
        assert_eq!(result, []);
    }

    #[test]
    fn test_6() {
        let list: Vec<usize> = (1..1000).rev().collect();
        let result = quicksort(&list);
        assert_eq!(result, (1..10000).collect::<Vec<usize>>());
    }
}

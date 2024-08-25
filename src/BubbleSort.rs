/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-08-25 10:42:39
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-08-25 11:40:02
 * @FilePath: /rust-algorithms/src/BubbleSort.rs
 * @Description: 冒泡排序算法
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */

pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() < 2 {
        return;
    }

    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        bubble_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        bubble_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        )
    }
}

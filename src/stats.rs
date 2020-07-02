// Module with basic statistics of numeric lists
use compare::{natural, Compare};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;

pub fn mode(list: &[i32], is_sorted: bool) -> i32 {
    // Calculates the most frequent element in the numeric list
    // Sorted lists are processed faster
    match is_sorted {
        true => internal_sorted_mode(&list),
        _ => internal_unsorted_mode(&list),
    }
}

pub fn mean(list: &[i32]) -> f32 {
    // Calculates mean of the numeric list
    list.iter().sum::<i32>() as f32 / list.len() as f32
}

pub fn median(list: &mut [i32]) -> f32 {
    // Calculates median of the numeric list
    list.sort();
    let len_mod_2: bool = list.len() % 2 == 0;
    match len_mod_2 {
        false => {
            let mid = (list.len() - 1) / 2;
            list[mid] as f32
        }
        true => {
            let mid_0 = list.len() / 2 - 1;
            let mid_1 = mid_0 + 1;
            (list[mid_0] as f32 + list[mid_1] as f32) / 2.0
        }
    }
}

fn internal_sorted_mode(list: &[i32]) -> i32 {
    // Calculates the most frequent item in sorted list
    let mut max_appearences: i32 = 0;
    let mut popular_elem = list[0];
    let mut count_appearences: i32 = 0;
    for (i, &elem) in list.iter().enumerate() {
        match i {
            0 => {
                count_appearences += 1;
                continue;
            }
            _ => (),
        }

        let cmp = natural();
        match cmp.compare(&elem, &list[i - 1]) {
            Less => panic!("Elements in list should be sorted"),
            Equal => count_appearences += 1,
            Greater => count_appearences = 0,
        }

        if count_appearences > max_appearences {
            max_appearences = count_appearences;
            popular_elem = elem;
        }
    }
    popular_elem
}

fn internal_unsorted_mode(list: &[i32]) -> i32 {
    // Calculates the most frequent item in unsorted list
    let mut frequencies = HashMap::new();
    for elem in list.iter() {
        let count = frequencies.entry(elem).or_insert(0);
        *count += 1;
    }

    let mut max_appearences = 0;
    let mut popular_elem = list[0];
    for (&elem, &count) in frequencies.iter() {
        if count > max_appearences {
            max_appearences = count;
            popular_elem = *elem;
        }
    }
    popular_elem
}

#[cfg(test)]
mod stats_tests {
    use super::*;

    #[test]
    fn mean_3_elements() {
        assert_eq!(mean(&[1, 3, 8]), 4.0);
    }

    #[test]
    fn median_3_elements() {
        assert_eq!(median(&mut [1, 3, 8]), 3.0);
    }

    #[test]
    fn median_2_elements() {
        assert_eq!(median(&mut [1, 4]), 2.5);
    }

    #[test]
    #[should_panic(expected = "Elements in list should be sorted")]
    fn mode_sorted_panic() {
        assert_eq!(mode(&[1, 4, 4, 3, 3, 4], true), 4);
    }

    #[test]
    fn mode_sorted() {
        assert_eq!(mode(&[1, 3, 3, 3, 4, 4], true), 3);
    }

    #[test]
    fn mode_unsorted() {
        assert_eq!(mode(&[1, 4, 4, 3, 3, 4], false), 4);
    }
}

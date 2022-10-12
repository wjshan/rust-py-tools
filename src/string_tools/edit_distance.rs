use std::cmp;

use pyo3::prelude::*;
use pyo3::pyfunction;
use pyo3::exceptions::*;

pub fn edit_distance(word1: &String, word2: &String) -> usize {
    /*!
     # 计算字符串之间的编辑距离
     ## Examples:
      ```
      let range = edit_distance(String::from("abcd"), String::from("avdd"))
      ```
     assert_eq!(edit_distance(String::from("abcd"), String::from("avdd")),2)
    */
    let word1_list: Vec<char> = word1.chars().collect();
    let word2_list: Vec<char> = word2.chars().collect();
    let m = word1_list.len();
    let n = word2_list.len();
    if m * n == 0 {
        return m + n;
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];

    // 初始化第0列
    for i in 0..n + 1 {
        dp[0][i] = i;
    }
    // 初始化第0行
    for i in 0..m + 1 {
        dp[i][0] = i
    }
    for i in 1..m + 1 {
        for j in 1..n + 1 {
            let left = dp[i][j - 1] + 1;
            let up = dp[i - 1][j] + 1;
            let mut up_left = dp[i - 1][j - 1];
            if word1_list[i - 1] != word2_list[j - 1] {
                up_left += 1;
            }
            dp[i][j] = cmp::min(left, cmp::min(up, up_left));
        }
    }

    dp[m][n]
}

#[pyclass]
pub struct MetchResult {
    #[pyo3(get, set)]
    first: String,
    #[pyo3(get, set)]
    second: String,
    #[pyo3(get, set)]
    ratio: f64,
}
#[pymethods]
impl MetchResult {
    #[new]
    fn new(first: String, second: String, ratio: f64) -> Self {
        MetchResult {
            first,
            second,
            ratio,
        }
    }
}

fn string_similarity(word1: &String, word2: &String) -> f64 {
    let range = edit_distance(word1, word2) as f64;
    let m = word1.chars().count();
    let n = word2.chars().count();
    let max_length = cmp::max(n, m) as f64;
    if max_length == 0.0 {
        return 1.0;
    }
    return 1.0 - range / max_length;
}

pub fn fuzzy_match(
    first_workds: Vec<String>,
    second_words: Vec<String>,
    cutoff: f64,
) -> Vec<MetchResult> {
    let mut result: Vec<MetchResult> = Vec::new();
    for first in &first_workds {
        for second in &second_words {
            let ratio = string_similarity(&first, &second);
            if ratio < cutoff {
                continue;
            }
            result.push(MetchResult::new(
                first.to_string(),
                second.to_string(),
                ratio,
            ));
        }
    }
    result
}

#[pyfunction[name="edit_distance"]]
pub fn edit_distance_py(word1: String, word2: String) -> PyResult<usize> {
    Ok(edit_distance(&word1, &word2))
}

#[pyfunction[name="string_similarity"]]
pub fn string_similarity_py(word1: String, word2: String) -> PyResult<f64> {
    Ok(string_similarity(&word1, &word2))
}

#[pyfunction[name="fuzzy_match"]]
pub fn fuzzy_match_py(first_workds: Vec<String>, second_words: Vec<String>, cutoff: f64)->PyResult<Vec<MetchResult>> {
    if cutoff < 0.0 || cutoff > 1.0 {
        let error_message = format!("0<=cusoff<=1, current is {}", cutoff);
        Err(PyValueError::new_err(error_message))
    } else {
        Ok(fuzzy_match(first_workds, second_words, cutoff))
    }
}

pub fn register(module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(edit_distance_py, module)?)?;
    module.add_function(wrap_pyfunction!(string_similarity_py, module)?)?;
    module.add_function(wrap_pyfunction!(fuzzy_match_py, module)?)?;
    Ok(())
}


#[cfg(test)]
mod edit_distance_tests {
    #[test]
    fn test0() {
        let s1 = String::from("柱钢筋绑扎");
        let s2 = String::from("划线、剪切");
        assert_eq!(super::edit_distance(&s1, &s2), 5);
    }
    #[test]
    fn test1() {
        let s1 = String::from("horse");
        let s2 = String::from("ros");
        assert_eq!(super::edit_distance(&s1, &s2), 3);
    }

    #[test]
    fn test2() {
        let s1 = String::from("intention");
        let s2 = String::from("execution");
        assert_eq!(super::edit_distance(&s1, &s2), 5);
    }

    #[test]
    fn test3() {
        let s1 = String::from("");
        let s2 = String::from("");
        assert_eq!(super::edit_distance(&s1, &s2), 0);
    }

    #[test]
    fn test4() {
        let s1 = String::from("a");
        let s2 = String::from("b");
        assert_eq!(super::edit_distance(&s1, &s2), 1);
    }

    #[test]
    fn test5() {
        let s1 = String::from("pneumonoultramicroscopicsilicovolcanoconiosis");
        let s2 = String::from("ultramicroscopically");
        assert_eq!(super::edit_distance(&s1, &s2), 27);
    }
}

#[cfg(test)]
mod string_similarity_tests{
    #[test]
    fn test0() {
        let s1 = String::from("柱钢筋绑扎");
        let s2 = String::from("划线、剪切");
        assert_eq!(super::string_similarity(&s1, &s2), 0.0);
    }
    #[test]
    fn test1() {
        let s1 = String::from("horse");
        let s2 = String::from("ros");
        assert_eq!(super::string_similarity(&s1, &s2), 0.4);
    }

    #[test]
    fn test2() {
        let s1 = String::from("intention");
        let s2 = String::from("execution");
        assert_eq!(super::string_similarity(&s1, &s2), 4.0/9.0);
    }

    #[test]
    fn test3() {
        let s1 = String::from("");
        let s2 = String::from("");
        assert_eq!(super::string_similarity(&s1, &s2), 1.0);
    }

    #[test]
    fn test4() {
        let s1 = String::from("a");
        let s2 = String::from("b");
        assert_eq!(super::string_similarity(&s1, &s2), 0.0);
    }

    #[test]
    fn test5() {
        let s1 = String::from("pneumonoultramicroscopicsilicovolcanoconiosis");
        let s2 = String::from("ultramicroscopically");
        assert_eq!(super::string_similarity(&s1, &s2), 0.4);
    }
}

#[cfg(test)]
mod fuzzy_match_test{
    use std::collections::HashMap;

    #[test]
    fn test0(){
        let first_list = vec!["柱钢筋绑扎".to_string(),"horse".to_string(),"".to_string(),"a".to_string()];
        let second_list = vec!["划线、剪切".to_string(),"avc".to_string()];
        let std_result:HashMap<(String, String), f64> = HashMap::from([
            (("柱钢筋绑扎".to_string(),"划线、剪切".to_string()),0.0),
            (("柱钢筋绑扎".to_string(),"avc".to_string()),0.0),
            (("horse".to_string(),"划线、剪切".to_string()),0.0),
            (("horse".to_string(),"avc".to_string()),0.0),
            (("".to_string(),"划线、剪切".to_string()),0.0),
            (("".to_string(),"avc".to_string()),0.0),
            (("a".to_string(),"划线、剪切".to_string()),0.0),
            (("a".to_string(),"avc".to_string()),1.0-2.0/3.0),

        ]);
        let matched = super::fuzzy_match(first_list, second_list, 0.0);

        for ma in &matched{
            let fist = ma.first.to_string();
            let second = ma.second.to_string();
            let std_res = std_result.get(&(fist,second)).unwrap();
            assert_eq!(&ma.ratio,std_res)
        }


    }
}
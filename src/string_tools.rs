use std::cmp;

use pyo3::{exceptions::PyValueError, prelude::*};

///  编辑距离
fn edit_distance(word1: &String, word2: &String) -> usize {
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

/// 计算字符串相似度
fn string_similarity(word1: &String, word2: &String) -> f64 {
    let range = edit_distance(word1, word2) as f64;
    let m = word1.chars().count();
    let n = word2.chars().count();
    let max_length = cmp::max(n, m) as f64;
    if max_length == 0.0{
        return 1.0;
    }
    return 1.0 - range / max_length;
}
#[pyclass]
struct MetchResult {
    #[pyo3(get)]
    first: String,
    #[pyo3(get)]
    second: String,
    #[pyo3(get)]
    ratio: f64,
}
#[pymethods]
impl MetchResult {
    #[new]
    fn new(first:String,second:String,ratio:f64)->Self{
        MetchResult{
            first,second,ratio
        }
    }
}

/// 对比两个字符串列表,根据cutoff数值进行筛选
fn fuzzy_match(
    first_workds: Vec<String>,
    second_words: Vec<String>,
    cutoff: f64,
) -> Vec<MetchResult> {
    let mut result: Vec<MetchResult> = Vec::new();
    for first in &first_workds {
        for seconds in &second_words {
            let ratio = string_similarity(&first, &seconds);
            if ratio < cutoff {
                continue;
            }
            result.push(MetchResult {
                first: first.to_string(),
                second: seconds.to_string(),
                ratio: ratio,
            })
        }
    }
    result
}

#[pymodule]
pub fn string(_py: Python, m: &PyModule) -> PyResult<()> {
    /// 编辑距离
    #[pyfn(m)]
    #[pyo3[name="edit_distance"]]
    fn edit_distance_py(_py: Python, word1: String, word2: String) -> PyResult<usize> {
        Ok(edit_distance(&word1, &word2))
    }

    /// 计算字符串相似度
    #[pyfn(m)]
    #[pyo3[name="string_similarity"]]
    fn string_similarity_py(_py: Python, word1: String, word2: String) -> PyResult<f64> {
        Ok(string_similarity(&word1, &word2))
    }

    m.add_class::<MetchResult>()?;

    /// 批量模糊匹配
    #[pyfn(m)]
    #[pyo3[name="fuzzy_match"]]
    fn fuzzy_match_py(
        _py: Python,
        first_workds: Vec<String>,
        second_words: Vec<String>,
        cutoff: f64,
    ) -> PyResult<Vec<MetchResult>> {
        if cutoff < 0.0 || cutoff > 1.0 {
            let error_message = format!("0<=cusoff<=1, current is {}", cutoff);
            Err(PyValueError::new_err(error_message))
        } else {
            Ok(fuzzy_match(first_workds, second_words, cutoff))
        }
    }

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
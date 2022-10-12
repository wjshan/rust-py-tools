use pyo3::prelude::*;
pub fn num_distinct(s: String, t: String) -> usize {
    /*!
     计算在`s`序列中,子序列`t`出现的次数
     
     即通过删除(也可以不删除)一些字符,使得序列`s`等于序列`t`

     $dp[i][j] = \begin{cases}
    dp[i-1][j-1]+dp[i][j-1] & t[i-1] = s[j-1] \\
    dp[i][j-1] & t[i-1] \neq s[j-1]
    \end{cases}$

    ## Example:

    - assert_eq!(num_distinct(String::from("rabbbit"), String::from("rabbit")), 3);
    
     */
    let s_words: Vec<char> = s.chars().collect();
    let t_words: Vec<char> = t.chars().collect();
    let m = s_words.len();
    let n = t_words.len();
    let mut top_dp = vec![1; m + 1];
    for i in 1..n + 1 {
        let mut current_dp = vec![0; m + 1];
        for j in 1..m + 1 {
            let t_w = t_words[i - 1];
            let s_w = s_words[j - 1];
            current_dp[j] = if t_w == s_w {
                top_dp[j - 1] + current_dp[j - 1]
            } else {
                current_dp[j - 1]
            }
        }
        top_dp = current_dp
    }
    top_dp[m]
}


#[pyfunction[name="num_distinct"]]
fn num_distinct_py(source: String, target: String) -> PyResult<usize> {
    /*!
     计算在`s`序列中,子序列`t`出现的次数
     
     即通过删除(也可以不删除)一些字符,使得序列`s`等于序列`t`

     $dp[i][j] = \begin{cases}
    dp[i-1][j-1]+dp[i][j-1] & t[i-1] = s[j-1] \\
    dp[i][j-1] & t[i-1] \neq s[j-1]
    \end{cases}$

    ## Example:

    - assert_eq!(num_distinct(String::from("rabbbit"), String::from("rabbit")), 3);
    
     */
    Ok(num_distinct(source, target))
}

pub fn register(module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(num_distinct_py, module)?)?;
    Ok(())
}

#[cfg(test)]
mod num_distinct_test {
    #[test]
    fn test0() {
        let res = super::num_distinct(String::from("rabbbit"), String::from("rabbit"));
        assert_eq!(res, 3);
    }

    #[test]
    fn test1() {
        let res = super::num_distinct(String::from("babgbag"), String::from("bag"));
        assert_eq!(res, 5);
    }
}

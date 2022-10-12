# Python tools compiled by rust

> pip install tust-py-tools-d

## string

### def edit_distance(word1:str,word2:str)->int
计算 `word1` 到 `word2`的编辑距离

$m=word1.length, n=word2.length$

时间复杂度: $O(m*n)$

空间复杂度: $O(m*n)$

### def string_similarity(word1:str,word2:str)->float:
计算字符串相似度
$1-range/max(word1.length,word2.length)$

$m=word1.length, n=word2.length$

时间复杂度: $O(m*n)$

空间复杂度: $O(m*n)$

### class MetchResult

Attributes:

- first(str): 字符1
- second(str): 字符2
- ratio:float: 相似度

### def fuzzy_match(first_workds:List[str],second_words:List[str],cutoff:float)->List[MetchResult]:

批量匹配两个列表的内字符的相似度

此方法会将将两个数组做一次笛卡尔乘积运算

时间复杂度: `m=first_workds.length`,`n=second_words.length` .$O(m*n*(a+b))$.a,b为列表包含字符串的平均长度

空间复杂度: $O(m*n)$


### def num_distinct(source:str,target:str)->int:

计算在`source`序列中,子序列`target`出现的次数

时间复杂度: $O(m*n)$ `m`为`source`的长度,`n`为`target`的长度

空间复杂度: $O(m)$ 我们需要两个长度为`m`的数组来记录状态值

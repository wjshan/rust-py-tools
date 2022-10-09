from typing import List


class MetchResult:
    fitst:str
    second:str
    ratio:float

def edit_distance(word1:str,word2:str)->int:
    """
    使用动态规划计算`word1`到`word2`之间的编辑距离
    即通过**增删改**这三种操作,将`word1`替换为`word2`所需的最少步骤

    时间复杂度: `m=word1.length`,`n=word2.length`, O(m+n)
    空间复杂度: O((m+1)*(n+1)) = O(m*n)

    Args:
        word1 (str): 原串
        word2 (str): 目标串

    Returns:
        int: 最少需要的步骤
    >>> edit_distance("horse", "ros")
    3
    >>> edit_distance("intention", "execution")
    5
    >>> edit_distance("", "")
    0
    >>> edit_distance("a", "b")
    1
    >>> edit_distance("pneumonoultramicroscopicsilicovolcanoconiosis", "ultramicroscopically")
    27
    """

def string_similarity(word1:str,word2:str)->float:
    """
    计算字符串相似度

    1-range/max(word1.length,word2.length)

    时间复杂度: `m=word1.length`,`n=word2.length`, O(m+n)
    空间复杂度: O((m+1)*(n+1)) = O(m*n)

    Args:
        word1 (str): _description_
        word2 (str): _description_

    Returns:
        float: 0<=ratio<=1
    >>> string_similarity("horse", "ros")
    0.4
    >>> string_similarity("intention", "execution")
    4/9
    >>> string_similarity("", "")
    1
    >>> string_similarity("a", "b")
    0
    >>> string_similarity("pneumonoultramicroscopicsilicovolcanoconiosis", "ultramicroscopically")
    0.4
    """

def fuzzy_match(first_workds:List[str],second_words:List[str],cutoff:float)->List[MetchResult]:
    """
    批量匹配两个列表的内字符的相似度
    此方法会将将两个数组做一次笛卡尔乘积运算

    时间复杂度: `m=first_workds.length`,`n=second_words.length` $O(m*n*(a+b))$.a,b为列表包含字符串的平均铲毒
    空间复杂度: O(m*n)
    
    Args:
        first_workds (List[str]): 列表串
        second_words (List[str]): 列表串
        cutoff (float): 匹配度过滤项,0<=cutoff<=1,如果需要全部的匹配度,可设置为0

    Returns:
        List[MetchResult]: 满足cutoff的匹配项

    >>> fuzzy_match(["柱钢筋绑扎","horse",])
    """
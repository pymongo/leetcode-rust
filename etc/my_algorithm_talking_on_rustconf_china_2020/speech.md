# Rust刷题经验分享演讲稿 - rustconf_china_2020

## SUMMARY

## 封面
- TODO 标题待定
- 姓名
- TODO 背景图

## 自我介绍

## 演讲稿

```
【封面】{PPT大标题是topic名字，第二行是我的名字，[可选]加上Rustconf2020的背景图}
大家好，我今天想和大家分享的topic是{TODO}。

【自我介绍】{
    标题=自我介绍，
    接着大图贴上我给Rust开源库提交的PR图，
    底部贴上leetcode和lintcode的全球排名图
}
首先，我先进行自我介绍。我叫吴翱翔，我曾参与给sqlx, tokio-postgres, bigdecimal, actix等Rust社区开源库贡献过代码，
leetcode全球排名1万名左右，刷题量有400+。~~我最近连续几个月都坚持用Rust做leetcode的每日一题，~~

【github仓库链接】{
    标题=我的Rust题解github仓库
    第二行链接
    第三行简介=leetcode,codeforces等算法题/OJ网站的题解，每个题解都有单元测试
}
这是我的Rust题解的github仓库，除了leetcode还有codeforces等知名算法题/OJ网站，每个题解不仅包含解题思路，还有相应的单元测试
接下来我愿意跟大家分享一下我用Rust做算法题的收获。

【我用Rust刷算法题中学到了什么】{
    【第一页】
    - 标准库API: Peekable, windows, checked_add...
    - 提升编程能力和解决问题的能力
    - 深入了解Vec,Box,Rc等智能指针
    - 读懂开源库一些底层算法，为提PR打基础
    - 更复杂的match表达式用法(例如match两个链表)
    【第二页】
    图: 我实现bigdecimal类型round API的源码
}
我最初看官方的the book, Rust by example, cookbook等书籍学习Rust，可是标准库内容很多，更新快，
官方书籍上并不能覆盖很多标准库的API用法。我从Rust刷算法题的过程中，学到了例如通过with_capacity提升内存分配的性能等等。
当我用Rust做完leetcode第二题，两个链表相加的一题，我就慢慢掌握了Box等智能指针的用法，还学到了match两个链表的写法。
在我刷题刷了一段时间后，就慢慢能看懂Rust的bigdecimal-rs等开源库底层的源码。
<PPT第二页到bigdecimal round的PR图片>
这是我为bigdecimal实现的round API，可以让decimal实现任意小数位的四舍五入精确运算。
round背后的算法灵感来源于我在leetcode上做过的多道题。

【1. 为什么要用Rust刷算法题】{
    1.1 刷题是Rust新手很好的练习/学习途径
    1.2 性能好: 运行时间容易跑进0ms，内存消耗低
    1.3 Rust强大的单元测试，支持assert_eq两个链表
    1.4 优雅的下标访问越界、数值溢出处理语句
    【1.2.1 leetcode_463. 岛屿周长性能对比】
    图: 岛屿周长一题各语言最快解法性能对比
    小字评论: Rust在数组类算题不仅是运行最快，而且内存消耗最低
    注: 除Rust以外的其它分别用各语言最快的解法记录去提交
    【1.2.2 leetcode_4. 寻找两个正序数组的中位数】
    - O(n*log(n))算法: 合并两个有序数组再找中位数，Rust运行时间4ms
    - O(n)算法: 两个数组进行归并排序的归并操作，Rust运行时间0ms
    - O(log(n))算法: 二分法
    用其它语言可能要用最快的算法才能跑进0ms，但是Rust用中等的归并算法就足以跑进0ms
    【1.2.1 】
    【1.3.1 】
    对比Rust在reverse_int一题上和官方解答-7,8魔法数数学公式推导的溢出判断
}
我刚入门Rust的时候，公司没有也不愿意在商业项目上使用Rust，我渴望通过项目来练手Rust。于是我就想到通过leetcode/codeforces等刷题网站去巩固Rust只是。
其实很多人谈起算法题就很恐惧，其实可以换个角度想，算法题无非就是公司商业项目中的一个小需求，而且是输入输出都固定非常明确易懂的需求，再加上一些性能上的要求。
<PPT到1.2.1 leetcode_463. 岛屿周长性能对比>
接下来我们看一组Rust和其它语言在leetcode463岛屿周长一题的性能对比。
可以看到Rust在数组类的题型里快得简直像leetcode亲儿子，但是在链表二叉树等复杂数据结构题型中，受限于所有权等机制，Rust的链表二叉树并不算快

【当前leetcode对Rust支持的不足】{
    - 所有N叉树(N-array)的题都不支持Rust
    - 所有多线程的题不支持Rust
    - 部分复杂链表/二叉树题不支持Rust(例如LRU)
    - leetcode社区上Rust的题解讨论太少
}


【结束标题: Thanks you】
```

## 我从leetcode Rust刷题中学到了什么

- two_sum: BTreeMap
- add_two_numbers: 学会let Some，match两个链表的写法，学习Box智能指针
- 某个每日一题，忘了: peekable Iterator
- reverse_int: checked_add的优雅判断溢出
- 很多题usize类型如何避免`0-1`的溢出
- 性能好

## 为什么要用Rust刷题

- 刚学习Rust没有公司项目练手，通过leetcode的题可以理解成就是一个个的业务需求
- 性能好(放一个图 rust4ms， 其它语言都是10ms+的图)，简直是leetcode亲儿子，leetcode特殊优化，
- 数组类题型性能目前是leetcode第一，二叉树/链表的题型性能略微吃亏
- 锻炼编程能力，学习更多Rust标准库的用法例如windows

## Rust工程化的体现 刷题特有的优势

0. dbg!超方便，但是Rust的Debug单步调试又不如Python等方便(容易跳入汇编代码)

1. 单元测试方便，(这里PPT画个图，代码提交出错，Rust加入新的测试用例，然后调试好代码再提交的循环)

2. assert_eq!支持链表、二叉树等复杂数据结构的比较，单元测试常用assert比较是否满足期待值
(实际上支持ParticalEq和Eq Trait的数据类型都能验证是否相等)

3. 链表、二叉树自动实现类似格式化打印，方便调试

\[演说稿]: 我以前用Python刷题时，因为自己不太会实现逐层递归打印链表/二叉树的方法，所以经常需要在leetcode上print程序运行到第几行时状态是多少

综上几点，Rust刷题容易做到完全脱离leetcode网页的IDE，自己本地IDE测试通过后，复制到网页上提交一般也能通过

> ListNode{val: 2, next: ListNode{val: 4, next: ListNode{val: 3, next: None}}}

TODO code force刷题的体验

简单介绍下从stdin读内容

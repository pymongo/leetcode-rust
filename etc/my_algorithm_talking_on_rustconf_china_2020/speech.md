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
    标题自我介绍，
    接着大图贴上我给Rust开源库提交的PR图，
    底部贴上leetcode和lintcode的全球排名图
}
首先，我先进行自我介绍。我叫吴翱翔，我曾参与给sqlx, tokio-postgres, bigdecimal, actix等Rust社区开源库贡献过代码，
leetcode全球排名1万名左右，刷题量有400+。~~我最近连续几个月都坚持用Rust做leetcode的每日一题，~~
我愿意跟大家分享一下我用Rust做算法题的收获。

【我用Rust刷算法题中学到了什么】{
    【第一页】
    - 标准库API: Peekable, windows, checked_add...
    - 提升编程能力和解决问题的能力
    - 通过链表二叉树入门Box,Rc等智能指针
    - 读懂开源库一些底层算法，为提PR打基础
    - ~~更复杂的match表达式用法~~
    【第二页】
    图: 我实现bigdecimal类型round API的源码
}
例如用with_capacity API去提高Vector内存分配的性能和效率

【1. 为什么要用Rust刷算法题】{
    1.1 学完Rust以后没商业项目可练，做题
    【】
}
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

## code force刷题的体验

简单介绍下从stdin读内容

## 当前leetcode对Rust支持的不足

- 所有N叉树的题都没有Rust
- 所有多线程的题没有Rust
- 部分高难度链表修改题没有Rust
- LRU没有Rust

# 演讲稿(PPT的逐页演说稿)

## 标题页

大家好，我今天想和大家分享的topic是「浅谈Rust在算法题和竞赛中的应用」。

## 自我介绍

首先，我先简单的自我介绍。我叫吴翱翔，我曾给rust-postgres, sqlx, bigdecimal-rs, actix等Rust社区开源库贡献过代码。
作为一名Rust的爱好者的同时，我还是一名业余的算法爱好者，我在leetcode/codeforce等知名算法题网站上刷题量有400+。

###我的Rust题解github仓库介绍

这是我的Rust题解的github仓库，除了leetcode还有codeforces等知名算法题/OJ网站，每个题解不仅包含解题思路，还有相应的单元测试
接下来我愿意跟大家分享一下我用Rust做算法题的收获，以及用Rust刷算法题的优势。

## 为什么要用Rust刷算法题 > 目录页

```
(PPT内容)
- 掌握更多算法，为读懂开源库源码提PR打基础
- 标准库API: peekable, windows, saturating_sub...
- 通过链表/二叉树题理解Box, Rc, RefCell等智能指针
- Rust性能优秀，leetcode耗时容易低于0ms/内存消耗低于2MB
- 刷题是Rust新手很好的练习/学习途径
- Rust工具链好用，单元测试强大(例如assert_eq两个链表)
- Rust代码简短，开发效率和可读性兼备
```

首先第一点，刷题可以读懂一些开源库的底层算法，我后面会展开讲我给BigDecimal提的round API的PR用到了哪些算法。
第二，用Rust刷题学到了很多冷门的标准库API，例如peekable，saturating_sub之类的API实际项目中我几乎没用过，
但是这些API在解决特定问题时非常有用，能扩展思维和知识储备。标准库的API很多，官方书籍上并不能覆盖很多标准库的API用法。
我从Rust刷算法题的过程中，学到了例如通过with_capacity提升内存分配的性能等等技巧和API。
第三，通过链表/二叉树的题理解一些智能指针，我当时学Rust的时候看了一些教程和视频，还不知道有Rc引用计数，
后来也是通过leetcode做题才慢慢接触和掌握Rc，Box等职能指针的用法。
第四，Rust的性能优秀，在leetcode上leetcode似乎像亲儿子一样快，目前是最快的，在codeforce上Rust性能和C语言差距不大。
我个人在刷题时追求能跑进0ms，因为能跑进0ms那必然是击败了100%的提交记录。
第五，刷题是学Rust的好途径，我刚入门Rust的时候，想找项目练手，但是公司也不希望在商业项目上用Rust，
所以我就把做题想象成做项目的一个小需求，慢慢熟悉和掌握Rust。
第六，Rust工具链好用，单元测试强大，我记得在Java的测试框架JUnit或Python的unittest光assert都有十几种方法，例如assertListEqual，
就是不同类型可能要用不用的assert方法，而且默认也不支持链表的比较。但是在Rust里由于统一性很强的类型系统，就用一个assert就能解决不同类型自身的Equal比较
第七，像Option这样的抽象在reverse_integer一题中让代码的可读性更强

### 为什么要用Rust刷算法题 > 我为Bigdecimal实现round...

这是我为bigdecimal实现的round API，可以让decimal实现任意小数位的四舍五入精确运算。
round背后的算法灵感来源于我在leetcode上做过的多道题。
我可以简单跟大家讲下这个四舍五入的算法，BigDecimal这个类型主要由BigInt存储数值部分以及一个i64存储浮点的位置。
比如说1.23就是由数值部分123和记录小数点位置的2组成，2表示123从右往左第二位数字2前面插入一个小数点，那么1.23.round(1)会等于1.2。
round的过程首先要找到需要round的那一位，round(1)也就是1.23中的2这一位。
那么取得十进制数每位的信息那就有点像leetcode上reverse_integer这题的算法。
round往后的过程需要判断正负数等略微麻烦的过程，由于时间关系我就不介绍了。

### 为什么要用Rust刷算法题 > Rust性能...(岛屿周长一题)

接下来我们看下Rust和其它语言在leetcode_463.岛屿周长一题上的性能对比， 图中的每种语言都是复制相应的最快的解法记录去提交。
即便在链表二叉树等复杂数据结构题型，受限于所有权等机制，Rust性能有所降低，但依然是leetcode第一梯队的性能

### 为什么要用Rust刷算法题 > Rust性能...(寻找两个有序数组的中位数一题)

让我们接下来再看看leetcode_4.寻找两个有序数组的中位数这题，这道题根据时间复杂度有三种档次的算法，
其中最快的解法是二分法，但是Rust并不需要用二分法就能跑进0ms以内。
说实话这题二分法的写法很啰嗦，要考虑多种特殊情况，没有归并算法容易理解和背诵。

### 为什么要用Rust刷算法题 > Rust简洁和可读性的体现(反转整数一题)

我们再看看leetcode_7.反转整数这题，看看上面这个官方解法中，用到了像7和-8这样的魔法数降低了可读性，
实际上这个7和-8是通过一系列数学公式推导推出了的溢出标志位。对于i32类型来说，反转的过程中可能会出现溢出，
leetcode这题要求溢出时返回0即可，实际上我认为这个返回值并不好，例如溢出时返回0和-1这样的标志位还不如返回None。
Rust可以返回一个Option，None表示输入的整数不能被反转或者说反转后溢出，Some表示可以正常反转，这样表示语义和可读性就好很多，
而且Rust的checked_开头的API可以检查溢出，配合上问号语法糖，一旦溢出就early return提前返回，代码简洁和可读性兼备。

### 为什么要用Rust刷算法题 > Rust简洁和可读性的体现(四题)

接下来继续看四个体现Rust可读性好的例子，reverse_bits这题Rust的API名字跟题目名称一样，
valid_number判断一个字符串是不是整数，用parse().is_ok()非常简洁，
然后就是分辨IP地址是v4还是v6这题，用match语句模式匹配，最后一个例子体现了Rust函数式的魅力，
首先构造一个从start到无穷大的构造器，然后设置步长和需要从迭代器中取几次，最后通过fold也就是其它语言的reduce API对迭代器进行两异或求值

## 题目分析 number_of_good_pairs > 题目说明页



### 继续优化 number_of_good_pairs

## 结束页

我的分享到此结束，谢谢大家！(鞠躬)

---

## PPT的代码块

我一般都是从vscode/IDEA粘贴富文本格式的代码到文本框中，

ms_powerpoint粘贴后，vscode没了缩进，IDEA的可能字体忽黑忽白，WPS又粘贴不了富文本。

事实上我的PPT又不做动画，keynote简洁的页面真的不说，而且对富文本支持超棒。

考虑到本次大会的PPT模板的背景色偏蓝灰色，要用暖色调的字体颜色才能对比感更强烈，IDEA默认theme可以，

但是IDEA对Rust代码的高亮不如vscode丰富，所以我最终选用vscode的monokai主题粘贴代码到keynote中

不用IDEA还有一个原因是，jetbrains字体mac系统没有内置，而vscode和xcode默认的menlo字体mac和keynote上都备有

### 我为什么不用carbon工具生成代码的svg/png

首先png缩放后容易看不清代码，而且图片灵活性不好。如果PPT支持svg格式就更好。

代码块用文本的好处是，可以灵活调整字体大小，粗细，行间距等等，方便PPT页面布局。

```
- 更复杂的match表达式用法(例如match两个链表)
【1. 为什么要用Rust刷算法题】{
1.3 优雅的下标访问越界、数值溢出处理语句

```

```
【当前leetcode对Rust支持的不足】{
    - 所有N叉树(N-array)的题都不支持Rust
    - 需要删除或挪动链表中间几个节点的题(例如leetcode 237Delete Node in a Linked List)
    - 必须调用clone或take才能非递归地遍历二叉树
    - 所有多线程的题不支持Rust
    - 部分复杂链表/二叉树题不支持Rust
    - leetcode社区上Rust的题解讨论太少
}
当然，当前的leetcode对Rust的支持也不是很完整，而且Rust解题的学习资料并不多，不过我相信这些问题以后会慢慢得到改善

【Rust题目的经验分享】{
    字符串的入参s，99%情况下都可以直接s.into_bytes()提升性能和方便后续操作
}
```

---
## (老的/废弃的PPT演说稿)

### 我从leetcode Rust刷题中学到了什么

- 很多题usize类型如何避免`0-1`的溢出

### 为什么要用Rust刷题

- 性能好(放一个图 rust4ms， 其它语言都是10ms+的图)，简直是leetcode亲儿子，leetcode特殊优化，
- 数组类题型性能目前是leetcode第一，二叉树/链表的题型性能略微吃亏

## Rust工程化的体现 刷题特有的优势

0. dbg!超方便，但是Rust的Debug单步调试又不如Python等方便(容易跳入汇编代码)

1. 单元测试方便，(这里PPT画个图，代码提交出错，Rust加入新的测试用例，然后调试好代码再提交的循环)

2. assert_eq!支持链表、二叉树等复杂数据结构的比较，单元测试常用assert比较是否满足期待值
(实际上支持ParticalEq和Eq Trait的数据类型都能验证是否相等)

3. 链表、二叉树自动实现类似格式化打印，方便调试

\[演说稿]: 我以前用Python刷题时，因为自己不太会实现逐层递归打印链表/二叉树的方法，所以经常需要在leetcode上print程序运行到第几行时状态是多少

综上几点，Rust刷题容易做到完全脱离leetcode网页的IDE，自己本地IDE测试通过后，复制到网页上提交一般也能通过

> ListNode{val: 2, next: ListNode{val: 4, next: ListNode{val: 3, next: None}}}

TODO codeforce刷题的体验

LeetCode
========

| # | Title | Solution | Category |
|---| ----- | -------- | ---------- |
|1|[Two Sum](https://leetcode.com/problems/two-sum/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/collections/btree_map_two_sum.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/collections/two_sum_test.go), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/test/java/com/leetcode/collections/HashMapTwoSum.java)|btree_map, bitwise, two_s_complement|
|2|[Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/collections/traverse_two_list_node.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/collections/traverse_two_list_node_test.go), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/test/java/com/leetcode/collections/TraverseTwoListNode.java)|no_0ms_solution, create/traverse_list_node|
|3|[Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/string/longest_non_repeated_substr.rs)|sliding_window|
|4|[Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/median_of_two_sorted_arrays.rs)|binary_search|
|5|[Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/string/longest_palindromic_substr.rs)|more_than_5_solutions, manacher, suffix_array|
|6_TODO|[ZigZag Conversion](https://leetcode.com/problems/zigzag-conversion/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/src/string/zigzag_conversion.rs)||
|9|[Palindromic Number](https://leetcode-cn.com/problems/palindrome-number/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/string/i32_is_palindromic.rs)|half_traverse_i32,dfa|
|28|[Implement strStr()](https://leetcode.com/problems/implement-strstr/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/string/contains_substr_kmp.rs)|kmp, dfa, multi-solutions|
|62|[Unique Paths](https://leetcode.com/problems/unique-paths/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/permutation/shortest_paths_on_checkerboard.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/permutation/shortest_paths_on_checkerboard_test.go)|permutation|
|70|[Climb Stairs](https://leetcode.com/problems/climbing-stairs/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/classic/tail_recursion_fibonacci.rs)|tail_recursion, fibonacci|
|88|[Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/merge_two_sorted_arrays.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/sorting/merge_two_sorted_arrays_test.go)||
|292|[Nim Game](https://leetcode.com/problems/nim-game/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/bitwise/nim_game.rs)||
|709|[To Lower Case](https://leetcode.com/problems/to-lower-case/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/impl_api/to_lowercase.rs)||
|743_TODO|[Network Delay Time](https://leetcode.com/problems/network-delay-time/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/graph_traversal/dijkstra_shortest_path_algorithm.rs)|dijkstra_shortest_path|
|912|[Sort An Array](https://leetcode.com/problems/sort-an-array/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/quick_sort.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/basic_sorting.py)|basic_sorting|

---

LintCode
========

| # | Title | Solution | Category |
|---| ----- | -------- | ---------- |
|254|[Drop Eggs](https://www.lintcode.com/problem/drop-eggs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/drop_eggs.rs)|sqrt_n|

---

- [rust leetcode](https://github.com/pymongo/rust_leetcode)
- [go_leetcode](https://github.com/pymongo/go_leetcode)
- [java_leetcode](https://github.com/pymongo/java_leetcode)
- [python_leetcode](https://github.com/pymongo/python_leetcode)

---

为什么不用C++?为什么选用以上Rust、Go、Java语言刷leetcode？

C++没有自带的项目构建工具、单元测试框架，如果想随意运行一个解法或修改自己旧的解法，就要改cmake文件，使项目变得难以维护。

Rust性能与C++相当，大部分题都能和C++一样跑进0ms。

而且Rust有类似npm但比npm更好用更工程化的cargo工具，cargo运行单元测试/性能测试很简单，无论rs文件在哪，只要方法名上加#\[test]就能TDD(测试驱动开发)。

我习惯通过单步调试+纸笔推演去读懂别人的优秀算法，但是Rust的单步调试经常跳到汇编代码中，按半天F7/F8才能跳出来，所以需要先用Go语言去理解我不懂的算法。

Java的优点不用我多说，借助maven+junit让我轻松地通过TDD的方式刷题，因为Java的语法类似C++，所有用Java也算了解用C++刷leetcode的心愿。

使用Java的另一个原因是，在某些题(如two sum)上Java(1ms)的性能比Go(4ms)还好。

而且很多算法资料都是Java语言的，leetcode上所有官方的题解都是以Java为主。

后来我补上了某些题的Python题解，其实我有点讨厌python运行速度太慢，很多题用一般的解法都会超时

由于牛客网、lintcode等平台支持python不支持Rust，而且python代码量少便于快速刷题

为了在牛客网上用python远程面试编程题时能游刃有余，还是先用python将自己的题量刷到200+先

而且Python的单元测试也简单，还支持typehint，更重要的是国内的面试官大部分都懂python代码

我的刷题量太少了，先刷个100+再说

LeetCode
========

- [rust leetcode](https://github.com/pymongo/rust_leetcode)
- [go_leetcode](https://github.com/pymongo/go_leetcode)
- [java_leetcode](https://github.com/pymongo/java_leetcode)

| # | Title | Solution | Category |
|---| ----- | -------- | ---------- |
|1|[Two Sum](https://leetcode.com/problems/two-sum/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/collections/tree_map_two_sum.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/collections/two_sum_test.go), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/test/java/com/leetcode/collections/HashMapTwoSum.java)|TreeMap, two's complement|
|2|[Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/collections/traverse_two_list_node.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/collections/traverse_two_list_node_test.go), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/test/java/com/leetcode/collections/TraverseTwoListNode.java)|create/traverse ListNode|
|3|[Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/merge_two_sorted_arrays.rs)|sliding_window|
|4|[Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/median_of_two_sorted_arrays.rs)|binary_search|
|62|[Unique Paths](https://leetcode.com/problems/unique-paths/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/permutation/shortest_paths_on_checkerboard.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/permutation/shortest_paths_on_checkerboard_test.go)|permutation|
|88|[Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/merge_two_sorted_arrays.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/sorting/merge_two_sorted_arrays_test.go)||
|709|[To Lower Case](https://leetcode.com/problems/to-lower-case/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/impl_api/to_lowercase.rs)||
|743_TODO|[Network Delay Time](https://leetcode.com/problems/network-delay-time/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/graph_traversal/dijkstra_shortest_path_algorithm.rs)|dijkstra_shortest_path|

---

为什么不用C++?为什么选用以上三种语言刷leetcode？

C++没有自带的项目构建工具、单元测试框架，如果想随意运行一个解法或修改自己旧的解法，就要改cmake文件，使项目变得难以维护。

Rust性能与C++相当，大部分题都能和C++一样跑进0ms。

而且Rust有类似npm但比npm更好用更工程化的cargo工具，cargo运行单元测试/性能测试很简单，无论rs文件在哪，只要方法名上加#\[test]就能TDD(测试驱动开发)。

我习惯通过单步调试+纸笔推演去读懂别人的优秀算法，但是Rust的单步调试经常跳到汇编代码中，按半天F7/F8才能跳出来，所以需要先用Go语言去理解我不懂的算法。

Java的优点不用我多说，借助maven+junit让我轻松地通过TDD的方式刷题，因为Java的语法类似C++，所有用Java也算了解用C++刷leetcode的心愿。

使用Java的另一个原因是，在某些题(如two sum)上Java(1ms)的性能比Go(4ms)还好。

而且很多算法资料都是Java语言的，leetcode上所有官方的题解都是以Java为主。

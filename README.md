LeetCode
========

因为leetcode-cn上我多做了2道面试题/剑指OFFER题(其实是重复的题目)，所以我leetcode国服账户会比全球服账户多AC了4题

| # | Title | Solutions | Category |
|---| ----- | -------- | ---------- |
|1|[Two Sum](https://leetcode.com/problems/two-sum/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/collections/btree_map_two_sum.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/collections/two_sum_test.go), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/test/java/com/leetcode/collections/HashMapTwoSum.java)|btree_map, bitwise(two_s_complement)|
|2|[Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/collections/traverse_two_list_node.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/collections/traverse_two_list_node_test.go), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/test/java/com/leetcode/collections/TraverseTwoListNode.java), [Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/add_two_numbers.py)|create/traverse_list_node|
|3|[Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/string/longest_non_repeated_substr.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/string/longest_non_repeated_substr.py)|sliding_window|
|4|[Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/median_of_two_sorted_arrays.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/median_of_two_sorted_arrays.py)|binary_search|
|5|[Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/string/longest_palindromic_substr.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_substr.py)|multi_solutions, manacher, suffix_array|
|6|[ZigZag Conversion](https://leetcode.com/problems/zigzag-conversion/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/string/zigzag_conversion.py)||
|7|[Reverse Integer](https://leetcode.com/problems/reverse-integer/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/reverse_integer.py)||
|9|[Palindromic Number](https://leetcode.com/problems/palindrome-number/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/string/i32_is_palindromic.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_substr.py)|half_traverse_i32|
|15|[3Sum](https://leetcode.com/problems/3sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/three_sum.py)|two_pointers, two_sum|
|17|[Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/phone_9_keypad_combination.py)|product(笛卡尔积)|
|18|[4Sum](https://www.lintcode.com/problem/4sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/four_sum.py)|two_pointers, two_sum|
|26|[Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/move_zeros.py)||
|27|[Subarray Sum](https://www.lintcode.com/problem/remove-element/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/move_zeros.py)||
|28|[Implement strStr()](https://leetcode.com/problems/implement-strstr/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/string/contains_substr_kmp.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/string/find_substr.py)|kmp, dfa, multi_solutions|
|33|[Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search.py)|binary_search|
|34|[Find First and Last Position of Element in Sorted Array](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search_first_and_last.py)|binary_search|
|39|[Combination Sum](https://leetcode.com/problems/combination-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/combination_target_sum.py)||
|46|[Permutations](https://leetcode.com/problems/permutations/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/permutation.py)|backtracking|
|46|[Permutations II](https://leetcode.com/problems/permutations-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/permutation.py)|backtracking|
|50|[Pow(x, n)](https://leetcode.com/problems/powx-n/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/pow.py)|binary_search, 快速幂运算|
|53|[Maximum Subarray](https://leetcode.com/problems/maximum-subarray/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/maximum_subarray.py)|greedy, dp|
|62|[Unique Paths](https://leetcode.com/problems/unique-paths/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/permutation/shortest_paths_on_checkerboard.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/permutation/shortest_paths_on_checkerboard_test.go)|permutation|
|69|[Sqrt(x)](https://leetcode.com/problems/sqrtx/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/sqrt.py)|牛顿连续均值求根法|
|70|[Climb Stairs](https://leetcode.com/problems/climbing-stairs/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/classic/tail_recursion_fibonacci.rs)|tail_recursion, fibonacci|
|75|[Sort Colors](https://leetcode.com/problems/sort-colors/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/sort_colors.py)|three_pointers, partition_array|
|78|[Subsets](https://leetcode.com/problems/subsets/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/subsets.py)||
|79|[Word Search](https://leetcode.com/problems/word-search/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/word_search.py)||
|81|[Search in Rotated Sorted Array II](https://leetcode.com/problems/search-in-rotated-sorted-array-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search_2_with_duplicate.py)|binary_search|
|88|[Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/merge_two_sorted_arrays.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/sorting/merge_two_sorted_arrays_test.go), [Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/merge_two_sorted_arrays.py)|merge_sort|
|90|[Subsets](https://leetcode.com/problems/subsets-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/subsets.py)||
|94|[Binary Tree Inorder Traversal](https://www.leetcode.com/problems/binary-tree-inorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|102|[Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_level_order.py)|BFS|
|104|[Maximum Depth of Binary Tree](https://www.leetcode.com/problems/maximum-depth-of-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_max_min.py)||
|105|[Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_pre_order_and_in_order.py)|DFS, stack|
|106|[Construct Binary Tree from Inorder and Postorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_in_order_and_post_order.py)|DFS|
|107|[Binary Tree Level Order Traversal II](https://leetcode.com/problems/binary-tree-level-order-traversal-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_level_order.py)|BFS|
|110|[Balanced Binary Tree](https://www.leetcode.com/problems/balanced-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_is_balance_binary_tree.py)|divide_and_conquer|
|111|[Minimum Depth of Binary Tree](https://www.leetcode.com/problems/minimum-depth-of-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_max_min.py)|divide_and_conquer, BFS|
|114|[Flatten Binary Tree to Linked List](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/in_place_flatten_binary_tree_to_linked_list.py)||
|125|[Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome.py)|two_pointers|
|126|[Word Ladder II](https://leetcode.com/problems/word-ladder-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/word_ladder_2.py)|BFS+DFS|
|127|[Word Ladder](https://leetcode.com/problems/word-ladder/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/word_ladder.py)|双向BFS|
|133|[Clone Graph](https://leetcode.com/problems/clone-graphs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/clone_graph.py)|DFS, BFS|
|134|[Gas Station](https://leetcode.com/problems/gas-station/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/gas_station.py)|greedy|
|136|[Single Number](https://leetcode.com/problems/single-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/xor_find_single.py)||
|144|[Binary Tree Preorder Traversal](https://www.leetcode.com/problems/binary-tree-preorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|145|[Binary Tree Postorder Traversal](https://www.leetcode.com/problems/binary-tree-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|145|[LRU Cache](https://leetcode.com/problems/lru-cache/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/lru_cache.py)|double_linked_list, OrderedDict|
|151|[Reverse Words in a String](https://leetcode.com/problems/reverse-words-in-a-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/reverse_words_in_a_string.py)||
|153|[Find Minimum in Rotated Sorted Array](https://www.leetcode.com/problems/find-minimum-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search.py)|binary_search|
|154|[Find Minimum in Rotated Sorted Array II](https://www.leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_min_2_with_duplicate.py)|binary_search|
|167|[Two Sum II - Input array is sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/two_sum_2_input_is_sorted.py)|two_pointers|
|170_LOCK|[Two Sum III - Data structure design](https://leetcode.com/problems/two-sum-iii-data-structure-design/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/two_sum_3_impl.py)|two_pointers|
|172|[Factorial Trailing Zeroes](https://leetcode.com/problems/factorial-trailing-zeroes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/factorial_trailing_zeroes.py)|factorial|
|173|[Binary Search Tree Iterator](https://leetcode.com/problems/binary-search-tree-iterator/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_in_order_iterator.py)||
|189|[Rotate Array](http://leetcode.com/problems/rotate-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/rotate_array_right_circle_shift_elements.py)|reverse, circle_shift|
|200|[Number of Islands](https://www.leetcode.com/problems/number-of-islands/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/number_of_islands.py)|union_find, DFS, BFS|
|206|[Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/reverse_linked_list.py)|list_node|
|208|[Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/prefix_tree_tire/impl_tire_prefix_tree.py)|前缀树/字典树|
|210|[Course Schedule II](https://leetcode.com/problems/course-schedule-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/course_schedule_2.py)|BFS, topological_sorting|
|212|[Word Search II](https://leetcode.com/problems/word-search-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/word_search_2.py)|前缀树|
|215|[Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)|quick_select, quick_sort|
|225|[Implement Stack using Queues](https://leetcode.com/problems/implement-stack-using-queues/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/impl_collections/impl_stack_using_queue.py)||
|230|[Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|231|[Power of Two](https://leetcode.com/problems/power-of-two/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_2.py)|bitwise, dichotomy|
|232|[Implement Queue using Stacks](https://leetcode.com/problems/implement-queue-using-stacks/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/impl_collections/impl_stack_using_queue.py)||
|235|[Lowest Common Ancestor of a Binary Search Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_lowest_common_ancestor.py)||
|236|[Lowest Common Ancestor of a Binary Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)|divide_and_conquer|
|257|[Binary Tree Paths](https://leetcode.com/problems/binary-tree-paths/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/root_to_leaf_paths_find_all.py)|DFS, backtracking|
|263|[Ugly Number](https://www.leetcode.com/problems/ugly-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/ugly_number.py)||
|264|[Ugly Number II](https://www.leetcode.com/problems/ugly-number-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/ugly_number_2_nth_ugly.py)||
|266_LOCK|[Palindrome Permutation](https://leetcode.com/problems/palindrome-permutation/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/palindrome_permutation.py)|greedy|
|269_LOCK|[Alien Dictionary](https://leetcode.com/problems/alien-dictionary/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/alien_dictionary.py)|heapq, topological_sorting|
|270_LOCK|[Closest Binary Search Tree Value](https://www.leetcode.com/problems/closest-binary-search-tree-value/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value.py)||
|272_LOCK|[Closest Binary Search Tree Value II](https://www.leetcode.com/problems/closest-binary-search-tree-value-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value_2.py)||
|278|[First Bad Version](https://leetcode.com/problems/first-bad-version/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/first_bad_version.py)|partition_array|
|283|[Move Zeros](https://leetcode.com/problems/move-zeroes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/move_zeros.py)|partition_array, 快慢双指针|
|292|[Nim Game](https://leetcode.com/problems/nim-game/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/bitwise/nim_game.rs)||
|297|[Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree.py)|serialize|
|326|[Power of Three](https://leetcode.com/problems/power-of-three/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_3.py)|bitwise|
|342|[Power of Four](https://leetcode.com/problems/power-of-four/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_4.py)|bitwise|
|344|[Reverse Array](https://leetcode.com/problems/reverse-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/reverse_string.py)||
|371|[Sum of Two Integers](https://leetcode.com/problems/sum-of-two-integers/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/binary_addition.py)|binary_addition|
|409|[Longest Palindrome](https://leetcode.com/problems/longest-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_combination.py)|dp(greedy)|
|413|[Reverse Integer](https://www.leetcode.com/problems/reverse-integer/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/reverse_integer.py)||
|449|[Serialize and Deserialize BST](https://leetcode.com/problems/serialize-and-deserialize-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_serialize.py)|DFS, stack|
|454|[4Sum II](https://www.leetcode.com/problems/4sum-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/four_sum_2.py)|two_sum|
|507|[Perfect Number](https://www.leetcode.com/problems/perfect-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/perfect_number.py)||
|509|[Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/classic/tail_recursion_fibonacci.rs)|tail_recursion, fibonacci|
|536_LOCK|[Construct Binary Tree from String](https://www.leetcode.com/problems/construct-binary-tree-from-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_string_with_parentheses.py)||
|547|[Friend Circles](https://leetcode.com/problems/friend-circles/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/friend_circles.py)|union_find, BFS, BFS|
|606|[Construct String from Binary Tree](https://www.leetcode.com/problems/construct-string-from-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_string_with_parentheses.py)||
|625_LOCK|[Minimum Factorization](https://www.leetcode.com/problems/minimum-factorization/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/minimum_factorization.py)|greedy|
|658|[Find K Closest Elements](https://leetcode.com/problems/find-k-closest-elements/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/find_k_closest_elements.py)|binary_search|
|680|[Valid Palindrome II](https://leetcode.com/problems/valid-palindrome-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome_2.py)|two_pointers, greedy|
|702_LOCK|[Search in a Sorted Array of Unknown Size](https://leetcode.com/problems/search-in-a-sorted-array-of-unknown-size/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search_unknown_size_sorted_array.py)|binary_search_first, 倍增法|
|704|[Binary Search](https://leetcode.com/problems/binary-search/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search.py)|binary_search|
|704|[Design HashMap](https://leetcode.com/problems/design-hashmap/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/impl_hashmap.py)||
|705|[Design HashSet](https://leetcode.com/problems/design-hashset/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/impl_hashmap.py)||
|709|[To Lower Case](https://leetcode.com/problems/to-lower-case/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/impl_api/to_lowercase.rs)||
|796|[Rotate String](https://leetcode.com/problems/rotate-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/string/rotate_string.py)|Rabin-Karp(rolling_hash), kmp|
|743_TODO|[Network Delay Time](https://leetcode.com/problems/network-delay-time/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/graph_traversal/dijkstra_shortest_path_algorithm.rs)|dijkstra_shortest_path|
|845|[Longest Mountain in Array](https://leetcode.com/problems/longest-mountain-in-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_longest.py)|mountain_array|
|852|[Peak Index in a Mountain Array](https://leetcode.com/problems/peak-index-in-a-mountain-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_max.py)|mountain_array, binary_search|
|869|[Reordered Power of 2](https://leetcode.com/problems/reordered-power-of-2/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/reordered_power_of_2.py)|permutation|
|876|[Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/middle_of_linked_list.py)|list_node, 快慢指针|
|887_TODO|[Super Egg Drop](https://leetcode.com/problems/super-egg-drop)||dp|
|889|[Construct Binary Tree from Preorder and Postorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_pre_order_and_post_order.py)|DFS|
|912|[Sort An Array](https://leetcode.com/problems/sort-an-array/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/quick_sort.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/basic_sorting.py)|multi_solutions|
|941|[Valid Mountain Array](https://leetcode.com/problems/find-in-mountain-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_valid.py)|mountain_array|
|953|[Verifying an Alien Dictionary](https://leetcode.com/problems/verifying-an-alien-dictionary/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/verifying_an_alien_dictionary.py)||
|973|[K Closest Points to Origin](https://leetcode.com/problems/k-closest-points-to-origin/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/k_closest_points_to_origin.py)|quick_select, sort_by_multi_keys|
|1095|[Find in Mountain Array](https://leetcode.com/problems/find-in-mountain-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_find.py)|binary_search, mountain_array|
|1099_LOCK|[Two Sum Less Than K](https://leetcode.com/problems/two-sum-less-than-k/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/two_sum_le_count.py)|two_pointers|
|1429_LOCK|[First Unique Number](https://leetcode.com/problems/first-unique-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/first_unique_number.py)||
|1470|[Shuffle the Array](https://leetcode.com/problems/shuffle-the-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/shuffle_the_array.py), [Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/unclassified/shuffle_the_array.rs)||
|1476|[Subrectangle Queries](https://leetcode.com/problems/subrectangle-queries/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/subrectangle_queries.py)||
|1480|[Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/running_sum_of_1d_array.py)||


---

LintCode
========

| # | Title | Solutions | Category |
|---| ----- | -------- | ---------- |
|1|[A + B Problem](https://www.lintcode.com/problem/a-b-problem/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/binary_addition.py)|binary_addition|
|2|[Trailing Zeros](https://www.lintcode.com/problem/trailing-zeros/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/factorial_trailing_zeroes.py)|factorial|
|4|[Ugly Number II](https://www.leetcode.com/problems/ugly-number-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/ugly_number_2_nth_ugly.py)||
|5|[Kth Largest Element](https://www.lintcode.com/problem/kth-largest-element/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)|quick_select, quick_sort|
|6|[Merge Two Sorted Arrays](https://www.lintcode.com/problem/merge-two-sorted-arrays/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/merge_two_sorted_arrays.py)|merge_sort|
|7|[Serialize and Deserialize Binary Tree](https://lintcode.com/problem/serialize-and-deserialize-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree.py)|serialize|
|8|[Rotate String](https://lintcode.com/problem/rotate-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/string/rotate_string.py)|Rabin-Karp|
|10|[String Permutation II](https://lintcode.com/problem/string-permutation-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/permutation.py)|backtracking|
|13|[Implement strStr()](https://www.lintcode.com/problem/implement-strstr/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/string/find_substr.py)|kmp, Rabin-Karp(rolling_hash)|
|15|[Permutations](https://lintcode.com/problem/permutations/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/permutation.py)|backtracking|
|16|[Permutations II](https://lintcode.com/problem/permutations-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/permutation.py)|backtracking|
|17|[Subsets](https://lintcode.com/problem/subsets/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/subsets.py)||
|18|[Subsets II](https://lintcode.com/problem/subsets-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/subsets.py)||
|14|[First Position of Target](https://www.lintcode.com/problem/first-position-of-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search_first_and_last.py)|binary_search|
|31|[Partition Array](https://www.lintcode.com/problem/partition-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/partition_array.py)|two_pointers|
|35|[Reverse Linked List](https://lintcode.com/problem/reverse-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/reverse_linked_list.py)|list_node|
|40|[Implement Queue by Two Stacks](https://www.lintcode.com/problem/implement-queue-by-two-stacks/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/impl_collections/impl_stack_using_queue.py)||
|41|[Maximum Subarray](https://www.lintcode.com/problem/maximum-subarray/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/maximum_subarray.py)|greedy, dp|
|49|[Sort Letters by Case](https://www.lintcode.com/problem/sort-letters-by-case/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/partition_array_by_odd_and_even.py)||
|53|[Reverse Words in a String](https://lintcode.com/problem/reverse-words-in-a-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/reverse_words_in_a_string.py)||
|56|[Two Sum](https://www.lintcode.com/problem/two-sum/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/collections/btree_map_two_sum.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/collections/two_sum_test.go), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/test/java/com/leetcode/collections/HashMapTwoSum.java)|btree_map, bitwise(two_s_complement)|
|57|[3Sum](https://www.lintcode.com/problem/3sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/three_sum.py)|two_pointers, two_sum|
|58|[4Sum](https://www.lintcode.com/problem/4sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/four_sum.py)|two_pointers, two_sum|
|59|[3Sum Closest](https://www.lintcode.com/problem/3sum-closest/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/https://github.com/pymongo/rust_leetcode/blob/master/src/two_sum/three_sum_closest.py)|two_sum|
|62|[Search in Rotated Sorted Array](https://lintcode.com/problem/search-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search.py)|binary_search|
|63|[Search in Rotated Sorted Array II](https://lintcode.com/problem/search-in-rotated-sorted-array-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search_2_with_duplicate.py)|binary_search|
|64|[Merge Sorted Array](https://lintcode.com/problem/merge-sorted-array/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/merge_two_sorted_arrays.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/sorting/merge_two_sorted_arrays_test.go), [Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/merge_two_sorted_arrays.py)|merge_sort|
|65|[Median of Two Sorted Arrays](https://www.lintcode.com/problem/median-of-two-sorted-arrays/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/sorting/median_of_two_sorted_arrays.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/median_of_two_sorted_arrays.py)|binary_search|
|66|[Binary Tree Preorder Traversal](https://www.lintcode.com/problem/binary-tree-preorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|67|[Binary Tree Inorder Traversal](https://www.lintcode.com/problem/binary-tree-inorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|68|[Binary Tree Postorder Traversal](https://www.lintcode.com/problem/binary-tree-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|69|[Binary Tree Level Order Traversal](https://lintcode.com/problem/binary-tree-level-order-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_level_order.py)|BFS|
|69|[Binary Tree Level Order Traversal II](https://lintcode.com/problem/binary-tree-level-order-traversal-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_level_order.py)|BFS|
|72|[Construct Binary Tree from Inorder and Postorder Traversal](https://lintcode.com/problem/construct-binary-tree-from-inorder-and-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_in_order_and_post_order.py)|DFS|
|73|[Construct Binary Tree from Preorder and Inorder Traversal](https://lintcode.com/problem/construct-binary-tree-from-preorder-and-inorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_pre_order_and_in_order.py)|DFS, stack|
|74|[First Bad Version](https://lintcode.com/problem/first-bad-version/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/first_bad_version.py)|partition_array|
|75|[Find Peak Element](https://www.lintcode.com/problem/find-peak-element/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_max.py)|mountain_array, binary_search|
|86|[Binary Search Tree Iterator](https://lintcode.com/problem/binary-search-tree-iterator/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_in_order_iterator.py)||
|88|[Lowest Common Ancestor of a Binary Tree](https://lintcode.com/problem/lowest-common-ancestor-of-a-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)|divide_and_conquer|
|90|[k Sum II](https://lintcode.com/problem/combination-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/combination_target_sum.py)||
|93|[Balanced Binary Tree](https://www.lintcode.com/problem/balanced-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_is_balance_binary_tree.py)|divide_and_conquer|
|97|[Maximum Depth of Binary Tree](https://www.lintcode.com/problem/maximum-depth-of-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_max_min.py)||
|100|[Remove Duplicates from Sorted Array](https://lintcode.com/problem/remove-duplicates-from-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/move_zeros.py)||
|114|[Unique Paths](https://lintcode.com/problem/unique-paths/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/permutation/shortest_paths_on_checkerboard.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/permutation/shortest_paths_on_checkerboard_test.go)|permutation|
|120|[Word Ladder](https://lintcode.com/problem/word-ladder/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/word_ladder.py)|双向BFS|
|121|[Word Ladder II](https://lintcode.com/problem/word-ladder-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/word_ladder_2.py)|BFS+DFS|
|123|[Word Search](https://lintcode.com/problem/word-search/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/word_search.py)||
|127|[Topological Sorting](https://lintcode.com/problem/topological-sorting/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/topological_sorting.py)|BFS, topological_sorting|
|128|[Hash Function](https://www.lintcode.com/problem/hash-function/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/impl_collections/hash_function.py)||
|129|[Rehashing](https://www.lintcode.com/problem/rehashing/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/impl_collections/rehashing.py)||
|132|[Word Search II](https://lintcode.com/problem/word-search-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/word_search_2.py)|前缀树|
|134|[LRU Cache](https://lintcode.com/problem/lru-cache/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/lru_cache.py)|double_linked_list, OrderedDict|
|135|[Combination Sum](https://lintcode.com/problem/combination-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/combination_target_sum.py)||
|137|[Clone Graph](https://lintcode.com/problem/clone-graphs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/clone_graph.py)|DFS, BFS|
|138|[Subarray Sum](https://www.lintcode.com/problem/subarray-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/subarray_sum_zero.py)||
|140|[Fast Power](https://lintcode.com/problem/fast-power/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/pow.py)|binary_search, 快速幂运算|
|141|[Sqrt(x)](https://lintcode.com/problem/sqrtx/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/sqrt.py)|牛顿连续均值求根法|
|142|[O(1) Check Power of 2](https://www.lintcode.com/problem/o1-check-power-of-2/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_2.py)|bitwise, dichotomy|
|143|[Sort Colors II](https://lintcode.com/problem/sort-colors-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/sort_colors_2.py)|quick_sort, counting_sort|
|144|[interleaving_positive_and_negative_numbers](https://lintcode.com/problem/sort-colors-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/interleaving_positive_and_negative_numbers.py)||
|147_LOCK_TODO|[Narcissistic Number](https://www.lintcode.com/problem/narcissistic-number)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/narcissistic_number.py)||
|148|[Sort Colors](https://lintcode.com/problem/sort-colors/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/sort_colors.py)|three_pointers, partition_array|
|155|[Minimum Depth of Binary Tree](https://www.lintcode.com/problem/minimum-depth-of-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_max_min.py)||
|159|[Find Minimum in Rotated Sorted Array](https://www.lintcode.com/problem/find-minimum-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search.py)|binary_search|
|160|[Find Minimum in Rotated Sorted Array II](https://www.lintcode.com/problem/find-minimum-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_min_2_with_duplicate.py)|binary_search|
|167|[Add Two Numbers](https://www.lintcode.com/problem/add-two-numbers/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/collections/traverse_two_list_node.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/collections/traverse_two_list_node_test.go), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/test/java/com/leetcode/collections/TraverseTwoListNode.java), [Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/add_two_numbers.py)|create/traverse_list_node|
|172|[Subarray Sum](https://www.lintcode.com/problem/remove-element/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/move_zeros.py)||
|183|[Wood Cut](https://www.lintcode.com/problem/wood-cut/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/wood_cut.py)|greedy|   
|187|[Gas Station](https://www.lintcode.com/problem/gas-station/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/gas_station.py)|greedy|
|200|[Longest Palindromic Substring](https://lintcode.com/problem/longest-palindromic-substring/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/string/longest_palindromic_substr.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_substr.py)|multi_solutions, manacher, suffix_array|
|228_LOCK|[Middle of the Linked List](https://www.lintcode.com/problem/middle-of-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/middle_of_linked_list.py)|list_node, 快慢指针|
|235|[Prime Factorization](https://www.lintcode.com/problem/prime-factorization/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/prime_factorization.py)|分解质因数|
|254|[Drop Eggs](https://www.lintcode.com/problem/drop-eggs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/drop_eggs.py)|sqrt_n|
|366|[Fibonacci](https://www.lintcode.com/problem/fibonacci/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/classic/tail_recursion_fibonacci.rs)|tail_recursion, fibonacci|
|373|[Partition Array by Odd and Even](https://www.lintcode.com/problem/partition-array-by-odd-and-even/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/partition_array_by_odd_and_even.py)||
|376|[Binary Tree Path Sum](https://www.lintcode.com/problem/binary-tree-path-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/root_to_leaf_paths_target_sum.py)|DFS, backtracking|
|382_LOCK|[Triangle Count](https://www.lintcode.com/problem/triangle-count/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/triangle_count.py)|two_sum, 双指针|
|384|[Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/string/longest_non_repeated_substr.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/string/longest_non_repeated_substr.py)|sliding_window|
|415|[Valid Palindrome](https://www.lintcode.com/problem/valid-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome.py)|two_pointers|
|425|[Letter Combinations of a Phone Number](https://lintcode.com/problem/letter-combinations-of-a-phone-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/phone_9_keypad_combination.py)|product(笛卡尔积)|
|428|[Pow(x, n)](https://lintcode.com/problem/powx-n/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/pow.py)|binary_search, 快速幂运算|
|433|[Number of Islands](https://www.lintcode.com/problem/number-of-islands/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/number_of_islands.py)|union_find, DFS, BFS|
|442|[Implement Trie (Prefix Tree)](https://lintcode.com/problem/implement-trie-prefix-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/prefix_tree_tire/impl_tire_prefix_tree.py)|前缀树/字典树|
|443_LOCK|[Two Sum - Greater than target](https://www.lintcode.com/problem/two-sum-greater-than-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/two_sum_le_count.py)|two_pointers|
|447_LOCK|[Search in a Big Sorted Array](https://www.lintcode.com/problem/search-in-a-big-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search_unknown_size_sorted_array.py)|binary_search_first, 倍增法|
|453|[Flatten Binary Tree to Linked List](https://lintcode.com/problem/flatten-binary-tree-to-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/in_place_flatten_binary_tree_to_linked_list.py)||
|457|[Classical Binary Search](https://www.lintcode.com/problem/classical-binary-search/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search.py)|binary_search|
|458|[Last Position of Target](https://www.lintcode.com/problem/last-position-of-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search_first_and_last.py)|binary_search|
|460|[Find K Closest Elements](https://lintcode.com/problem/find-k-closest-elements/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/find_k_closest_elements.py)|binary_search|
|463|[Sort Integers](https://www.lintcode.com/problem/sort-integers/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/basic_sorting.py)|multi_solutions|
|464|[Sort Integers II](https://www.lintcode.com/problem/sort-integers-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/basic_sorting.py)|multi_solutions|
|474_LOCK|[Lowest Common Ancestor II](https://www.lintcode.com/problem/lowest-common-ancestor-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)||
|480|[Binary Tree Paths](https://lintcode.com/problem/binary-tree-paths/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/root_to_leaf_paths_find_all.py)|DFS, backtracking|
|491|[Palindromic Number](https://www.lintcode.com/problem/palindrome-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_substr.py)||
|492|[Implement Queue by Linked List](https://www.lintcode.com/problem/implement-queue-by-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/impl_collections/impl_queue_by_linked_list.py)||
|494|[Implement Stack by Two Queues](https://www.lintcode.com/problem/implement-stack-by-two-queues/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/impl_collections/impl_stack_using_queue.py)||
|495|[Implement Stack](https://www.lintcode.com/problem/implement-stack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/impl_collections/impl_stack_using_queue.py)||
|517|[Ugly Number](https://www.lintcode.com/problem/ugly-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/ugly_number.py)||
|521_LOCK|[Remove Duplicate Numbers in Array](https://www.lintcode.com/problem/remove-duplicate-numbers-in-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/remove_duplicate_numbers_in_array.py)|partition_array|
|533_LOCK|[Two Sum - Closest to target](https://www.lintcode.com/problem/two-sum-closest-to-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/two_sum_unique_pairs.py)|two_sum|
|539|[Move Zeros](https://lintcode.com/problem/move-zeroes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/move_zeros.py)|partition_array, 快慢双指针|
|578|[Lowest Common Ancestor III](https://www.lintcode.com/problem/lowest-common-ancestor-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)||
|584_TODO|[Drop Eggs II](https://www.lintcode.com/problem/drop-eggs-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/drop_eggs_dp.py)|dp|
|585_LOCK|[Maximum Number in Mountain Sequence](https://www.lintcode.com/problem/maximum-number-in-mountain-sequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_max.py)|mountain_array, binary_search|
|586_LOCK|[Sqrt(x) II](https://www.lintcode.com/problem/sqrtx-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/sqrt.py)|牛顿连续均值求根法|
|587_LOCK|[Two Sum - Unique pairs](https://www.lintcode.com/problem/two-sum-unique-pairs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/two_sum_unique_pairs.py)|two_sum|
|594|[Implement strStr() II](https://www.lintcode.com/problem/strstr-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/string/find_substr.py)|kmp, Rabin-Karp(rolling_hash)|
|596_LOCK|[Minimum Subtree](https://www.lintcode.com/problem/maximum-subtree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/subtree_max_sum.py)|divide_and_conquer|
|607|[Two Sum III - Data structure design](https://lintcode.com/problem/two-sum-iii-data-structure-design/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/two_sum_3_impl.py)|two_pointers|
|608|[Two Sum II - Input array is sorted](https://lintcode.com/problem/two-sum-ii-input-array-is-sorted/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/two_sum_2_input_is_sorted.py)|two_pointers|
|609|[Two Sum - Less than or equal to target](https://www.lintcode.com/problem/two-sum-less-than-or-equal-to-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/two_sum_le_count.py)|two_pointers|
|611_LOCK|[Knight Shortest Path](https://www.lintcode.com/problem/knight-shortest-path/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/knight_shortest_path.py)|bfs|
|612|[K Closest Points](https://www.lintcode.com/problem/k-closest-points/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/k_closest_points_to_origin.py)|quick_select, sort_by_multi_keys|
|616|[Course Schedule II](https://lintcode.com/problem/course-schedule-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/course_schedule_2.py)|BFS, topological_sorting|
|627|[Longest Palindromic Combination](https://www.lintcode.com/problem/longest-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_combination.py)|greedy|
|628|[Maximum Subtree](https://www.lintcode.com/problem/maximum-subtree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/subtree_max_sum.py)|divide_and_conquer|
|630_LOCK|[Knight Shortest Path II](https://www.lintcode.com/problem/knight-shortest-path-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/knight_shortest_path_2.py)|bfs|
|657|[Insert Delete GetRandom](https://www.lintcode.com/problem/insert-delete-getrandom-o1/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/impl_collections/insert_delete_random.py)||
|667|[Longest Palindromic Subsequence](https://www.lintcode.com/problem/longest-palindromic-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_subsequence.py)|dp(greedy)|
|685|[First Unique Number in Data Stream](https://www.lintcode.com/problem/first-unique-number-in-data-stream/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/first_unique_number.py)||
|816|[Traveling Salesman Problem](https://www.lintcode.com/problem/traveling-salesman-problem/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/traveling_salesman_problem.py)||
|871|[Minimum Factorization](https://www.lintcode.com/problem/minimum-factorization/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/minimum_factorization.py)|greedy|
|880|[Construct Binary Tree from String](https://www.lintcode.com/problem/construct-binary-tree-from-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_string_with_parentheses.py)||
|891|[Valid Palindrome II](https://lintcode.com/problem/valid-palindrome-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome_2.py)|two_pointers, greedy|
|892|[Alien Dictionary](https://lintcode.com/problem/alien-dictionary/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/alien_dictionary.py)|heapq, topological_sorting|
|900|[Closest Binary Search Tree Value](https://www.lintcode.com/problem/closest-binary-search-tree-value/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value.py)||
|901|[Closest Binary Search Tree Value II](https://www.lintcode.com/problem/closest-binary-search-tree-value-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value_2.py)||
|902|[Kth Smallest Element in a BST](https://lintcode.com/problem/kth-smallest-element-in-a-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|916|[Palindrome Permutation](https://www.lintcode.com/problem/palindrome-permutation/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/palindrome_permutation.py)|greedy|
|976|[4Sum II](https://www.lintcode.com/problem/4sum-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum/four_sum_2.py)|two_sum|
|1137|[Construct String from Binary Tree](https://www.lintcode.com/problem/construct-string-from-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_string_with_parentheses.py)||
|1179|[Friend Circles](https://lintcode.com/problem/friend-circles/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/friend_circles.py)|union_find, DFS, BFS|
|1199|[Perfect Number](https://www.lintcode.com/problem/perfect-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/perfect_number.py)||
|1235|[Serialize and Deserialize BST](https://lintcode.com/problem/serialize-and-deserialize-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_serialize.py)|DFS, stack|
|1276|[Sum of Two Integers](https://lintcode.com/problem/sum-of-two-integers/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/binary_addition.py)|binary_addition|
|1283|[Reverse Array](https://lintcode.com/problem/reverse-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/reverse_string.py)||
|1285|[Power of Four](https://lintcode.com/problem/power-of-four/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_4.py)|bitwise|
|1294|[Power of Three](https://lintcode.com/problem/power-of-three/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_3.py)|bitwise|
|1300|[Bash Game(Nim Game)](https://www.lintcode.com/problem/bash-game/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/bitwise/nim_game.rs)||
|1311|[Lowest Common Ancestor of a Binary Search Tree](https://lintcode.com/problem/lowest-common-ancestor-of-a-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_lowest_common_ancestor.py)||
|1334|[Rotate Array](https://www.lintcode.com/problem/rotate-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/rotate_array_right_circle_shift_elements.py)|reverse, circle_shift|
|1363|[ZigZag Conversion](https://lintcode.com/problem/zigzag-conversion/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/string/zigzag_conversion.py)||
|1424|[Longest Mountain in Array](https://www.lintcode.com/problem/longest-mountain-in-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_longest.py)|mountain_array|
|1479|[Can Reach The Endpoint](https://www.lintcode.com/problem/can-reach-the-endpoint/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/can_reach_the_end_point.py)|bfs|
|1499|[Reordered Power of 2](https://lintcode.com/problem/reordered-power-of-2/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/reordered_power_of_2.py)|permutation|
|1535|[To Lower Case](https://www.lintcode.com/problem/to-lower-case/)|[Rust](https://github.com/pymongo/rust_leetcode/blob/master/src/impl_api/to_lowercase.rs)||
|1536|[Find First and Last Position of Element in Sorted Array](https://www.lintcode.com/problem/find-first-and-last-position-of-element-in-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search_first_and_last.py)|binary_search|
|1593|[Construct Binary Tree from Preorder and Postorder Traversal](https://lintcode.com/problem/construct-binary-tree-from-preorder-and-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_pre_order_and_post_order.py)|DFS|
|1609|[Middle of the Linked List](https://lintcode.com/problem/middle-of-the-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/list_node/middle_of_linked_list.py)|list_node, 快慢指针|
|1790_LOCK|[Rotate String II](https://www.lintcode.com/problem/rotate-string-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/rotate_string_2.py)|reverse, circle_shift|
|1848_ABANDONED|[Word Search III](https://www.lintcode.com/problem/word-search-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_permutation_combination/word_search_3.py)||
|1876|[Alien Dictionary(easy)](https://www.lintcode.com/problem/alien-dictionaryeasy/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/verifying_an_alien_dictionary.py)||

---

Project Euler
===

TODO

# Category

## Palindrome

### Palindrome Sequence

- [Longest Palindromic Subsequence](https://www.lintcode.com/problem/longest-palindromic-subsequence) | [Python](https://github.com/pymongo/python_leetcode/blob/master/https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_subsequence.py) | dp(greedy)
- [Valid Palindrome II](https://lintcode.com/problem/valid-palindrome-ii/) | [Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome_2.py) | two_pointers, greedy

### Palindrome Permutation

- [Longest Palindromic Combination](https://www.lintcode.com/problem/longest-palindrome) | [Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_combination.py) | greedy
- [Palindrome Permutation](https://www.lintcode.com/problem/palindrome-permutation/) | [Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/palindrome_permutation.py) | greedy

## greedy

- [Gas Station](https://www.lintcode.com/problem/gas-station/) | [Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/gas_station.py) | greedy
- [Maximum Subarray](https://www.lintcode.com/problem/maximum-subarray/) | [Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/maximum_subarray.py) |greedy, dp

---

- [rust leetcode](https://github.com/pymongo/rust_leetcode)
- [go_leetcode](https://github.com/pymongo/go_leetcode)
- [java_leetcode](https://github.com/pymongo/java_leetcode)
- [python_leetcode](https://github.com/pymongo/python_leetcode)

---

TODO 链表翻转, 链表厂(ByteDance)喜欢考链表

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

## pip和pipenv

python自带的pip没有类似`pom.xml`或`package.json`之类管理项目第三方依赖的清单文件

> pipenv uses Pipfile and Pipfile.lock

所以推荐用pyenv管理python版本，pipenv管理第三方依赖

## Python测试用例可读性

```python
import unittest
from copy import deepcopy

def solution(nums, target): pass

class UnitTest(unittest.TestCase):
    TEST_CASES = [
        ([-1, 0, 3, 5, 9, 12], 9, 4),
        ([-1, 0, 3, 5, 9, 12], 2, -1),
        ([-1, 1, 2, 2, 2, 3], 2, 2),
    ]

    def test(self):
        for nums, target, expected in deepcopy(self.TEST_CASES):
            self.assertEqual(expected, solution(nums, target))
```

另一种提高单元测试-测试用例可读性的方法，用namedtuple

例如Point = namedtuple('Point', ['x', 'y']) 会生成/定义一个含有x,y字段的Point类
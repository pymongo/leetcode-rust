- problem number with `🔒` suffix need leetcode/lintcode subscription to unlock
- problem number with `🤔` suffix has many solutions(I hasn't write all solutions)
- problem number with `❌` suffix is a bad solution(like leetcode#371 Python can't do bitwise add)

codeforces_solutions
===

| Problem | Solution |
| ------- | -------- |
|[1A - Theatre Square](https://codeforces.com/problemset/problem/1/A)|[Rust](examples/codeforces_easy_problems.rs)|
|[4A - Watermelon](https://codeforces.com/problemset/problem/4/A)|[Rust](examples/codeforces_easy_problems.rs)|
|[71A - Way Too Long Words](https://codeforces.com/problemset/problem/71/A)|[Rust](examples/codeforces_easy_problems.rs)|

leetcode_solutions
===

%w(Python Rust Go Java C++).sort().reverse() = \["Rust", "Python", "Java", "Go", "C++"]

| # | Problem | Solutions | Category/Comment |
|---| ------- | --------- | ---------------- |
|1|[Two Sum](https://leetcode.com/problems/two-sum/)|[Rust](src/easy/two_sum.rs), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/test/java/com/leetcode/collections/HashMapTwoSum.java), [Go](https://github.com/pymongo/go_leetcode/blob/master/two_sum_test.go)|bitwise|
|2|[Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)|[Rust](src/linked_list/add_two_linked_list.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/add_two_numbers.py), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/test/java/com/leetcode/collections/TraverseTwoListNode.java), [Go](https://github.com/pymongo/go_leetcode/blob/master/traverse_two_list_node_test.go)|linked_list|
|3|[Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/)|[Rust](src/string/longest_non_repeated_substr.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/string/longest_non_repeated_substr.py)|sliding_window|
|4|[Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)|[Rust](src/binary_search/median_of_two_sorted_arrays.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/median_of_two_sorted_arrays.py)|binary_search|
|5🤔|[Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/)|[Rust](src/dp/longest_palindromic_substr.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_substr.py)|manacher, suffix_array|
|6|[ZigZag Conversion](https://leetcode.com/problems/zigzag-conversion/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/string/zigzag_conversion.py)|
|7|[Reverse Integer](https://leetcode.com/problems/reverse-integer/)|[Rust](src/easy/reverse_integer_checked_mul_overflow.rs)|
|8|[String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/string_to_integer_atoi.py)|
|9|[Palindromic Number](https://leetcode.com/problems/palindrome-number/)|[Rust](src/easy/reverse_integer_checked_mul_overflow.rs)|
|10|[Regular Expression Matching](https://leetcode.com/problem/regular-expression-matching/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/regular_expression_matching.py)|
|11|[Container With Most Water](https://leetcode.com/problems/container-with-most-water/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/container_with_most_water.py)|
|12|[Integer to Roman](https://leetcode.com/problems/integer-to-roman/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/int_to_roman.py)|
|13|[Roman to Integer](https://leetcode.com/problems/roman-to-integer/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/int_to_roman.py)|
|14|[Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/longest_common_prefix_of_k_strs.py)|
|15|[3Sum](https://leetcode.com/problems/3sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/three_sum_plans_detail.py)|two_pointers, two_sum|
|16|[3Sum Closest](https://leetcode.com/problems/3sum-closest/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/three_sum_closest.py)|two_sum|
|17|[Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/phone_9_keypad_combination.py)|product(笛卡尔积)|
|18|[4Sum](https://leetcode.com/problems/4sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/four_sum_plans_detail.py)|two_pointers, two_sum|
|19|[Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/remove_nth_node_from_end.py)|sliding_window|
|20|[Valid Parentheses](https://leetcode.com/problems/valid-parentheses/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/valid_parentheses.py)|stack|
|21|[Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)|[Rust](src/linked_list/merge_two_sorted_linked_list.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_k_sorted_lists.py)|
|22|[Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/generate_parentheses.py)|backtracking|
|23|[Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_k_sorted_lists.py)|
|24|[Swap Nodes in Pairs](https://leetcode.com/problems/swap-nodes-in-pairs/)|[Rust](src/linked_list/swap_nodes_in_pairs.rs)|
|26|[Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/)|[Rust](src/easy/array/partition_array.rs)|
|27|[Remove Element](https://leetcode.com/problems/remove-element/)|[Rust](src/easy/array/partition_array.rs)|
|28🤔|[Implement strStr()](https://leetcode.com/problems/implement-strstr/)|[Rust](src/string/contains_substr_kmp.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/string/find_substr.py)|kmp, dfa|
|31|[Next Permutation](https://leetcode.com/problems/next-permutation/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/next_permutation.py)|
|33|[Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search.py)|binary_search|
|34|[Find First and Last Position of Element in Sorted Array](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/)|[Rust](src/binary_search/mod.rs)|
|35|[Search Insert Position](https://leetcode.com/problems/search-insert-position/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/search_insert_position.py)|
|36|[Valid Sudoku](https://leetcode.com/problems/valid-sudoku/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/valid_soduku.py)|
|37|[Soduku Solver](https://leetcode.com/problems/soduku-solver/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/soduku_solver.py)|
|38|[Count And Say](https://leetcode.com/problems/count-and-say/)|[Rust](src/easy/very_easy.rs)|
|39|[Combination Sum](https://leetcode.com/problems/combination-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/combination_target_sum.py), [Rust](src/backtracking/combination_sum_1_2.rs)|
|40|[Combination Sum II](https://leetcode.com/problems/combination-sum-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/combination_target_sum.py), [Rust](src/backtracking/combination_sum_1_2.rs)|
|42|[Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/)|[Rust](src/dp/trapping_rain_water.rs)|
|44|[Wildcard Matching](https://leetcode.com/problems/wildcard-matching/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/wildcard_matching.py)|
|45|[Jump Game II](https://leetcode.com/problems/jump-game-ii/)|[Rust](src/dp/jump_game_2.rs)|greedy|
|46|[Permutations](https://leetcode.com/problems/permutations/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/permutation.py)|backtracking|
|47|[Permutations II](https://leetcode.com/problems/permutations-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/permutation.py)|backtracking|
|48|[Rotate Image](https://leetcode.com/problems/rotate-image/)|[Rust](src/easy/grid_or_matrix/rotate_matrix.rs)|
|49|[Group Anagrams](https://leetcode.com/problems/group-anagrams/)|[Rust](src/easy/counter.rs)|
|50|[Pow(x, n)](https://leetcode.com/problems/powx-n/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/pow.py)|binary_search|
|51|[N Queens](https://leetcode.com/problems/n-queens)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/n_queens.py), [Rust](src/backtracking/n_queens.rs)||
|52|[N Queens II](https://leetcode.com/problems/n-queens-ii)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/n_queens.py), [Rust](src/backtracking/n_queens.rs)||
|53|[Maximum Subarray](https://leetcode.com/problems/maximum-subarray/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/maximum_subarray.py)|greedy, dp|
|54|[Spiral Matrix](https://leetcode.com/problems/spiral-matrix/)|[Rust](src/easy/grid_or_matrix/spiral_matrix.rs)|
|55|[Jump Game](https://leetcode.com/problems/jump-game/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/jump_game.py)|greedy, dp|
|59|[Spiral Matrix II](https://leetcode.com/problems/spiral-matrix-2/)|[Rust](src/easy/grid_or_matrix/spiral_matrix_2.rs)|
|60|[Permutation Sequence](https://leetcode.com/problems/permutation-sequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/permutation.py)||
|62|[Unique Paths](https://leetcode.com/problems/unique-paths/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/unique_paths.py), [Rust](src/easy/unique_path.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/shortest_paths_on_checkerboard_test.go)|combination|
|63|[Unique Paths II](https://leetcode.com/problems/unique-paths-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/unique_paths_2.py)|
|64|[Minimum Path Sum](https://leetcode.com/problems/minimum-path-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/grid_min_path_sum.py)||
|65|[Valid Number](https://leetcode.com/problems/valid-number/)|[Rust](src/easy/very_easy.rs)|
|66|[Plus One](https://leetcode.com/problems/plus-one/)|[Rust](src/easy/very_easy.rs)|
|67|[Add Binary](https://lintcode.com/problem/add-binary/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/easy/add_bigint_string.cpp)||
|69|[Sqrt(x)](https://leetcode.com/problems/sqrtx/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/sqrt.py)|牛顿连续均值求根法|
|70|[Climb Stairs](https://leetcode.com/problems/climbing-stairs/)|[Rust](src/easy/very_easy.rs)|fibonacci|
|72|[Edit Distance](https://leetcode.com/problems/edit-distance/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/edit_distance.py)||
|73|[Set Matrix Zeroes](https://leetcode.com/problems/set-matrix-zeroes/)|[Rust](src/easy/very_easy.rs)|
|74|[Search a 2D Matrix](https://leetcode.com/problems/search-a-2d-matrix/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/search_a_2d_matrix.py)||
|75|[Sort Colors](https://leetcode.com/problems/sort-colors/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/sort_colors.py)|three_pointers, partition_array|
|77|[Combinations](https://leetcode.com/problems/combinartions/)|[Rust](src/backtracking/combinations.rs), [C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/backtracking/combinartions.cpp)|
|78|[Subsets](https://leetcode.com/problems/subsets/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/subsets_combinations.py)|
|79|[Word Search](https://leetcode.com/problems/word-search/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/word_search.py)|
|81|[Search in Rotated Sorted Array II](https://leetcode.com/problems/search-in-rotated-sorted-array-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search_2_with_duplicate.py)|binary_search|
|82|[Remove Duplicates from Sorted List II](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/linked_list/remove_duplicates_from_sorted_list.cpp)||
|83|[Remove Duplicates from Sorted List](https://leetcode.com/problems/remove-duplicates-from-sorted-list/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/linked_list/remove_duplicates_from_sorted_list.cpp)||
|88|[Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/)|[Rust](src/easy/very_easy.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_two_sorted_arrays.py), [Go](https://github.com/pymongo/go_leetcode/blob/master/merge_two_sorted_arrays_test.go)|merge_sort|
|89|[Gray Code](https://leetcode.com/problems/gray-code/)|[Rust](src/easy/array/gray_code.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/easy/gray_code.py)||
|90|[Subsets](https://leetcode.com/problems/subsets-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/subsets_combinations.py)|
|91|[Decode Ways](https://leetcode.com/problems/decode-ways/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/decode_ways.py)|
|92|[Reverse Linked List II](https://leetcode.com/problems/reverse-linked-list-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/reverse_linked_list.py)|
|93|[Restore IP Addresses](https://leetcode.com/problems/restore-ip-addresses/)|[Rust](src/backtracking/restore_ip_address.rs)|
|94|[Binary Tree Inorder Traversal](https://leetcode.com/problems/binary-tree-inorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|98|[Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_valid.py)|
|99|[Recover Binary Search Tree](https://leetcode.com/problems/recover-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_recover_swap_two_nodes.py)|
|100|[Same Tree](https://leetcode.com/problems/same-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/is_same_tree.py)|
|102|[Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/)|[Rust](src/binary_tree/level_order_traversal.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_level_order.py)|
|104|[Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_max_min.py)||
|105|[Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_pre_order_and_in_order.py)|DFS, stack|
|106|[Construct Binary Tree from Inorder and Postorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_in_order_and_post_order.py)|DFS|
|107|[Binary Tree Level Order Traversal II](https://leetcode.com/problems/binary-tree-level-order-traversal-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_level_order.py)|BFS|
|108|[Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/sorted_list_or_array_to_bst.py)|divide_and_conquer|
|109|[Convert Sorted List to Binary Search Tree](https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/sorted_list_or_array_to_bst.py)|divide_and_conquer|
|110|[Balanced Binary Tree](https://leetcode.com/problems/balanced-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_is_balance_binary_tree.py)|divide_and_conquer|
|111|[Minimum Depth of Binary Tree](https://leetcode.com/problems/minimum-depth-of-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_max_min.py)|divide_and_conquer, BFS|
|114|[Flatten Binary Tree to Linked List](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/in_place_flatten_binary_tree_to_linked_list.py)||
|118|[Pascals Triangle](https://leetcode.com/problems/pascals-triangle/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/pascals_triangle.py)||
|119|[Pascals Triangle II](https://leetcode.com/problems/pascals-triangle-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/pascals_triangle.py)||
|120|[Triangle](https://leetcode.com/problems/triangle/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/triangle.py)||
|121|[Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|122|[Best Time to Buy and Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)|
|123|[Best Time to Buy and Sell Stock III](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)|
|124|[Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_maximum_path_sum.py)||
|125|[Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome.py)|two_pointers|
|126|[Word Ladder II](https://leetcode.com/problems/word-ladder-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/word_ladder_2.py)|BFS+DFS|
|127|[Word Ladder](https://leetcode.com/problems/word-ladder/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/word_ladder.py)|双向BFS|
|128|[Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/longest_consecutive_sequence.py)|并查集|
|129|[Sum Root to Leaf Numbers](https://leetcode.com/problems/sum-root-to-leaf-numbers/)|[Rust](src/binary_tree/sum_root_to_leaf_numbers.rs)||
|133|[Clone Graph](https://leetcode.com/problems/clone-graphs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/clone_graph.py)|DFS, BFS|
|134|[Gas Station](https://leetcode.com/problems/gas-station/)|[Rust](src/greedy/gas_station.rs)|
|136|[Single Number](https://leetcode.com/problems/single-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/xor_find_single.py)||
|138|[Copy List with Random Pointer](https://leetcode.com/problems/copy-list-with-random-pointer/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/clone_linked_list_with_random_ptr.py)||
|139|[Word Break](https://leetcode.com/problems/word-break/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_word_break_coin_change.py)|完全背包问题|
|140|[Word Break II](https://leetcode.com/problems/word-break-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/word_break_2.py)||
|141|[Linked List Cycle](https://leetcode.com/problems/linked-list-cycle/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/linked_list_cycle.py)|
|142|[Linked List Cycle II](https://leetcode.com/problems/linked-list-cycle-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/linked_list_cycle.py)|
|144|[Binary Tree Preorder Traversal](https://leetcode.com/problems/binary-tree-preorder-traversal/)|[Rust](src/binary_tree/preorder_traversal.rs)|
|145|[Binary Tree Postorder Traversal](https://leetcode.com/problems/binary-tree-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|146|[LRU Cache](https://leetcode.com/problems/lru-cache/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/lru_cache.py)|double_linked_list, OrderedDict|
|147|[Insertion Sort List](https://leetcode.com/problems/insertion-sort-list/)|[Rust](src/linked_list/insertion_sort_linked_list.rs)|
|148|[Sort List](https://leetcode.com/problems/sort-list/)|[Rust](src/linked_list/merge_two_sorted_linked_list.rs)|
|151|[Reverse Words in a String](https://leetcode.com/problems/reverse-words-in-a-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/reverse_words_in_a_string.py)||
|153|[Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search.py)|binary_search|
|154|[Find Minimum in Rotated Sorted Array II](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_min_2_with_duplicate.py)|binary_search|
|155|[Min Stack](https://leetcode.com/problems/min-stack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/min_stack.py)|
|160|[Intersection of Two Linked Lists](https://leetcode.com/problems/intersection-of-two-linked-lists/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/linked_list_cycle.py)|
|164|[Maximum Gap](https://leetcode.com/problems/maximum-gap/)|[Rust](src/easy/very_easy.rs)|
|167|[Two Sum II - Input array is sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_2_input_is_sorted.py)|two_pointers|
|169|[Majority Element](https://leetcode.com/problems/majority-element/)|[Rust](src/easy/majority_element.rs)||
|170🔒|[Two Sum III - Data structure design](https://leetcode.com/problems/two-sum-iii-data-structure-design/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_3_impl.py)|two_pointers|
|172|[Factorial Trailing Zeroes](https://leetcode.com/problems/factorial-trailing-zeroes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/factorial_trailing_zeroes.py)|factorial|
|173|[Binary Search Tree Iterator](https://leetcode.com/problems/binary-search-tree-iterator/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_in_order_iterator.py)||
|175|[Combine Two Tables](https://leetcode.com/problems/combine-two-tables/)|
|176|[Second Highest Salary](https://leetcode.com/problems/second-highest-salary/)|[SQL](https://github.com/pymongo/python_leetcode/blob/master/database_problems/second_highest_salary.sql)||
|181|[Employees Earning More Than Their Managers](https://leetcode.com/problems/employees-earning-more-than-their-managers/)|[SQL](sql_problems/very_easy_sql_problems.sql)|
|182|[Duplicate Emails](https://leetcode.com/problems/duplicate-emails/)|[SQL](https://github.com/pymongo/python_leetcode/blob/master/database_problems/duplicate_emails.sql)|
|183|[Customers Who Never Order](https://leetcode.com/problems/customers-who-never-order/)|||
|188|[Best Time to Buy and Sell Stock IV](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|189|[Rotate Array](http://leetcode.com/problems/rotate-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/rotate_array_right_circle_shift_elements.py)|reverse, circle_shift|
|190|[Reverse Bits](https://leetcode.com/problems/reverse-bits/)|[Rust](src/easy/bitwise.rs)|
|191|[Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/)|[Rust](src/easy/bitwise.rs)|
|198|[House Robber](https://leetcode.com/problems/house-robber/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/house_robber.py)||
|200|[Number of Islands](https://leetcode.com/problems/number-of-islands/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/number_of_islands.py)|union_find, DFS, BFS|
|202|[Happy Number](https://leetcode.com/problems/happy-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/happy_number.py)|
|203|[Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/remove_linked_list_element.py)|
|204|[Count Primes](https://leetcode.com/problems/count-primes/)|[Rust](src/easy/count_primes.rs)|
|206|[Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/reverse_linked_list.py), [Rust](src/linked_list/reverse_linked_list.rs)||
|208|[Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/tire/impl_tire_prefix_tree.py)|前缀树/字典树|
|207|[Course Schedule](https://leetcode.com/problems/course-schedule/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/course_schedule_2.py)|BFS, topological_sorting|
|209|[Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/minimun_size_subarray_sum.py)|sliding_window|
|210|[Course Schedule II](https://leetcode.com/problems/course-schedule-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/course_schedule_2.py)|BFS, topological_sorting|
|212|[Word Search II](https://leetcode.com/problems/word-search-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/word_search_2.py)|前缀树|
|213|[House Robber II](https://leetcode.com/problems/house-robber-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/house_robber.py)||
|215|[Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)|quick_select, quick_sort, heap|
|216|[Combination Sum III](https://leetcode.com/problems/combination-sum-iii/)|[Rust](src/backtracking/combination_sum_3.rs)|
|217|[Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)|[Rust](src/easy/very_easy.rs)|
|219|[Contains Duplicate II](https://leetcode.com/problems/contains-duplicate-ii/)|[Rust](src/easy/very_easy.rs)|
|222|[Count Complete Tree Nodes](https://leetcode.com/problems/count-complete-tree-nodes/)|[Rust](src/binary_tree/level_order_traversal.rs)|
|225|[Implement Stack using Queues](https://leetcode.com/problems/implement-stack-using-queues/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_stack_by_queue.py)|
|226|[Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/invert_binary_tree.py), [Rust](src/linked_list/reverse_linked_list.rs)||
|227|[Basic Calculator II](https://leetcode.com/problems/basic-calculator/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/calculator_2.py)||
|230|[Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|231|[Power of Two](https://leetcode.com/problems/power-of-two/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_2.py)|bitwise, dichotomy|
|232|[Implement Queue using Stacks](https://leetcode.com/problems/implement-queue-using-stacks/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_stack_by_queue.py)||
|234|[Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/)|[Rust](src/linked_list/linked_list_is_palindrome.rs)|
|235|[Lowest Common Ancestor of a Binary Search Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_lowest_common_ancestor.py)||
|236|[Lowest Common Ancestor of a Binary Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)|divide_and_conquer|
|237|[Delete Node in a Linked List](https://leetcode.com/problems/delete-node-in-a-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/very_easy.py)|
|239|[Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/)|[Rust](src/special_data_structure/monotonic_queue_sliding_window_max.rs)|
|240|[Search a 2D Matrix II](https://leetcode.com/problems/search-a-2d-matrix-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/search_a_2d_matrix_2.py)|
|242|[Valid Anagram](https://leetcode.com/problems/valid-anagram/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/valid_anagram.py)||
|257|[Binary Tree Paths](https://leetcode.com/problems/binary-tree-paths/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/root_to_leaf_paths_find_all.py)|DFS, backtracking|
|263|[Ugly Number](https://leetcode.com/problems/ugly-number/)|[Rust](src/easy/very_easy.rs)|
|264|[Ugly Number II](https://leetcode.com/problems/ugly-number-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/ugly_number_2_nth_ugly.py)|
|266🔒|[Palindrome Permutation](https://leetcode.com/problems/palindrome-permutation/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/palindrome_permutation.py)|greedy|
|269🔒|[Alien Dictionary](https://leetcode.com/problems/alien-dictionary/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/alien_dictionary.py)|heapq, topological_sorting|
|270🔒|[Closest Binary Search Tree Value](https://leetcode.com/problems/closest-binary-search-tree-value/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value.py)||
|272🔒|[Closest Binary Search Tree Value II](https://leetcode.com/problems/closest-binary-search-tree-value-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value_2.py)||
|278|[First Bad Version](https://leetcode.com/problems/first-bad-version/)|[Rust](src/easy/very_easy.rs)|
|279|[Perfect Squares](https://leetcode.com/problems/perfect-squares/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)|完全背包问题|
|283|[Move Zeros](https://leetcode.com/problems/move-zeroes/)|[Rust](src/easy/array/partition_array.rs)|
|287|[Find the Duplicate Number](https://leetcode.com/problems/find-the-duplicate-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/find_duplicate_number.py)|快慢双指针|
|290|[Word Pattern](https://leetcode.com/problems/word-pattern/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/word_pattern.py)||
|291🔒|[Word Pattern II](https://leetcode.com/problems/word-pattern-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/word_pattern_2.py)|DFS|
|292|[Nim Game](https://leetcode.com/problems/nim-game/)|[Rust](src/easy/bitwise.rs)|
|297|[Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree.py)|serialize|
|300|[Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/longest_increase_subsequence.py)|接龙型动态规划|
|302🔒|[Smallest Rectangle Enclosing Black Pixels](https://leetcode.com/problems/smallest-rectangle-enclosing-black-pixels)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/smallest_rectangle_enclosing_black_pixels.py)||
|303|[Range Sum Query - Immutable](https://leetcode.com/problems/range-sum-query-immutable/)|[Rust](src/easy/very_easy.rs)|
|307|[Range Sum Query - Mutable](https://leetcode.com/problems/range-sum-query-mutable/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/segment_tree/segment_tree_range_sum.py)|
|309|[Best Time to Buy and Sell Stock with Cooldown](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|311🔒|[Sparse Matrix Multiplication](https://leetcode.com/problems/sparse-matrix-multiplication/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/sparse_matrix_multiplication.py)||
|312|[Burst Balloons](https://leetcode.com/problems/burst-balloons/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/burst_ballon.py)||
|322|[Coin Change](https://leetcode.com/problem/coin-change/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)|完全背包问题|
|326|[Power of Three](https://leetcode.com/problems/power-of-three/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_3.py)|bitwise|
|328|[Odd Even Linked List](https://leetcode.com/problems/odd-even-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/odd_even_linked_list.py)|
|337|[House Robber III](https://leetcode.com/problems/house-robber-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/house_robber.py)||
|342|[Power of Four](https://leetcode.com/problems/power-of-four/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_4.py)|bitwise|
|343|[Integer Break](https://leetcode.com/problems/integer-break/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/integer_break.py)|划分类DP|
|344|[Reverse Array](https://leetcode.com/problems/reverse-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/reverse_string.py)||
|347|[Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/)|[Rust](src/easy/counter.rs)|
|349|[Intersection of Two Arrays](https://leetcode.com/problems/intersection-of-two-arrays/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/intersection_of_two_arrays.py)||
|350|[Intersection of Two Arrays II](https://leetcode.com/problems/intersection-of-two-arrays-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/intersection_of_two_arrays.py)||
|354|[Russian Doll Envelopes](https://leetcode.com/problems/russian-doll-envelopes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/russian_doll_envelopes.py)||
|367|[Valid Perfect Square](https://leetcode.com/problems/valid-perfect-square/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/sqrt.py)||
|368|[Largest Divisible Subset](https://leetcode.com/problems/largest-divisible-subset/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/largest_divisible_subset.py)||
|369🔒|[Plus One Linked List](https://leetcode.com/problems/plus-one-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/bottom_up_plus_one_linked_list.py)||
|371❌|[Sum of Two Integers](https://leetcode.com/problems/sum-of-two-integers/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/binary_addition.py)|binary_addition|
|377|[Combination Sum IV](https://leetcode.com/problems/combination-sum-iv/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)|完全背包问题|
|380|[Insert Delete GetRandom O(1)](https://leetcode.com/problems/insert-delete-getrandom-o1/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/insert_delete_get_random_o1.py)||
|381|[Insert Delete GetRandom O(1) - Duplicates allowed](https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/insert_delete_get_random_o1.py)||
|387|[First Unique Character in a String](https://leetcode.com/problems/first-unique-character-in-a-string/)|[Rust](src/easy/counter.rs)|
|398|[Random Pick Index](https://leetcode.com/problems/random-pick-index/)|[Rust](src/easy/very_easy.rs)|
|404|[Sum of Left Leaves](https://leetcode.com/problems/sum-of-left-leaves/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/sum_of_left_leaves.py)||
|406|[Queue Reconstruction by Height](https://leetcode.com/problems/queue-reconstruction-by-height/)|[Rust](src/easy/very_easy.rs)|
|409|[Longest Palindrome](https://leetcode.com/problems/longest-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_combination.py)|dp(greedy)|
|412|[Fizz Buzz](https://leetcode.com/problems/fizz-buzz/)|[Rust](src/easy/very_easy.rs)|
|413|[Arithmetic Slices](https://leetcode.com/problems/arithmetic-slices/)|[Rust](src/easy/very_easy.rs)|
|415|[Add String](https://leetcode.com/problems/add-strings/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/easy/add_bigint_string.cpp)||
|416|[Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)|0-1背包问题|
|426|[Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/longest_repeating_character_replacement.py)||
|429|[N-ary Tree Level Order Traversal](https://leetcode.com/problems/n-ary-tree-level-order-traversal/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/bfs/n_ary_tree_level_order.cpp)||
|449|[Serialize and Deserialize BST](https://leetcode.com/problems/serialize-and-deserialize-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_serialize.py)|DFS, stack|
|454|[4Sum II](https://leetcode.com/problems/4sum-ii/)|[Rust](src/easy/very_easy.rs)|
|461|[Hamming Distance](https://leetcode.com/problems/hamming-distance/)|[Rust](src/easy/bitwise.rs)|
|463|[Island Perimeter](https://leetcode.com/problems/island-perimeter/)|[Rust](src/easy/very_easy.rs)|
|474|[Ones and Zeroes](https://leetcode.com/problems/ones-and-zeroes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_ones_and_zeros.py)|完全背包问题|
|485|[Max Consecutive Ones](https://leetcode.com/problems/max-consecutive-ones/)|[Rust](src/easy/very_easy.rs)|
|486|[Validate IP Address](https://leetcode.com/problems/validate-ip-address/)|[Rust](src/easy/string/parse_ip_address.rs)|
|490🔒|[The Maze](https://leetcode.com/problems/the-maze/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/the_maze.py)|
|494|[Target Sum](https://leetcode.com/problems/target-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_target_sum.py)|0-1背包问题|
|498|[Diagonal Traverse](https://leetcode.com/problems/diagonal-traverse/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/diagonal_traverse.py)|
|503|[Next Greater Element II](https://leetcode.com/problems/next-greater-element-ii/)|[Rust](src/special_data_structure/monotonic_stack_next_greater_element_2.rs)|
|507|[Perfect Number](https://leetcode.com/problems/perfect-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/perfect_number.py)||
|509|[Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)|[Rust](src/easy/very_easy.rs)|
|514|[Freedom Trail](https://leetcode.com/problems/freedom-trail/)|[Rust](src/dp/freedom_trail.rs)|
|518|[Coin Change 2](https://leetcode.com/problem/coin-change-2/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)|完全背包问题|
|535|[Encode and Decode TinyURL](https://leetcode.com/problems/encode-and-decode-tinyurl/)|||
|536🔒|[Construct Binary Tree from String](https://leetcode.com/problems/construct-binary-tree-from-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_string_with_parentheses.py)||
|538|[Convert BST to Greater Tree](https://leetcode.com/problems/convert-bst-to-greater-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_to_gst.py)||
|547|[Friend Circles](https://leetcode.com/problems/friend-circles/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/friend_circles.py)|union_find, BFS, BFS|
|559|[Maximum Depth of N-ary Tree](https://leetcode.com/problems/maximum-depth-of-n-ary-tree/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/bfs/n_ary_tree_level_order.cpp)||
|567|[Reverse Words in a String III](https://leetcode.com/problems/reverse-words-in-a-string-iii/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/easy/reverse_words_in_a_string_3.cpp)||
|589|[N-ary Tree Preorder Traversal](https://leetcode.com/problems/n-ary-tree-preorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/n_ary_tree_preorder_postorder.py)||
|590|[N-ary Tree Postorder Traversal](https://leetcode.com/problems/n-ary-tree-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/n_ary_tree_preorder_postorder.py)||
|595|[Big Countries](https://leetcode.com/problems/big-countries/)|||
|606|[Construct String from Binary Tree](https://leetcode.com/problems/construct-string-from-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_string_with_parentheses.py)||
|617|[Merge Two Binary Trees](https://leetcode.com/problems/merge-two-binary-trees/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/merge_two_binary_tree.py)||
|620|[Not Boring Movies](https://leetcode.com/problems/not-boring-movies/)|||
|625🔒|[Minimum Factorization](https://leetcode.com/problems/minimum-factorization/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/minimum_factorization.py)|greedy|
|627|[Swap Salary](https://leetcode.com/problems/swap-salary/)|[SQL](https://github.com/pymongo/python_leetcode/blob/master/database_problems/invert_sex.sql)||
|633|[Average of Levels in Binary Tree](https://leetcode.com/problems/average-of-levels-in-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_level_order.py)||
|650|[2 Keys Keyboard](https://leetcode.com/problems/2-keys-keyboard/)|[Rust](src/easy/very_easy.rs)|
|653|[First Unique Number in Data Stream](https://leetcode.com/problems/two-sum-iv-input-is-a-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_two_sum.py)||
|658|[Find K Closest Elements](https://leetcode.com/problems/find-k-closest-elements/)|[Rust](src/easy/very_easy.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/find_k_closest_elements.py)|binary_search|
|669|[Trim A Binary Search Tree](https://leetcode.com/problems/trim-a-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_trim_in_range.py)||
|674|[Longest Continuous Increasing Subsequence](https://leetcode.com/problems/longest-continuous-increasing-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/longest_continuous_increase_subsequence.py)||
|680|[Valid Palindrome II](https://leetcode.com/problems/valid-palindrome-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome_2.py)|two_pointers, greedy|
|695|[Max Area of Island](https://leetcode.com/problems/max-area-of-island/)|[Rust](src/bfs/max_area_of_island.rs)|
|696|[Count Binary Substrings](https://leetcode.com/problems/count-binary-substrings/)|[Rust](src/easy/very_easy.rs)|
|700|[Search in a Binary Search Tree](https://leetcode.com/problems/search-in-a-binary-search-tree/)|[Rust](src/binary_tree/search_val_or_range_in_bst.rs)|
|702🔒|[Search in a Sorted Array of Unknown Size](https://leetcode.com/problems/search-in-a-sorted-array-of-unknown-size/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search_unknown_size_sorted_array.py)|binary_search_first, 倍增法|
|703|[Kth Largest Element in a Stream](https://leetcode.com/problems/kth-largest-element-in-a-stream/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/top_k_largest_stream_min_heap.py)|min_heap|
|704|[Binary Search](https://leetcode.com/problems/binary-search/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search.py)|binary_search|
|705|[Design HashSet](https://leetcode.com/problems/design-hashset/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/impl_hashmap.py)||
|706|[Design HashMap](https://leetcode.com/problems/design-hashmap/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/impl_hashmap.py)||
|709|[To Lower Case](https://leetcode.com/problems/to-lower-case/)|[Rust](src/easy/very_easy.rs)|
|713|[1-bit and 2-bit Characters](https://leetcode.com/problems/1-bit-and-2-bit-characters/)|[Rust](src/easy/one_or_two_bit_char.rs)||
|714|[Best Time to Buy and Sell Stock with Transaction Fee](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|716🔒|[Max Stack](https://leetcode.com/problems/max_stack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/max_stack.py)||
|728|[Self Dividing Numbers](https://leetcode.com/problems/self-dividing-numbers/)|[Rust](src/easy/very_easy.rs)|
|760🔒|[Find Anagram Mappings](https://leetcode.com/problems/find-anagram-mappings/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/mapping_two_anagram_list_int.py)||
|796|[Rotate String](https://leetcode.com/problems/rotate-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/rotate_string.py)|Rabin-Karp(rolling_hash), kmp|
|743|[Network Delay Time](https://leetcode.com/problems/network-delay-time/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/network_delay_time_dijkstra.py)|
|763|[Partition Labels](https://leetcode.com/problems/partition-labels/)|[Rust](src/greedy/partition_labels.rs)|
|771|[Jewels and Stones](https://leetcode.com/problems/jewels-and-stones/)|[Rust](src/easy/very_easy.rs)|
|807|[Max Increase to Keep City Skyline](https://leetcode.com/problems/max-increase-to-keep-city-skyline/)|[Rust](src/easy/very_easy.rs)|
|832|[Flipping an Image](https://leetcode.com/problems/flipping-an-image/)|[Rust](src/easy/very_easy.rs)|
|841|[Keys and Rooms](https://leetcode.com/problems/keys-and-rooms/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/bfs/keys_and_rooms.cpp)||
|844|[Backspace String Compare](https://leetcode.com/problems/backspace-string-compare/)|[Rust](src/easy/string/backspace_string_compare.rs)||
|845|[Longest Mountain in Array](https://leetcode.com/problems/longest-mountain-in-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_longest.py)|mountain_array|
|852|[Peak Index in a Mountain Array](https://leetcode.com/problems/peak-index-in-a-mountain-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_max.py)|mountain_array, binary_search|
|860|[Lemonade Change](https://leetcode.com/problems/lemonade-change/)|[Rust](src/easy/very_easy.rs)|
|861|[Score After Flipping Matrix](https://leetcode.com/problems/score-after-flipping-matrix/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/score_after_flipping_matrix.py)|greedy|
|867|[Transpose Matrix](https://leetcode.com/problems/transpose-matrix/)|[Rust](src/easy/very_easy.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/easy/transpose_matrix_test.go), [C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/easy/transpose_matrix.cpp)|
|869|[Reordered Power of 2](https://leetcode.com/problems/reordered-power-of-2/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/reordered_power_of_2.py)|permutation|
|875|[Koko Eating Bananas](https://leetcode.com/problems/koko-eating-bananas/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/koko_eating_bananas.py)||
|876|[Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/middle_of_linked_list.py), [Rust](src/linked_list/middle_of_linked_list.rs)|快慢双指针|
|887|[Super Egg Drop](https://leetcode.com/problems/super-egg-drop)|[Rust](src/dp/drop_eggs.rs)|
|889|[Construct Binary Tree from Preorder and Postorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_pre_order_and_post_order.py)|DFS|
|905|[Sort Array By Parity](https://leetcode.com/problems/sort-array-by-parity/)|[Rust](src/easy/array/partition_array.rs)|
|912|[Sort An Array](https://leetcode.com/problems/sort-an-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/basic_sorting.py)|
|922|[Sort Array By Parity II](https://leetcode.com/problems/sort-array-by-parity-ii/)|[Rust](src/easy/array/partition_array.rs)|
|925|[Long Pressed Name](https://leetcode.com/problems/long-pressed-name/)|[Rust](src/easy/string/long_pressed_name.rs)||
|938|[Range Sum of BST](https://leetcode.com/problems/range-sum-of-bst/)|[Rust](src/binary_tree/search_val_or_range_in_bst.rs)|
|941|[Valid Mountain Array](https://leetcode.com/problems/find-in-mountain-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_valid.py)|mountain_array|
|950|[Reveal Cards In Increasing Order](https://leetcode.com/problems/reveal-cards-in-increasing-order/)|[Rust](src/easy/very_easy.rs)|
|953|[Verifying an Alien Dictionary](https://leetcode.com/problems/verifying-an-alien-dictionary/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/verifying_an_alien_dictionary.py)|
|973|[K Closest Points to Origin](https://leetcode.com/problems/k-closest-points-to-origin/)|[Rust](src/easy/very_easy.rs)|quick_select|
|976|[Largest Perimeter Triangle](https://leetcode.com/problems/largest-perimeter-triangle/)|[Rust](src/easy/very_easy.rs)|
|977|[Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/)|[Rust](src/easy/array/squares_of_a_sorted_array.rs)|
|1002|[Find Common Characters](https://leetcode.com/problems/find-common-characters/)|[Rust](src/easy/find_common_characters.rs)|
|1030|[Matrix Cells in Distance Order](https://leetcode.com/problems/matrix-cells-in-distance-order/)|[Rust](src/easy/very_easy.rs)|
|1038|[Binary Search Tree to Greater Sum Tree](https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_to_gst.py)||
|1046|[Last Stone Weight](https://leetcode.com/problems/last-stone-weight/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/last_stone_weight.py)||
|1049|[Last Stone Weight II](https://leetcode.com/problems/last-stone-weight-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)|0-1背包问题|
|1051|[Height Checker](https://leetcode.com/problems/height-checker/)|[Rust](src/easy/very_easy.rs)|
|1095|[Find in Mountain Array](https://leetcode.com/problems/find-in-mountain-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_find.py)|binary_search, mountain_array|
|1099🔒|[Two Sum Less Than K](https://leetcode.com/problems/two-sum-less-than-k/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_le_count.py)|two_pointers|
|1108|[Defanging an IP Address](https://leetcode.com/problems/defanging-an-ip-address/)|[Rust](src/easy/very_easy.rs)|
|1109|[Corporate Flight Bookings](https://leetcode.com/problems/corporate-flight-bookings/)|[Rust](src/easy/corporate_flight_bookings.rs)||
|1143|[Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/longest_common_subsequence.py)||
|1147|[Work Plan](https://lintcode.com/problem/work-plan/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/house_robber.py)||
|1167🔒|[Longest Common Subsequence](https://leetcode.com/problems/minimum-cost-to-connect-sticks/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/minimum_cost_to_connect_sticks.py)||
|1207|[Unique Number of Occurrences](https://leetcode.com/problems/unique-number-of-occurrences/)|[Rust](src/easy/counter.rs)|
|1213🔒|[Intersection of Three Sorted Arrays](https://leetcode.com/problems/intersection-of-three-sorted-arrays/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/intersection_of_two_arrays.py)||
|1226|[The Dining Philosophers](https://leetcode.com/problems/the-dining-philosophers/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/concurrency/dining_philosophers_problem.cpp)||
|1227|[Airplane Seat Assignment Probability](https://leetcode.com/problems/airplane-seat-assignment-probability/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/airplane_seat_assignment_probability.py)||
|1252|[Cells with Odd Values in a Matrix](https://leetcode.com/problems/cells-with-odd-values-in-a-matrix/)|[Rust](src/easy/very_easy.rs)|
|1266|[Minimum Time Visiting All Points](https://leetcode.com/problems/minimum-time-visiting-all-points/)|[Rust](src/easy/very_easy.rs)|
|1281|[Subtract the Product and Sum of Digits of an Integer](https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/)|[Python](src/easy/very_easy.rs)|
|1295|[Find Numbers with Even Number of Digits](https://leetcode.com/problems/find-numbers-with-even-number-of-digits/)|[Rust](src/easy/array/find_numbers_with_even_number_of_digits.rs)|
|1299|[Replace Elements with Greatest Element on Right Side](https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/)|[Rust](src/easy/very_easy.rs)|
|1313|[Decompress Run-Length Encoded List](https://leetcode.com/problems/decompress-run-length-encoded-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/decompress_run_length_encoded_list.py)|
|1329|[Sort The Matrix Diagonally](https://leetcode.com/problems/sort-the-matrix-diagonally/)|[Rust](src/easy/matrix_diagonal_traverse.rs)|
|1342|[Number of Steps to Reduce a Number to Zero](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/number_of_steps_to_reduce_a_number_to_zero.py)|
|1351|[Count Negative Numbers in a Sorted Matrix](https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/)|[Rust](src/easy/count_negative_numbers_in_a_sorted_matrix.rs)|
|1356|[Sort Integers by The Number of 1 Bits](https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/)|[Rust](src/easy/very_easy.rs)|
|1365|[How Many Numbers Are Smaller Than the Current Number](https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/how_many_numbers_are_smaller_than_current.py)|[Rust](src/easy/array/how_many_numbers_are_smaller_than_the_current_number.rs)|前缀和|
|1370|[Increasing Decreasing String](https://leetcode.com/problems/increasing-decreasing-string/)|[Rust](src/easy/counter.rs)|
|1374|[Generate a String With Characters That Have Odd Counts](https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/)|[Rust](src/easy/counter.rs)|
|1379|[Find a Corresponding Node of a Binary Tree in a Clone of That Tree](https://leetcode.com/problems/find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/find_a_corresponding_node_in_a_cloned_tree.py)||
|1389|[Create Target Array in the Given Order](https://leetcode.com/problems/create-target-array-in-the-given-order/)|[Rust](src/easy/very_easy.rs)|
|1395|[Count Number of Teams](https://leetcode.com/problems/count-number-of-teams/)|[Rust](src/easy/count_number_of_teams.rs)||
|1409|[Queries on a Permutation With Key](https://leetcode.com/problems/queries-on-a-permutation-with-key/)|[Rust](src/easy/array/queries_on_a_permutation_with_key.rs)||
|1429🔒|[First Unique Number](https://leetcode.com/problems/first-unique-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/first_unique_number_in_stream.py)|
|1431|[Kids With the Greatest Number of Candies](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/)|[Rust](src/easy/very_easy.rs)|
|1450|[Number of Students Doing Homework at a Given Time](https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/)|[Rust](src/easy/very_easy.rs)|
|1464|[Maximum Product of Two Elements in an Array](https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/)|[Rust](src/easy/very_easy.rs)|
|1470|[Shuffle the Array](https://leetcode.com/problems/shuffle-the-array/)|[Rust](src/easy/very_easy.rs)|
|1476|[Subrectangle Queries](https://leetcode.com/problems/subrectangle-queries/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/subrectangle_queries.py)||
|1480|[Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array/)|[Rust](src/easy/very_easy.rs)|
|1486|[XOR Operation in an Array](https://leetcode.com/problems/xor-operation-in-an-array/)|[Rust](src/easy/very_easy.rs)|
|1502|[Can Make Arithmetic Progression From Sequence](https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/)|[Rust](src/easy/very_easy.rs)|
|1512|[Number of Good Pairs](https://leetcode.com/problems/number-of-good-pairs/)|[Rust](src/easy/counter.rs)|
|1528|[Shuffle String](https://leetcode.com/problems/shuffle-string/)|[Rust](src/easy/very_easy.rs)|
|1534|[Count Good Triplets](https://leetcode.com/problems/count-good-triplets/)|[Rust](src/easy/array/count_good_triplets.rs)|
|1551|[Minimum Operations to Make Array Equal](https://leetcode.com/problems/minimum-operations-to-make-array-equal/)|[Rust](src/easy/very_easy.rs)|
|1572|[Matrix Diagonal Sum](https://leetcode.com/problems/matrix-diagonal-sum/)|[Rust](src/easy/very_easy.rs)|
|1576|[Replace All ?'s to Avoid Consecutive Repeating Characters](https://leetcode.com/problems/replace-all-s-to-avoid-consecutive-repeating-characters/)|[Rust](src/easy/very_easy.rs)|
|1577|[Number of Ways Where Square of Number Is Equal to Product of Two Numbers](https://leetcode.com/problems/number-of-ways-where-square-of-number-is-equal-to-product-of-two-numbers/)|[Rust](src/easy/number_of_ways_where_square_of_number_is_equal_to_product_of_two_numbers.rs)|
|1578|[Minimum Deletion Cost to Avoid Repeating Letters](https://leetcode.com/problems/minimum-deletion-cost-to-avoid-repeating-letters/)|[Rust](src/easy/very_easy.rs)|
|1582|[Number of Ways Where Square of Number Is Equal to Product of Two Numbers](https://leetcode.com/problems/special-positions-in-a-binary-matrix/)|[Rust](src/easy/special_positions_in_a_binary_matrix.rs)|
|1584|[Min Cost to Connect All Points](https://leetcode.com/problems/min-cost-to-connect-all-points/)|[Rust](src/union_find/min_cost_to_connect_all_points.rs)|
|1588|[Sum of All Odd Length Subarrays](https://leetcode.com/problems/sum-of-all-odd-length-subarrays/)|[Rust](src/easy/array/sum_of_all_odd_length_subarrays.rs)|
|1601|[Design Parking System](https://leetcode.com/problems/design-parking-system/)|[Rust](src/easy/very_easy.rs)|
|1614|[Maximum Nesting Depth of the Parentheses](https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/)|[Rust](src/easy/very_easy.rs)|
|1637|[Widest Vertical Area Between Two Points Containing No Points](https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/)|[Rust](src/easy/very_easy.rs)|
|1640|[Check Array Formation Through Concatenation](https://leetcode.com/problems/check-array-formation-through-concatenation/)|[Rust](src/easy/very_easy.rs)|
|1656|[Design an Ordered Stream](https://leetcode.com/problems/design-an-ordered-stream/)|[Rust](src/easy/very_easy.rs)|
|1672|[Richest Customer Wealth](https://leetcode.com/problems/richest-customer-wealth/)|[Rust](src/easy/very_easy.rs)|
|1678|[Goal Parser Interpretation](https://leetcode.com/problems/goal-parser-interpretation/)|[Rust](src/easy/very_easy.rs)|

---

剑指Offer
========

| # | Title | Solutions | Category |
|---| ----- | -------- | ---------- |
|59|[队列的最大值](https://leetcode.com/problems/dui-lie-de-zui-da-zhi-lcof/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/max_queue.py)||
|60|[n个骰子的点数](https://leetcode.com/problems/nge-tou-zi-de-dian-shu-lcof/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/dices_sum.py)||
|61|[扑克牌中的顺子](https://leetcode.com/problems/bu-ke-pai-zhong-de-shun-zi-lcof/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/straight_in_playing_cards.py)||
|62|[圆圈中最后剩下的数字](https://leetcode.com/problems/yuan-quan-zhong-zui-hou-sheng-xia-de-shu-zi-lcof/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/josephus.py)|约瑟夫环|
|66|[构建乘积数组](https://leetcode.com/problems/gou-jian-cheng-ji-shu-zu-lcof)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/contruct_product_array.py), [Rust](src/easy/construct_product_array.rs)||

---

TODO:

- [ ] Rust的leetcode/codeforces都不支持OnceCell API，但是罗马数字转阿拉伯需要用lazy_static的HashMap，所以能不能「复制粘贴」一份OnceCell的代码试试手，顺便理解源码
- [ ] 43/415 字符串整数相乘/相加
- [ ] 415是经典面试题利用字符串模拟竖式加法进行大数相加
- [ ] 有兴趣但是困难级别的题: 1. 推箱子 2. 华容道

有意思的但是没有找到题号的问题:

1. 海盗分金币问题(比较脑筋急转弯的递推)
2. 最大直方图矩阵(Longest Histogram Rectangle), 单调栈O(n^3)->O(n)

---

lintcode_problems
===

| # | Title | Solutions | Category |
|---| ----- | --------- | -------- |
|1|[A + B Problem](https://lintcode.com/problem/a-b-problem/)|leetcode_372|
|2|[Trailing Zeros](https://lintcode.com/problem/trailing-zeros/)|leetcode_172|
|3|[Digit Counts](https://lintcode.com/problem/digit-counts/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/very_easy.py)|
|4|[Ugly Number II](https://lintcode.com/problem/ugly-number-ii/)|leetcode_264|
|5|[Kth Largest Element](https://lintcode.com/problem/kth-largest-element/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)|quick_select, heap|
|6|[Merge Two Sorted Arrays](https://lintcode.com/problem/merge-two-sorted-arrays/)|leetcode_88|
|7|[Serialize and Deserialize Binary Tree](https://lintcode.com/problem/serialize-and-deserialize-binary-tree/)|leetcode_297|
|8|[Rotate String](https://lintcode.com/problem/rotate-string/)|leetcode_796|
|9|[Fizz Buzz](https://lintcode.com/problem/fizz-buzz/)|leetcode_412|
|10|[String Permutation II](https://lintcode.com/problem/string-permutation-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/permutation.py)|backtracking|
|11|[Search Range in Binary Search Tree](https://lintcode.com/problem/search-range-in-binary-search-tree/)|leetcode_938|
|12|[Min Stack](https://lintcode.com/problem/min-stack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/min_stack.py)||
|13|[Implement strStr()](https://lintcode.com/problem/implement-strstr/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/string/find_substr.py)|kmp, Rabin-Karp(rolling_hash)|
|14|[First Position of Target](https://lintcode.com/problem/first-position-of-target/)|leetcode_34|
|15|[Permutations](https://lintcode.com/problem/permutations/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/permutation.py)|backtracking|
|16|[Permutations II](https://lintcode.com/problem/permutations-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/permutation.py)|backtracking|
|17|[Subsets](https://lintcode.com/problem/subsets/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/subsets_combinations.py)||
|18|[Subsets II](https://lintcode.com/problem/subsets-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/subsets_combinations.py)||
|20|[Dices Sum](https://lintcode.com/problem/dices-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/dices_sum.py)||
|22|[Flatten List](https://lintcode.com/problem/flatten-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/flatten_list.py)||
|28|[Search a 2D Matrix](https://lintcode.com/problem/search-a-2d-matrix/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/search_a_2d_matrix.py)||
|31|[Partition Array](https://lintcode.com/problem/partition-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/partition_array.py)|two_pointers|
|33|[N Queens](https://lintcode.com/problem/n-queens)|leetcode_51|
|34|[N Queens II](https://lintcode.com/problem/n-queens-ii)|leetcode_52|
|35|[Reverse Linked List](https://lintcode.com/problem/reverse-linked-list/)|leetcode_206|
|36|[Reverse Linked List II](https://lintcode.com/problem/reverse-linked-list-ii/)|leetcode_92|
|37|[Reverse 3-digit Integer](https://lintcode.com/problem/reverse-3-digit-integer/)|||
|38|[Search a 2D Matrix II](https://lintcode.com/problem/search-a-2d-matrix-ii/)|leetcode_240|
|40|[Implement Queue by Two Stacks](https://lintcode.com/problem/implement-queue-by-two-stacks/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_queue_by_stack.py)||
|41|[Maximum Subarray](https://lintcode.com/problem/maximum-subarray/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/maximum_subarray.py)|greedy, dp|
|46|[Majority Element](https://lintcode.com/problem/majority-element/)|[Rust](src/easy/majority_element.rs)|
|49|[Sort Letters by Case](https://lintcode.com/problem/sort-letters-by-case/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/sort_colors.py)|
|50|[Product of Array Exclude Itself](https://lintcode.com/problem/product-of-array-exclude-itself/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/contruct_product_array.py), [Rust](src/easy/construct_product_array.rs)||
|52|[Next Permutation](https://lintcode.com/problem/next-permutation/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/next_permutation.py)||
|53|[Reverse Words in a String](https://lintcode.com/problem/reverse-words-in-a-string/)|leetcode_151|
|56|[Two Sum](https://lintcode.com/problem/two-sum/)|leetcode_1|
|57|[3Sum](https://lintcode.com/problem/3sum/)|leetcode_15|
|58|[4Sum](https://lintcode.com/problem/4sum/)|leetcode_18|
|59|[3Sum Closest](https://lintcode.com/problem/3sum-closest/)|leetcode_16|
|60|[Search Insert Position](https://lintcode.com/problem/search-insert-position/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/search_insert_position.py)||
|61|[Search for a Range](https://lintcode.com/problem/search-for-a-range/)|leetcode_34|
|62|[Search in Rotated Sorted Array](https://lintcode.com/problem/search-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search.py)|binary_search|
|63|[Search in Rotated Sorted Array II](https://lintcode.com/problem/search-in-rotated-sorted-array-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search_2_with_duplicate.py)|binary_search|
|64|[Merge Sorted Array](https://lintcode.com/problem/merge-sorted-array/)|leetcode_88|
|65|[Median of Two Sorted Arrays](https://lintcode.com/problem/median-of-two-sorted-arrays/)|leetcode_4|
|66|[Binary Tree Preorder Traversal](https://lintcode.com/problem/binary-tree-preorder-traversal/)|leetcode_144|
|67|[Binary Tree Inorder Traversal](https://lintcode.com/problem/binary-tree-inorder-traversal/)|leetcode_94|
|68|[Binary Tree Postorder Traversal](https://lintcode.com/problem/binary-tree-postorder-traversal/)|leetcode_145|
|69|[Binary Tree Level Order Traversal](https://lintcode.com/problem/binary-tree-level-order-traversal/)|leetcode_102|
|70|[Binary Tree Level Order Traversal II](https://lintcode.com/problem/binary-tree-level-order-traversal-ii/)|leetcode_102|
|72|[Construct Binary Tree from Inorder and Postorder Traversal](https://lintcode.com/problem/construct-binary-tree-from-inorder-and-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_in_order_and_post_order.py)|DFS|
|73|[Construct Binary Tree from Preorder and Inorder Traversal](https://lintcode.com/problem/construct-binary-tree-from-preorder-and-inorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_pre_order_and_in_order.py)|DFS, stack|
|74|[First Bad Version](https://lintcode.com/problem/first-bad-version/)|leetcode_278|
|75|[Find Peak Element](https://lintcode.com/problem/find-peak-element/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_max.py)|mountain_array, binary_search|
|76|[Longest Increasing Subsequence](https://lintcode.com/problem/longest-increasing-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/longest_increase_subsequence.py)|接龙型动态规划|
|77|[Longest Common Subsequence](https://lintcode.com/problem/longest-common-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/longest_common_subsequence.py)||
|78|[Longest Common Prefix](https://lintcode.com/problem/longest-common-prefix/)|leetcode_14|
|79|[Longest Common Substring](https://lintcode.com/problem/longest-common-substring/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/longest_common_substring.py)||
|80|[Median](https://lintcode.com/problem/median/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)|quick_select|
|86|[Binary Search Tree Iterator](https://lintcode.com/problem/binary-search-tree-iterator/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_in_order_iterator.py)||
|88|[Lowest Common Ancestor of a Binary Tree](https://lintcode.com/problem/lowest-common-ancestor-of-a-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)|divide_and_conquer|
|90|[k Sum II](https://lintcode.com/problem/k-sum-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/k_sum_2.py)||
|92|[Backpack](https://lintcode.com/problem/backpack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)||
|93|[Balanced Binary Tree](https://lintcode.com/problem/balanced-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_is_balance_binary_tree.py)|divide_and_conquer|
|94|[Binary Tree Maximum Path Sum](https://lintcode.com/problem/binary-tree-maximum-path-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_maximum_path_sum.py)||
|95|[Validate Binary Search Tree](https://lintcode.com/problem/validate-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_valid.py)||
|97|[Maximum Depth of Binary Tree](https://lintcode.com/problem/maximum-depth-of-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_max_min.py)||
|98|[Sort List](https://lintcode.com/problem/sort-list/)|leetcode_148|
|100|[Remove Duplicates from Sorted Array](https://lintcode.com/problem/remove-duplicates-from-sorted-array/)|leetcode_26|
|102|[Linked List Cycle](https://lintcode.com/problem/linked-list-cycle/)|leetcode_141|
|103|[Linked List Cycle II](https://lintcode.com/problem/linked-list-cycle-ii/)|leetcode_142|
|104|[Merge k Sorted Lists](https://lintcode.com/problem/merge-k-sorted-lists/)|leetcode_23|
|105|[Copy List with Random Pointer](https://lintcode.com/problem/copy-list-with-random-pointer/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/clone_linked_list_with_random_ptr.py)||
|106|[Convert Sorted List to Binary Search Tree](https://lintcode.com/problem/convert-sorted-list-to-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/sorted_list_or_array_to_bst.py)|divide_and_conquer|
|107|[Word Break](https://lintcode.com/problem/word-break/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_word_break_coin_change.py)|完全背包问题|
|109|[Triangle](https://lintcode.com/problem/triangle/)|leetcode_120|
|110|[Minimum Path Sum](https://lintcode.com/problem/minimum-path-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/grid_min_path_sum.py)||
|112|[Remove Duplicates from Sorted List](https://lintcode.com/problem/remove-duplicates-from-sorted-list/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/linked_list/remove_duplicates_from_sorted_list.cpp)||
|113|[Remove Duplicates from Sorted List II](https://lintcode.com/problem/remove-duplicates-from-sorted-list-ii/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/linked_list/remove_duplicates_from_sorted_list.cpp)||
|114|[Unique Paths](https://lintcode.com/problem/unique-paths/)|leetcode_62|
|115|[Unique Paths II](https://lintcode.com/problem/unique-paths-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/unique_paths_2.py)||
|116|[Jump Game](https://lintcode.com/problem/jump-game/)|leetcode_55|
|117|[Jump Game II](https://lintcode.com/problem/jump-game-ii/)|[Rust](src/dp/jump_game_2.rs)|greedy|
|119|[Edit Distance](https://lintcode.com/problem/edit-distance/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/edit_distance.py)||
|120|[Word Ladder](https://lintcode.com/problem/word-ladder/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/word_ladder.py)|双向BFS|
|121|[Word Ladder II](https://lintcode.com/problem/word-ladder-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/word_ladder_2.py)|BFS+DFS|
|123|[Word Search](https://lintcode.com/problem/word-search/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/word_search.py)||
|124|[Longest Consecutive Sequence](https://lintcode.com/problem/longest-consecutive-sequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/longest_consecutive_sequence.py)|并查集|
|125|[Backpack II](https://lintcode.com/problem/backpack-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)||
|127|[Topological Sorting](https://lintcode.com/problem/topological-sorting/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/topological_sorting.py)|BFS, topological_sorting|
|128|[Hash Function](https://lintcode.com/problem/hash-function/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/hash_function.py)||
|129|[Rehashing](https://lintcode.com/problem/rehashing/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/rehashing.py)||
|132|[Word Search II](https://lintcode.com/problem/word-search-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/word_search_2.py)|前缀树|
|134|[LRU Cache](https://lintcode.com/problem/lru-cache/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/lru_cache.py)|double_linked_list, OrderedDict|
|135|[Combination Sum](https://lintcode.com/problem/combination-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/combination_target_sum.py), [Rust](src/backtracking/combination_sum_1_2.rs)||
|137|[Clone Graph](https://lintcode.com/problem/clone-graphs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/clone_graph.py)|DFS, BFS|
|138|[Subarray Sum](https://lintcode.com/problem/subarray-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/subarray_sum_zero.py)||
|140|[Fast Power](https://lintcode.com/problem/fast-power/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/pow.py)|binary_search, 快速幂运算|
|141|[Sqrt(x)](https://lintcode.com/problem/sqrtx/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/sqrt.py)|牛顿连续均值求根法|
|142|[O(1) Check Power of 2](https://lintcode.com/problem/o1-check-power-of-2/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_2.py)|bitwise, dichotomy|
|143|[Sort Colors II](https://lintcode.com/problem/sort-colors-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/sort_colors_2.py)|quick_sort, counting_sort|
|144|[interleaving_positive_and_negative_numbers](https://lintcode.com/problem/sort-colors-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/interleaving_positive_and_negative_numbers.py)||
|145|[Lower case to Uppercase](https://lintcode.com/problem/lowercase-to-uppercase/)|leetcode_709|
|147🔒|[Narcissistic Number](https://lintcode.com/problem/narcissistic-number)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/narcissistic_number.py)||
|148|[Sort Colors](https://lintcode.com/problem/sort-colors/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/sort_colors.py)|three_pointers, partition_array|
|149|[Best Time to Buy and Sell Stock](https://lintcode.com/problem/best-time-to-buy-and-sell-stock/)|leetcode_121|
|150|[Best Time to Buy and Sell Stock II](https://lintcode.com/problem/best-time-to-buy-and-sell-stock-ii/)|leetcode_122|
|151|[Best Time to Buy and Sell Stock III](https://lintcode.com/problem/best-time-to-buy-and-sell-stock-iii/)|leetcode_123|
|152|[Combinations](https://lintcode.com/problem/combinartions/)|leetcode_77|
|153|[Combination Sum II](https://lintcode.com/problem/combination-sum-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/combination_target_sum.py), [Rust](src/backtracking/combination_sum_1_2.rs)||
|154|[Regular Expression Matching](https://lintcode.com/problem/regular-expression-matching/)|leetcode_10|
|155|[Minimum Depth of Binary Tree](https://lintcode.com/problem/minimum-depth-of-binary-tree/)|leetcode_111|
|158|[Valid Anagram](https://lintcode.com/problem/valid-anagram/)|leetcode_242|
|159|[Find Minimum in Rotated Sorted Array](https://lintcode.com/problem/find-minimum-in-rotated-sorted-array/)|leetcode_153|
|160|[Find Minimum in Rotated Sorted Array II](https://lintcode.com/problem/find-minimum-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_min_2_with_duplicate.py)|binary_search|
|161|[Rotate Image](https://lintcode.com/problem/rotate-image/)|[Rust](src/easy/grid_or_matrix/rotate_matrix.rs)|
|162|[Set Matrix Zeroes](https://lintcode.com/problem/set-matrix-zeroes/)|leetcode_73|
|165|[Merge Two Sorted Lists](https://lintcode.com/problem/merge-two-sorted-lists/)|leetcode_21|
|167|[Add Two Numbers](https://lintcode.com/problem/add-two-numbers/)|leetcode_2|
|168|[Burst Balloons](https://lintcode.com/problem/burst-balloons/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/burst_ballon.py)||
|171|[Anagrams](https://lintcode.com/problem/anagrams/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/group_anagrams.py)||
|172|[Remove Element](https://lintcode.com/problem/remove-element/)|leetcode_27|
|173|[Insertion Sort List](https://lintcode.com/problem/insertion-sort-list/)|leetcode_147|
|174|[Remove Nth Node From End of List](https://lintcode.com/problem/remove-nth-node-from-end-of-list/)|leetcode_19|
|175|[Invert Binary Tree](https://lintcode.com/problem/invert-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/invert_binary_tree.py), [Rust](src/linked_list/reverse_linked_list.rs)||
|177|[Convert Sorted Array to Binary Search Tree With Minimal Height](https://lintcode.com/problem/convert-sorted-array-to-binary-search-tree-with-minimal-height/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/sorted_list_or_array_to_bst.py)|divide_and_conquer|
|183|[Wood Cut](https://lintcode.com/problem/wood-cut/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/wood_cut.py)|greedy|   
|187|[Gas Station](https://lintcode.com/problem/gas-station/)|leetcode_134|
|190|[Next Permutation II](https://lintcode.com/problem/next-permutation-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/next_permutation.py)||
|192|[Wildcard Matching](https://lintcode.com/problem/wildcard-matching/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/wildcard_matching.py)||
|197|[Permutation Index](https://lintcode.com/problem/permutation-index/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/permutation_index.py)||
|200|[Longest Palindromic Substring](https://lintcode.com/problem/longest-palindromic-substring/)|leetcode_5|
|209|[First Unique Character in a String](https://lintcode.com/problem/first-unique-character-in-a-string/)|leetcode_387|
|221|[String Permutation](https://lintcode.com/problem/string-permutation/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/valid_anagram.py)||
|224_FIXME|[String Permutation](https://lintcode.com/problem/string-permutation/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/valid_anagram.py)|
|228🔒|[Middle of the Linked List](https://lintcode.com/problem/middle-of-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/middle_of_linked_list.py), [Rust](src/linked_list/middle_of_linked_list.rs)|快慢指针|
|235|[Prime Factorization](https://lintcode.com/problem/prime-factorization/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/prime_factorization.py)|分解质因数|
|241🔒|[String to Integer (atoi)](https://lintcode.com/problem/string-to-integer-atoi/)|leetcode_8|
|249|[Count of Smaller Number before itself](https://lintcode.com/problem/count-of-smaller-number-before-itself/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/segment_tree/count_of_smaller_number_before_itself.py)|sqrt_n|
|254|[Drop Eggs](https://lintcode.com/problem/drop-eggs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/drop_eggs.py)|sqrt_n|
|272🔒|[Climbing Stairs II](https://lintcode.com/problem/climbing-stairs-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/climb_stairs_2.py)||
|283|[Max of 3 Numbers](https://lintcode.com/problem/max-of-3-numbers/)|very_easy|
|309|[Interleaved Array](https://lintcode.com/problem/interleaved-array/)|
|328|[String Partition](https://lintcode.com/problem/string-partition/)|leetcode_763|
|334|[Order Check](https://lintcode.com/problem/order-check/)|leetcode_1051|
|362|[Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/)|leetcode_239|
|363|[Trapping Rain Water](https://lintcode.com/problem/trapping-rain-water/)|leetcode_42|
|366|[Fibonacci](https://lintcode.com/problem/fibonacci/)|leetcode_509|
|372|[Delete Node in a Linked List](https://lintcode.com/problem/delete-node-in-a-linked-list/)|leetcode_237|
|373|[Partition Array by Odd and Even](https://lintcode.com/problem/partition-array-by-odd-and-even/)|leetcode_905|
|374|[Spiral Matrix](https://lintcode.com/problem/spiral-matrix/)|leetcode_54|
|376|[Binary Tree Path Sum](https://lintcode.com/problem/binary-tree-path-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/root_to_leaf_paths_target_sum.py)|DFS, backtracking|
|380|[Intersection of Two Linked Lists](https://lintcode.com/problem/intersection-of-two-linked-lists/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/linked_list_cycle.py)||
|381|[Spiral Matrix II](https://lintcode.com/problem/spiral-matrix-2/)|leetcode_59|
|382🔒|[Triangle Count](https://lintcode.com/problem/triangle-count/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/triangle_count.py)|two_sum, 双指针|
|383|[Container With Most Water](https://lintcode.com/problem/container-with-most-water/)|leetcode_11|
|384|[Longest Substring Without Repeating Characters](https://lintcode.com/problem/longest-substring-without-repeating-characters/)|leetcode_3|
|386|[Longest Substring with At Most K Distinct Characters](https://lintcode.com/problem/longest-substring-with-at-most-k-distinct-characters/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/longest_substr_with_longest_k_distinct_chars.py)||
|388|[Permutation Sequence](https://lintcode.com/problem/permutation-sequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/permutation.py)||
|389|[Valid Sudoku](https://lintcode.com/problem/valid-sudoku/)|leetcode_36|
|392|[House Robber](https://lintcode.com/problem/house-robber/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/house_robber.py)||
|393|[Best Time to Buy and Sell Stock IV](https://lintcode.com/problem/best-time-to-buy-and-sell-stock-iv/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|397|[Longest Continuous Increasing Subsequence](https://lintcode.com/problem/longest-continuous-increasing-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/longest_continuous_increase_subsequence.py)||
|400|[Maximum Gap](https://lintcode.com/problem/maximum-gap/)|leetcode_164|
|406|[Minimum Size Subarray Sum](https://lintcode.com/problem/minimum-size-subarray-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/minimun_size_subarray_sum.py)|sliding_window|
|407|[Plus One](https://lintcode.com/problem/plus-one/)|leetcode_66|
|408|[Add Binary](https://lintcode.com/problem/add-binary/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/easy/add_bigint_string.cpp)|
|411|[Gray Code](https://lintcode.com/problem/gray-code/)|[Rust](src/easy/array/gray_code.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/easy/gray_code.py)||
|415|[Valid Palindrome](https://lintcode.com/problem/valid-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome.py)|two_pointers|
|417|[Valid number](https://lintcode.com/problem/valid-number/)|leetcode_65|
|418|[Integer to Roman](https://lintcode.com/problem/integer-to-roman/)|leetcode_12|
|419|[Roman to Integer](https://lintcode.com/problem/roman-to-integer/)|leetcode_13|
|420|[Count And Say](https://lintcode.com/problem/count-and-say/)|leetcode_38|
|423|[Valid Parentheses](https://lintcode.com/problem/valid-parentheses/)|leetcode_20|
|425|[Letter Combinations of a Phone Number](https://lintcode.com/problem/letter-combinations-of-a-phone-number/)|leetcode_17|
|426|[Restore IP Addresses](https://lintcode.com/problem/restore-ip-addresses/)|leetcode_93|
|427|[Generate Parentheses](https://lintcode.com/problem/generate-parentheses/)|leetcode_22|
|428|[Pow(x, n)](https://lintcode.com/problem/powx-n/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/pow.py)|binary_search, 快速幂运算|
|433|[Number of Islands](https://lintcode.com/problem/number-of-islands/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/number_of_islands.py)|union_find, DFS, BFS|
|437|[Copy Books](https://lintcode.com/problem/copy-bookes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/copy_books.py)|dp, binary_search|
|440🔒|[Backpack III](https://lintcode.com/problem/backpack-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_3_item_can_select_multi.py)||
|442|[Implement Trie (Prefix Tree)](https://lintcode.com/problem/implement-trie-prefix-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/tire/impl_tire_prefix_tree.py)|前缀树/字典树|
|443🔒|[Two Sum - Greater than target](https://lintcode.com/problem/two-sum-greater-than-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_le_count.py)|two_pointers|
|447🔒|[Search in a Big Sorted Array](https://lintcode.com/problem/search-in-a-big-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search_unknown_size_sorted_array.py)|binary_search_first, 倍增法|
|451|[Swap Nodes in Pairs](https://lintcode.com/problem/swap-nodes-in-pairs/)|leetcode_24|
|452|[Remove Linked List Elements](https://lintcode.com/problem/remove-linked-list-elements/)|leetcode_203|
|453|[Flatten Binary Tree to Linked List](https://lintcode.com/problem/flatten-binary-tree-to-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/in_place_flatten_binary_tree_to_linked_list.py)||
|454|[Rectangle Area](https://lintcode.com/problem/rectangle-area/)|||
|457|[Classical Binary Search](https://lintcode.com/problem/classical-binary-search/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search.py)|binary_search|
|458|[Last Position of Target](https://lintcode.com/problem/last-position-of-target/)|leetcode_34|
|460|[Find K Closest Elements](https://lintcode.com/problem/find-k-closest-elements/)|leetcode_658|
|461|[Kth Smallest Numbers in Unsorted Array](https://lintcode.com/problem/kth-smallest-numbers-in-unsorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)||
|462|[Total Occurrence of Target](https://lintcode.com/problem/total-occurrence-of-target/)|leetcode_34|
|463🤔|[Sort Integers](https://lintcode.com/problem/sort-integers/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/basic_sorting.py)|
|464🤔|[Sort Integers II](https://lintcode.com/problem/sort-integers-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/basic_sorting.py)|
|466|[Count Linked List Nodes](https://lintcode.com/problem/count-linked-list-nodes/)|too_easy|
|469|[Same Tree](https://lintcode.com/problem/binary-tree-level-order-traversal/)|leetcode_100|
|474🔒|[Lowest Common Ancestor II](https://lintcode.com/problem/lowest-common-ancestor-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)||
|476|[Stone Game(diff to leetcode)](https://lintcode.com/problem/stone-game/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stone_game_merge.py)||
|480|[Binary Tree Paths](https://lintcode.com/problem/binary-tree-paths/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/root_to_leaf_paths_find_all.py)|DFS, backtracking|
|484|[Swap Two Integers in Array](https://lintcode.com/problem/swap-two-integers-in-array/)|||
|486|[Merge K Sorted Arrays](https://lintcode.com/problem/merge-k-sorted-arrays/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_two_sorted_arrays.py)||
|488|[Happy Number](https://lintcode.com/problem/happy-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/happy_number.py)||
|491|[Palindromic Number](https://lintcode.com/problem/palindrome-number/)|leetcode_9|
|492|[Implement Queue by Linked List](https://lintcode.com/problem/implement-queue-by-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_queue_by_linked_list.py)||
|494|[Implement Stack by Two Queues](https://lintcode.com/problem/implement-stack-by-two-queues/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_stack_by_queue.py)||
|495|[Implement Stack](https://lintcode.com/problem/implement-stack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_stack_by_queue.py)||
|512|[Decode Ways](https://lintcode.com/problem/decode-ways/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/decode_ways.py)||
|513|[Perfect Squares](https://lintcode.com/problem/perfect-squares/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)|完全背包问题|
|517|[Ugly Number](https://lintcode.com/problem/ugly-number/)|leetcode_263|
|521🔒|[Remove Duplicate Numbers in Array](https://lintcode.com/problem/remove-duplicate-numbers-in-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/remove_duplicate_numbers_in_array.py)|partition_array|
|533🔒|[Two Sum - Closest to target](https://lintcode.com/problem/two-sum-closest-to-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_unique_pairs.py)|two_sum|
|534|[House Robber II](https://lintcode.com/problem/house-robber-ii/)|leetcode_213|
|535|[House Robber III](https://lintcode.com/problem/house-robber-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/house_robber.py)||
|539|[Move Zeros](https://lintcode.com/problem/move-zeroes/)|leetcode_283|
|544|[Top k Largest Numbers](https://lintcode.com/problem/top-k-largest-numbers/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/top_k_largest_stream_min_heap.py)|min_heap|
|545|[Top k Largest Numbers II](https://lintcode.com/problem/top-k-largest-numbers-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/top_k_largest_stream_min_heap.py)|min_heap|
|547|[Intersection of Two Arrays](https://lintcode.com/problem/intersection-of-two-arrays/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/intersection_of_two_arrays.py)||
|548|[Intersection of Two Arrays II](https://lintcode.com/problem/intersection-of-two-arrays-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/intersection_of_two_arrays.py)||
|562|[Backpack IV](https://lintcode.com/problem/backpack-iv/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)|完全背包问题|
|564|[Combination Sum IV](https://lintcode.com/problem/combination-sum-iv/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)|完全背包问题|
|563|[Backpack V](https://lintcode.com/problem/backpack-v/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_5_plans_count.py)||
|577|[Merge k Sorted Interval Lists](https://lintcode.com/problem/merge-k-sorted-interval-lists/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_two_sorted_interval_lists.py)||
|578|[Lowest Common Ancestor III](https://lintcode.com/problem/lowest-common-ancestor-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)||
|582|[Word Break II](https://lintcode.com/problem/word-break-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/word_break_2.py)||
|584|[Drop Eggs II](https://lintcode.com/problem/drop-eggs-ii/)|[Rust](src/dp/drop_eggs.rs)|dp|
|585🔒|[Maximum Number in Mountain Sequence](https://lintcode.com/problem/maximum-number-in-mountain-sequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_max.py)|mountain_array, binary_search|
|586🔒|[Sqrt(x) II](https://lintcode.com/problem/sqrtx-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/sqrt.py)|牛顿连续均值求根法|
|587🔒|[Two Sum - Unique pairs](https://lintcode.com/problem/two-sum-unique-pairs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_unique_pairs.py)|two_sum|
|588|[Partition Equal Subset Sum](https://lintcode.com/problem/partition-equal-subset-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)|0-1背包问题|
|594|[Implement strStr() II](https://lintcode.com/problem/strstr-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/string/find_substr.py)|kmp, Rabin-Karp(rolling_hash)|
|596🔒|[Minimum Subtree](https://lintcode.com/problem/maximum-subtree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/subtree_max_sum.py)|divide_and_conquer|
|600|[Smallest Rectangle Enclosing Black Pixels](https://lintcode.com/problem/smallest-rectangle-enclosing-black-pixels)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/smallest_rectangle_enclosing_black_pixels.py)||
|603|[Largest Divisible Subset](https://lintcode.com/problem/largest-divisible-subset/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/largest_divisible_subset.py)||
|604🔒|[Window Sum](https://lintcode.com/problem/window-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/very_easy.py)||
|606|[Kth Largest Element II](https://lintcode.com/problem/kth-largest-element-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)|quick_select, quick_sort, heap|
|607|[Two Sum III - Data structure design](https://lintcode.com/problem/two-sum-iii-data-structure-design/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_3_impl.py)|two_pointers|
|608|[Two Sum II - Input array is sorted](https://lintcode.com/problem/two-sum-ii-input-array-is-sorted/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_2_input_is_sorted.py)|two_pointers|
|609|[Two Sum - Less than or equal to target](https://lintcode.com/problem/two-sum-less-than-or-equal-to-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_le_count.py)|two_pointers|
|609|[Two Sum - Difference equals to target](https://lintcode.com/problem/two-sum-difference-equals-to-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_diff.py)|two_pointers|
|611🔒|[Knight Shortest Path](https://lintcode.com/problem/knight-shortest-path/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/knight_shortest_path.py)|bfs|
|612🔒|[K Closest Points](https://lintcode.com/problem/k-closest-points/)|leetcode_973|
|615|[Course Schedule](https://lintcode.com/problem/course-schedule/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/course_schedule_2.py)|BFS, topological_sorting|
|616|[Course Schedule II](https://lintcode.com/problem/course-schedule-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/course_schedule_2.py)|BFS, topological_sorting|
|627|[Longest Palindromic Combination](https://lintcode.com/problem/longest-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_combination.py)|greedy|
|628|[Maximum Subtree](https://lintcode.com/problem/maximum-subtree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/subtree_max_sum.py)|divide_and_conquer|
|630🔒|[Knight Shortest Path II](https://lintcode.com/problem/knight-shortest-path-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/knight_shortest_path_2.py)|bfs|
|632|[Binary Tree Maximum Node](https://lintcode.com/problem/binary-tree-maximum-node/)|||
|633|[Find the Duplicate Number](https://lintcode.com/problem/find-the-duplicate-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/find_duplicate_number.py)|快慢双指针|
|654|[Sparse Matrix Multiplication](https://lintcode.com/problem/sparse-matrix-multiplication/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/sparse_matrix_multiplication.py)||
|655|[Add String](https://lintcode.com/problem/add-strings/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/easy/add_bigint_string.cpp)||
|657|[Insert Delete GetRandom O(1)](https://lintcode.com/problem/insert-delete-getrandom-o1/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/insert_delete_get_random_o1.py)||
|661|[Convert BST to Greater Tree](https://lintcode.com/problem/convert-bst-to-greater-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_to_gst.py)||
|667|[Longest Palindromic Subsequence](https://lintcode.com/problem/longest-palindromic-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_subsequence.py)|dp(greedy)|
|668|[Ones and Zeroes](https://lintcode.com/problem/ones-and-zeroes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_ones_and_zeros.py)|完全背包问题|
|669|[Coin Change](https://lintcode.com/problem/coin-change/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)|完全背包问题|
|683|[Word Break III](https://lintcode.com/problem/word-break-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/word_break_3.py)||
|685|[First Unique Number in Data Stream](https://lintcode.com/problem/first-unique-number-in-data-stream/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/first_unique_number_in_stream.py)||
|689|[First Unique Number in Data Stream](https://lintcode.com/problem/two-sum-iv-input-is-a-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_two_sum.py)||
|691|[Recover Binary Search Tree](https://lintcode.com/problem/recover-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_recover_swap_two_nodes.py)||
|701|[Trim A Binary Search Tree](https://lintcode.com/problem/trim-a-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_trim_in_range.py)||
|702|[Russian Doll Envelopes](https://lintcode.com/problem/russian-doll-envelopes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/russian_doll_envelopes.py)||
|719|[Calculate Maximum Value](https://lintcode.com/problem/calculate-maximum-value/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/calculate_max_value_2.py)||
|724|[Minimum Partition](https://lintcode.com/problem/minimum-partition/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)|0-1背包问题|
|740|[Coin Change 2](https://lintcode.com/problem/coin-change-2/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)|完全背包问题|
|741|[Calculate Maximum Value II](https://lintcode.com/problem/calculate-maximum-value-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/calculate_max_value_2.py)||
|742|[Self Dividing Numbers](https://lintcode.com/problem/self-dividing-numbers/)|leetcode_728|
|749|[John's backyard garden](https://lintcode.com/problem/johns-backyard-garden/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)||
|761|[Smallest Subset](https://lintcode.com/problem/smallest-subset/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/smallest_subset.py)||
|772|[Group Anagrams](https://lintcode.com/problem/group-anagrams/)|leetcode_49|
|773|[Valid Anagram](https://lintcode.com/problem/vlid-anagram/)|leetcode_242|
|777|[Valid Perfect Square](https://lintcode.com/problem/valid-perfect-square/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/sqrt.py)||
|787|[The Maze](https://lintcode.com/problem/the-maze/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/the_maze.py)||
|793|[Intersection of Arrays](https://lintcode.com/problem/intersection-of-arrays/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/intersection_of_two_arrays.py)||
|802|[Soduku Solver](https://lintcode.com/problem/soduku-solver/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/soduku_solver.py)||
|813|[Find Anagram Mappings](https://lintcode.com/problem/find-anagram-mappings/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/mapping_two_anagram_list_int.py)||
|816|[Traveling Salesman Problem](https://lintcode.com/problem/traveling-salesman-problem/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/traveling_salesman_problem.py)||
|828|[Word Pattern](https://lintcode.com/problem/word-pattern/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/word_pattern.py)||
|829|[Word Pattern II](https://lintcode.com/problem/word-pattern-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/word_pattern_2.py)|DFS|
|835|[Hamming Distance](https://lintcode.com/problem/hamming-distance/)|leetcode_461|
|839|[Merge Two Sorted Interval Lists](https://lintcode.com/problem/merge-two-sorted-interval-lists/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_two_sorted_interval_lists.py)||
|840|[Range Sum Query - Mutable](https://lintcode.com/problem/range-sum-query-mutable/)|leetcode_307|
|841|[String Replace](https://lintcode.com/problem/string-replace/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/tire/string_replace.py)||
|845|[Greatest Common Divisor](https://lintcode.com/problem/greatest-common-divisor/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/greatest_common_divisor.py)||
|859|[Max Stack](https://lintcode.com/problem/max_stack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/max_stack.py)||
|871|[Minimum Factorization](https://lintcode.com/problem/minimum-factorization/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/minimum_factorization.py)|greedy|
|880|[Construct Binary Tree from String](https://lintcode.com/problem/construct-binary-tree-from-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_string_with_parentheses.py)||
|891|[Valid Palindrome II](https://lintcode.com/problem/valid-palindrome-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome_2.py)|two_pointers, greedy|
|892|[Alien Dictionary](https://lintcode.com/problem/alien-dictionary/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/alien_dictionary.py)|heapq, topological_sorting|
|900|[Closest Binary Search Tree Value](https://lintcode.com/problem/closest-binary-search-tree-value/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value.py)||
|901|[Closest Binary Search Tree Value II](https://lintcode.com/problem/closest-binary-search-tree-value-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value_2.py)||
|902|[Kth Smallest Element in a BST](https://lintcode.com/problem/kth-smallest-element-in-a-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|904|[Plus One Linked List](https://lintcode.com/problem/plus-one-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/bottom_up_plus_one_linked_list.py)||
|916|[Palindrome Permutation](https://lintcode.com/problem/palindrome-permutation/)|leetcode_266|
|943|[Range Sum Query - Immutable](https://lintcode.com/problem/range-sum-query-immutable/)|leetcode_303|
|954|[Insert Delete GetRandom O(1) - Duplicates allowed](https://lintcode.com/problem/insert-delete-getrandom-o1-duplicates-allowed/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/insert_delete_get_random_o1.py)||
|955|[Implement Queue by Circular Array](https://lintcode.com/problem/implement-queue-by-circular-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_queue_by_circluar_array.py)||
|973|[1-bit and 2-bit Characters](https://lintcode.com/problem/1-bit-and-2-bit-characters/)|[Rust](src/easy/one_or_two_bit_char.rs)||
|975|[2 Keys Keyboard](https://lintcode.com/problem/2-keys-keyboard/)|leetcode_650|
|976|[4Sum II](https://lintcode.com/problem/4sum-ii/)|leetcode_454|
|980|[Basic Calculator II](https://lintcode.com/problem/basic-calculator-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/calculator_2.py)||
|982|[Arithmetic Slices](https://lintcode.com/problem/arithmetic-slices/)|leetcode_413|
|995|[Best Time to Buy and Sell Stock with Cooldown](https://lintcode.com/problem/best-time-to-buy-and-sell-stock-with-cooldown/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|1000|[Best Time to Buy and Sell Stock with Transaction Fee](https://lintcode.com/problem/best-time-to-buy-and-sell-stock-with-transaction-fee/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|1010|[Max Increase to Keep City Skyline](https://lintcode.com/problem/max-increase-to-keep-city-skyline/)|leetcode_807|
|1038|[Jewels and Stones](https://lintcode.com/problem/jewels-and-stones/)|leetcode_771|
|1057|[Network Delay Time](https://lintcode.com/problem/network-delay-time/)|leetcode_743|
|1079|[Count Binary Substrings](https://lintcode.com/problem/count-binary-substrings/)|leetcode_696|
|1115|[Average of Levels in Binary Tree](https://lintcode.com/problem/average-of-levels-in-binary-tree/)|leetcode_633|
|1137|[Construct String from Binary Tree](https://lintcode.com/problem/construct-string-from-binary-tree/)|leetcode_606|
|1153|[string sorting](https://lintcode.com/problem/string-sorting/)|||
|1162|[Merge Two Binary Trees](https://lintcode.com/problem/merge-two-binary-trees/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/merge_two_binary_tree.py)||
|1173|[Reverse Words in a String III](https://lintcode.com/problem/reverse-words-in-a-string-iii/)|[C++](https://github.com/pymongo/cpp_learn/blob/master/leetcode/easy/reverse_words_in_a_string_3.cpp)||
|1179🔒|[Friend Circles](https://lintcode.com/problem/friend-circles/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/friend_circles.py)|union_find, DFS, BFS|
|1186|[Encode and Decode TinyURL](https://lintcode.com/problem/encode-and-decode-tinyurl/)|||
|1199|[Perfect Number](https://lintcode.com/problem/perfect-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/perfect_number.py)||
|1080|[Max Area of Island](https://lintcode.com/problem/max-area-of-island/)|leetcode_695|
|1196|[Freedom Trail](https://lintcode.com/problem/freedom-trail/)|leetcode_514|
|1201|[Next Greater Element II](https://lintcode.com/problem/next-greater-element-ii/)|leetcode_503|
|1205|[Diagonal Traverse](https://lintcode.com/problem/diagonal-traverse/)|leetcode_498|
|1208|[Target Sum](https://lintcode.com/problem/target-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_target_sum.py)|0-1背包问题|
|1212|[Max Consecutive Ones](https://lintcode.com/problem/max-consecutive-ones/)|leetcode_485|
|1222|[Validate IP Address](https://lintcode.com/problem/validate-ip-address/)|leetcode_486|
|1225|[Island Perimeter](https://lintcode.com/problem/island-perimeter/)|leetcode_463|
|1235|[Serialize and Deserialize BST](https://lintcode.com/problem/serialize-and-deserialize-bst/)|leetcode_449|
|1246|[Longest Repeating Character Replacement](https://lintcode.com/problem/longest-repeating-character-replacement/)|leetcode_426|
|1252|[Queue Reconstruction by Height](https://lintcode.com/problem/queue-reconstruction-by-height/)|leetcode_406|
|1254|[Sum of Left Leaves](https://lintcode.com/problem/sum-of-left-leaves/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/sum_of_left_leaves.py)||
|1276|[Sum of Two Integers](https://lintcode.com/problem/sum-of-two-integers/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/binary_addition.py)|binary_addition|
|1281|[Top K Frequent Elements](https://lintcode.com/problem/top-k-frequent-elements/)|leetcode_347|
|1283|[Reverse Array](https://lintcode.com/problem/reverse-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/reverse_string.py)||
|1284|[Integer Break](https://lintcode.com/problem/integer-break/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/integer_break.py)|划分类DP|
|1285|[Power of Four](https://lintcode.com/problem/power-of-four/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_4.py)|bitwise|
|1292|[Odd Even Linked List](https://lintcode.com/problem/odd-even-linked-list/)|leetcode_328|
|1294|[Power of Three](https://lintcode.com/problem/power-of-three/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/is_power_of_3.py)|bitwise|
|1300|[Bash Game(Nim Game)](https://lintcode.com/problem/bash-game/)|leetcode_292|
|1311|[Lowest Common Ancestor of a Binary Search Tree](https://lintcode.com/problem/lowest-common-ancestor-of-a-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_lowest_common_ancestor.py)||
|1317|[Count Complete Tree Nodes](https://lintcode.com/problem/count-complete-tree-nodes/)|leetcode_222|
|1319|[Contains Duplicate II](https://lintcode.com/problem/contains-duplicate-ii/)|leetcode_219|
|1320|[Contains Duplicate](https://lintcode.com/problem/contains-duplicate-ii/)|leetcode_217|
|1324|[Count Primes](https://lintcode.com/problem/count-primes/)|leetcode_204|
|1321🔒|[Combination Sum III](https://lintcode.com/problem/combination-sum-iii/)|[Rust](src/backtracking/combination_sum_3.rs)||
|1332|[Number of 1 Bits](https://lintcode.com/problem/number-of-1-bits/)|leetcode_191|
|1333|[Reverse Bits](https://lintcode.com/problem/reverse-bits/)|leetcode_190|
|1334|[Rotate Array](https://lintcode.com/problem/rotate-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/rotate_array_right_circle_shift_elements.py)|reverse, circle_shift|
|1343|[Sum of Two Strings](https://lintcode.com/problem/sum-of-two-strings/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/sum_of_to_strings.py)||
|1354|[Pascals Triangle II](https://lintcode.com/problem/pascals-triangle-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/pascals_triangle.py)||
|1355|[Pascals Triangle](https://lintcode.com/problem/pascals-triangle/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/pascals_triangle.py)||
|1359|[Convert Sorted Array to Binary Search Tree](https://lintcode.com/problem/convert-sorted-array-to-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/sorted_list_or_array_to_bst.py)|divide_and_conquer|
|1363|[ZigZag Conversion](https://lintcode.com/problem/zigzag-conversion/)|leetcode_6|
|1375|[Substring With At Least K Distinct Characters](https://lintcode.com/problem/substring-with-at-least-k-distinct-characters/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/substring_with_at_least_k_distinct_characters.py)||
|1424|[Longest Mountain in Array](https://lintcode.com/problem/longest-mountain-in-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_longest.py)|mountain_array|
|1428|[Keys and Rooms](https://lintcode.com/problem/keys-and-rooms/)|leetcode_841|
|1479|[Can Reach The Endpoint](https://lintcode.com/problem/can-reach-the-endpoint/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/can_reach_the_end_point.py)|bfs|
|1492|[Koko Eating Bananas](https://lintcode.com/problem/koko-eating-bananas/)|leetcode_875|
|1499|[Reordered Power of 2](https://lintcode.com/problem/reordered-power-of-2/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/reordered_power_of_2.py)|permutation|
|1505|[Find The Number](https://lintcode.com/problem/find-the-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/very_easy.py)|
|1508|[Score After Flipping Matrix](https://lintcode.com/problem/score-after-flipping-matrix/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/score_after_flipping_matrix.py)|greedy|
|1509|[Lemonade Change](https://lintcode.com/problem/lemonade-change/)|[Rust](src/easy/very_easy.rs)|
|1524|[Search in a Binary Search Tree](https://lintcode.com/problem/search-in-a-binary-search-tree/)|leetcode_700|
|1525|[N-ary Tree Postorder Traversal](https://lintcode.com/problem/n-ary-tree-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/n_ary_tree_preorder_postorder.py)||
|1529|[N-ary Tree Preorder Traversal](https://lintcode.com/problem/n-ary-tree-preorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/n_ary_tree_preorder_postorder.py)||
|1533|[N-ary Tree Level Order Traversal](https://lintcode.com/problem/n-ary-tree-level-order-traversal/)|leetcode_429|
|1535|[To Lower Case](https://lintcode.com/problem/to-lower-case/)|leetcode_709|
|1536|[Find First and Last Position of Element in Sorted Array](https://lintcode.com/problem/find-first-and-last-position-of-element-in-sorted-array/)|leetcode_34|
|1593|[Construct Binary Tree from Preorder and Postorder Traversal](https://lintcode.com/problem/construct-binary-tree-from-preorder-and-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_pre_order_and_post_order.py)|DFS|
|1609|[Middle of the Linked List](https://lintcode.com/problem/middle-of-the-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/middle_of_linked_list.py)|list_node, 快慢指针|
|1656|[Legal Number Statistics](https://lintcode.com/problem/legal-number-statistics/)|very_easy|
|1704|[Range Sum of BST](https://lintcode.com/problem/range-sum-of-bst/)|leetcode_938|
|1777🔒|[Circle Area](https://lintcode.com/problem/circle-area/)|very_easy|
|1790🔒|[Rotate String II](https://lintcode.com/problem/rotate-string-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/rotate_string_2.py)|reverse, circle_shift|
|1848_ABANDONED|[Word Search III](https://lintcode.com/problem/word-search-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/word_search_3.py)||
|1870|[number of substrings with all zeroes](https://lintcode.com/problem/number-of-substrings-with-all-zeroes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/number_of_substrings_with_all_zeroes.py)|
|1872|[Longest Common Subsequence](https://lintcode.com/problem/minimum-cost-to-connect-sticks/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/minimum_cost_to_connect_sticks.py)||
|1876|[Alien Dictionary(easy)](https://lintcode.com/problem/alien-dictionaryeasy/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/verifying_an_alien_dictionary.py)||
|1901|[Squares of a Sorted Array](https://lintcode.com/problem/squares-of-a-sorted-array/)|leetcode_977|

---

# Category

## Dynamic Programming

### 划分型动态规划

戳气球(dp/burst_ballon.py)、石子归并(dp/stone_game_merge.py)

### 选或不选类DP

- 打家劫舍系列
- 股票买卖系列

### 计数型动态规划

TODO k sum

## Palindrome

### Palindrome Sequence

- [Longest Palindromic Subsequence](https://lintcode.com/problem/longest-palindromic-subsequence)
- [Valid Palindrome II](https://lintcode.com/problem/valid-palindrome-ii/)

### Palindrome Permutation

- [Longest Palindromic Combination](https://lintcode.com/problem/longest-palindrome) | [Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_combination.py) | greedy
- [Palindrome Permutation](https://lintcode.com/problem/palindrome-permutation/) | [Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/palindrome_permutation.py) | greedy

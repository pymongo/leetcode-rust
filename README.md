## some interesting topic about Rust on leetcode

### use rustc source code to solve leetcode

leetcode-rust package require **rustc_private** feature to enable rustc relative API

e.g. use `rustc_span::lev_distance::lev_distance(&a, &b)` one line to solve leetcode edit-distance (Unfortunately we can't use nightly and rustc-dev in leetcode)

- [rustc_span::lev_distance::lev_distance](src/dp/edit_distance.rs) solve [leetcode edit distance](https://leetcode.com/problems/edit-distance/)
- [rustc_lexer::tokenize](src/compiler/basic_calculator_ii_no_parentheses.rs) solve [leetcode Basic Calculator II](https://leetcode.com/problems/basic-calculator-ii/)

### use rustc_graphviz visualize leetcode binary tree

![graphviz_leetcode_binary_tree.png](https://i.loli.net/2021/10/20/SfqnTh5BQvPl74E.png)

---

codeforces_solutions
===

| Problem | Solution |
| ------- | -------- |
|[1A - Theatre Square](https://codeforces.com/problemset/problem/1/A)|[Rust](src/easy/codeforces_easy.rs)|
|[4A - Watermelon](https://codeforces.com/problemset/problem/4/A)|[Rust](src/easy/codeforces_easy.rs)|
|[71A - Way Too Long Words](https://codeforces.com/problemset/problem/71/A)|[Rust](src/easy/codeforces_easy.rs)|
|[158A - Next Round](https://codeforces.com/problemset/problem/158/A)|[Rust](src/easy/codeforces_easy.rs)|
|[231A - Team](https://codeforces.com/problemset/problem/231/A)|[Rust](src/easy/codeforces_easy.rs)|

leetcode_solutions
===

(problem number with `🔒` suffix need leetcode VIP to unlock)

| # | Problem | Solutions | Category/Comment |
|---| ------- | --------- | ---------------- |
|1|[Two Sum](https://leetcode.com/problems/two-sum/)|[Rust](src/two_sum_two_pointers/two_sum.rs), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/main/java/com/leetcode/Bitwise.java)|bitwise|
|2|[Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)|[Rust](src/linked_list/add_two_linked_list.rs), [Java](https://github.com/pymongo/java_leetcode/blob/master/src/main/java/com/leetcode/ListNode.java), [Go](https://github.com/pymongo/go_leetcode/blob/master/traverse_two_list_node_test.go)|linked_list|
|3|[Longest Substring Without Repeating Char...](https://leetcode.com/problems/longest-substring-without-repeating-characters/)|[Rust](src/uncategorized/longest_non_repeated_substr.rs)|sliding_window|
|4|[Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)|[Rust](src/binary_search/median_of_two_sorted_arrays.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/median_of_two_sorted_arrays.py)|binary_search|
|5|[Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/)|[Rust](src/dp/longest_palindromic_substr.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_substr.py)|manacher, suffix_array|
|6|[ZigZag Conversion](https://leetcode.com/problems/zigzag-conversion/)|[Rust](src/easy/leetcode_very_easy.rs)|
|7|[Reverse Integer](https://leetcode.com/problems/reverse-integer/)|[Rust](src/uncategorized/reverse_integer_checked_mul_overflow.rs)|
|8|[String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/string_to_integer_atoi.py)|
|9|[Palindromic Number](https://leetcode.com/problems/palindrome-number/)|[Rust](src/uncategorized/reverse_integer_checked_mul_overflow.rs)|
|10|[Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/regular_expression_matching.py)|
|11|[Container With Most Water](https://leetcode.com/problems/container-with-most-water/)|[Rust](src/greedy/container_with_most_water.rs)|
|12|[Integer to Roman](https://leetcode.com/problems/integer-to-roman/)|[Rust](src/greedy/int_to_roman.rs)|
|13|[Roman to Integer](https://leetcode.com/problems/roman-to-integer/)|[Rust](src/greedy/int_to_roman.rs)|
|14|[Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/longest_common_prefix_of_k_strs.py)|
|15|[3Sum](https://leetcode.com/problems/3sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/three_sum_plans_detail.py)|two_pointers, two_sum|
|16|[3Sum Closest](https://leetcode.com/problems/3sum-closest/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/three_sum_closest.py)|two_sum|
|17|[Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)|[Rust](src/bfs_dfs_backtracking/letter_combinations_of_a_phone_number.rs)|BFS|
|18|[4Sum](https://leetcode.com/problems/4sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/four_sum_plans_detail.py)|two_pointers, two_sum|
|19|[Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)|[Rust](src/linked_list/nth_node_from_end.rs)|sliding_window|
|20|[Valid Parentheses](https://leetcode.com/problems/valid-parentheses/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/valid_parentheses.py)|stack|
|21|[Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)|[Rust](src/linked_list/merge_two_sorted_linked_list.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_k_sorted_lists.py)|
|22|[Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/generate_parentheses.py)|backtracking|
|23|[Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_k_sorted_lists.py)|
|24|[Swap Nodes in Pairs](https://leetcode.com/problems/swap-nodes-in-pairs/)|[Rust](src/linked_list/swap_nodes_in_pairs.rs)|
|26|[Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/)|[Rust](src/easy/array/partition_array.rs)|
|27|[Remove Element](https://leetcode.com/problems/remove-element/)|[Rust](src/easy/array/partition_array.rs)|
|28|[Implement strStr()](https://leetcode.com/problems/implement-strstr/)|[Rust](src/uncategorized/contains_substr_kmp.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/string/find_substr.py)|kmp, dfa|
|31|[Next Permutation](https://leetcode.com/problems/next-permutation/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/next_permutation.py)|
|33|[Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search.py)|binary_search|
|34|[Find First and Last Position of Element ...](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/)|[Rust](src/binary_search/mod.rs)|
|35|[Search Insert Position](https://leetcode.com/problems/search-insert-position/)|[Rust](src/binary_search/mod.rs)|
|36|[Valid Sudoku](https://leetcode.com/problems/valid-sudoku/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/valid_soduku.py)|
|37|[Soduku Solver](https://leetcode.com/problems/soduku-solver/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/soduku_solver.py)|
|38|[Count And Say](https://leetcode.com/problems/count-and-say/)|[Rust](src/easy/leetcode_very_easy.rs)|
|39|[Combination Sum](https://leetcode.com/problems/combination-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/combination_target_sum.py), [Rust](src/bfs_dfs_backtracking/combination_sum_1_2.rs)|
|40|[Combination Sum II](https://leetcode.com/problems/combination-sum-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/combination_target_sum.py), [Rust](src/bfs_dfs_backtracking/combination_sum_1_2.rs)|
|42|[Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/)|[Rust](src/dp/trapping_rain_water.rs)|
|44|[Wildcard Matching](https://leetcode.com/problems/wildcard-matching/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/wildcard_matching.py)|
|45|[Jump Game II](https://leetcode.com/problems/jump-game-ii/)|[Rust](src/dp/jump_game_ii.rs)|greedy|
|46|[Permutations](https://leetcode.com/problems/permutations/)|[Rust](src/bfs_dfs_backtracking/permutations.rs)|
|47|[Permutations II](https://leetcode.com/problems/permutations-ii/)|[Rust](src/bfs_dfs_backtracking/permutations.rs)|
|48|[Rotate Image](https://leetcode.com/problems/rotate-image/)|[Rust](src/easy/grid_or_matrix/rotate_matrix.rs)|
|49|[Group Anagrams](https://leetcode.com/problems/group-anagrams/)|[Rust](src/counter/anagrams.rs)|
|50|[Pow(x, n)](https://leetcode.com/problems/powx-n/)|[Rust](src/math/pow.rs)|
|51|[N Queens](https://leetcode.com/problems/n-queens)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/n_queens.py), [Rust](src/bfs_dfs_backtracking/n_queens.rs)||
|52|[N Queens II](https://leetcode.com/problems/n-queens-ii)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/n_queens.py), [Rust](src/bfs_dfs_backtracking/n_queens.rs)|
|53|[Maximum Subarray](https://leetcode.com/problems/maximum-subarray/)|[Rust](src/greedy/maximum_subarray.rs)|
|54|[Spiral Matrix](https://leetcode.com/problems/spiral-matrix/)|[Rust](src/easy/grid_or_matrix/spiral_matrix.rs)|
|55|[Jump Game](https://leetcode.com/problems/jump-game/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/jump_game.py)|greedy, dp|
|58|[Length of Last Word](https://leetcode.com/problems/length-of-last-word/)|[Rust](src/easy/leetcode_very_easy)|
|59|[Spiral Matrix II](https://leetcode.com/problems/spiral-matrix-2/)|[Rust](src/easy/grid_or_matrix/spiral_matrix.rs)|
|60|[Permutation Sequence](https://leetcode.com/problems/permutation-sequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/permutation.py)||
|62|[Unique Paths](https://leetcode.com/problems/unique-paths/)|[Rust](src/dp/unique_paths.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/shortest_paths_on_checkerboard_test.go)|combination|
|63|[Unique Paths II](https://leetcode.com/problems/unique-paths-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/unique_paths_2.py)|
|64|[Minimum Path Sum](https://leetcode.com/problems/minimum-path-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/grid_min_path_sum.py)||
|65|[Valid Number](https://leetcode.com/problems/valid-number/)|[Rust](src/easy/leetcode_very_easy.rs)|
|66|[Plus One](https://leetcode.com/problems/plus-one/)|[Rust](src/easy/leetcode_very_easy.rs)|
|67|[Add Binary](https://leetcode.com/problems/add-binary/)|[C++](https://github.com/pymongo/learn_cpp/blob/master/leetcode/add_bigint_string.cpp)|
|69|[Sqrt(x)](https://leetcode.com/problems/sqrtx/)|[Rust](src/math/sqrt.rs)|
|70|[Climb Stairs](https://leetcode.com/problems/climbing-stairs/)|[Rust](src/dp/fibonacci.rs)|fibonacci|
|72|[Edit Distance](https://leetcode.com/problems/edit-distance/)|[Rust](src/dp/edit_distance.rs)|
|73|[Set Matrix Zeroes](https://leetcode.com/problems/set-matrix-zeroes/)|[Rust](src/easy/grid_or_matrix/mod.rs)|
|74|[Search a 2D Matrix](https://leetcode.com/problems/search-a-2d-matrix/)|[Rust](src/binary_search/search_a_2d_matrix.rs)|
|75|[Sort Colors](https://leetcode.com/problems/sort-colors/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/sort_colors.py)|three_pointers, partition_array|
|77|[Combinations](https://leetcode.com/problems/combinations/)|[Rust](src/bfs_dfs_backtracking/combinations_subsets.rs)|
|78|[Subsets](https://leetcode.com/problems/subsets/)|[Rust](src/bfs_dfs_backtracking/combinations_subsets.rs)|
|79|[Word Search](https://leetcode.com/problems/word-search/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/word_search.py)|
|81|[Search in Rotated Sorted Array II](https://leetcode.com/problems/search-in-rotated-sorted-array-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search_2_with_duplicate.py)|binary_search|
|82|[Remove Duplicates from Sorted List II](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/)|[C++](https://github.com/pymongo/learn_cpp/blob/master/leetcode/linked_list/remove_duplicates_from_sorted_list.cpp)|
|83|[Remove Duplicates from Sorted List](https://leetcode.com/problems/remove-duplicates-from-sorted-list/)|[C++](https://github.com/pymongo/learn_cpp/blob/master/leetcode/linked_list/remove_duplicates_from_sorted_list.cpp)|
|86|[Partition List](https://leetcode.com/problems/partition-list/)|[Rust](src/linked_list/partition_list.rs)|
|88|[Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/)|[Rust](src/easy/leetcode_very_easy.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_two_sorted_arrays.py)|merge_sort|
|89|[Gray Code](https://leetcode.com/problems/gray-code/)|[Rust](src/bitwise/gray_code.rs)|
|90|[Subsets II](https://leetcode.com/problems/subsets-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/subsets_combinations.py)|
|91|[Decode Ways](https://leetcode.com/problems/decode-ways/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/decode_ways.py)|
|92|[Reverse Linked List II](https://leetcode.com/problems/reverse-linked-list-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/reverse_linked_list.py)|
|93|[Restore IP Addresses](https://leetcode.com/problems/restore-ip-addresses/)|[Rust](src/bfs_dfs_backtracking/restore_ip_address.rs)|
|94|[Binary Tree Inorder Traversal](https://leetcode.com/problems/binary-tree-inorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|98|[Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/)|[Rust](src/binary_tree/is_bst.rs)|
|99|[Recover Binary Search Tree](https://leetcode.com/problems/recover-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_recover_swap_two_nodes.py)|
|100|[Same Tree](https://leetcode.com/problems/same-tree/)|[Rust](src/binary_tree/same_tree.rs)|
|102|[Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/)|[Rust](src/binary_tree/level_order_traversal.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_level_order.py)|
|103|[Binary Tree Zigzag Level Order Traversal](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/)|[Rust](src/binary_tree/level_order_traversal.rs)|
|104|[Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/)|[Rust](src/binary_tree/max_depth_of_binary_tree.rs)|
|105|[Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_pre_order_and_in_order.py)|DFS, stack|
|106|[Construct Binary Tree from Inorder and Postorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_in_order_and_post_order.py)|DFS|
|107|[Binary Tree Level Order Traversal II](https://leetcode.com/problems/binary-tree-level-order-traversal-ii/)|[Rust](src/binary_tree/level_order_traversal.rs)|
|108|[Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/sorted_list_or_array_to_bst.py)|divide_and_conquer|
|109|[Convert Sorted List to Binary Search Tree](https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/sorted_list_or_array_to_bst.py)|divide_and_conquer|
|110|[Balanced Binary Tree](https://leetcode.com/problems/balanced-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_is_balance_binary_tree.py)|divide_and_conquer|
|111|[Minimum Depth of Binary Tree](https://leetcode.com/problems/minimum-depth-of-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_max_min.py)|divide_and_conquer, BFS|
|114|[Flatten Binary Tree to Linked List](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/in_place_flatten_binary_tree_to_linked_list.py)||
|118|[Pascals Triangle](https://leetcode.com/problems/pascals-triangle/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/pascals_triangle.py)|
|119|[Pascals Triangle II](https://leetcode.com/problems/pascals-triangle-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/pascals_triangle.py)|
|120|[Triangle](https://leetcode.com/problems/triangle/)|[Rust](src/dp/triangle.rs)|
|121|[Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|122|[Best Time to Buy and Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)|
|123|[Best Time to Buy and Sell Stock III](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)|
|124|[Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_maximum_path_sum.py)||
|125|[Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome.py)|two_pointers|
|126|[Word Ladder II](https://leetcode.com/problems/word-ladder-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/word_ladder_2.py)|BFS+DFS|
|127|[Word Ladder](https://leetcode.com/problems/word-ladder/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/word_ladder.py)|双向BFS|
|128|[Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/longest_consecutive_sequence.py)|并查集|
|129|[Sum Root to Leaf Numbers](https://leetcode.com/problems/sum-root-to-leaf-numbers/)|[Rust](src/binary_tree/sum_root_to_leaf_numbers.rs)|
|130|[Surrounded Regions](https://leetcode.com/problems/surrounded-regions/)|[Rust](src/bfs_dfs_backtracking/surrounded_regions.rs)|
|133|[Clone Graph](https://leetcode.com/problems/clone-graphs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/clone_graph.py)|DFS, BFS|
|134|[Gas Station](https://leetcode.com/problems/gas-station/)|[Rust](src/greedy/gas_station.rs)|
|135|[Candy](https://leetcode.com/problems/candy/)|[Rust](src/greedy/candy.rs)|
|136|[Single Number](https://leetcode.com/problems/single-number/)|[Rust](src/bitwise/find_single_number.rs)|
|137|[Single Number II](https://leetcode.com/problems/single-number-ii/)|[Rust](src/bitwise/find_single_number.rs)|
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
|150|[Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/)|[Rust](src/compiler/evaluate_reverse_polish_notation.rs)|
|151|[Reverse Words in a String](https://leetcode.com/problems/reverse-words-in-a-string/)|[Rust](src/uncategorized/string_in_place.rs)|
|153|[Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_search.py)|binary_search|
|154|[Find Minimum in Rotated Sorted Array II](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_min_2_with_duplicate.py)|binary_search|
|155|[Min Stack](https://leetcode.com/problems/min-stack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/min_stack.py)|
|160|[Intersection of Two Linked Lists](https://leetcode.com/problems/intersection-of-two-linked-lists/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/linked_list_cycle.py)|
|164|[Maximum Gap](https://leetcode.com/problems/maximum-gap/)|[Rust](src/easy/leetcode_very_easy.rs)|
|165|[Compare Version Numbers](https://leetcode.com/problems/compare-version-numbers/)|[Rust](src/easy/leetcode_very_easy.rs)|
|167|[Two Sum II - Input array is sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_2_input_is_sorted.py)|two_pointers|
|168|[Excel Sheet Column Title](https://leetcode.com/problems/excel-sheet-column-title/)|[Rust](src/math/excel_sheet_column_title.rs)|
|169|[Majority Element](https://leetcode.com/problems/majority-element/)|[Rust](src/greedy/majority_element.rs)|
|170🔒|[Two Sum III - Data structure design](https://leetcode.com/problems/two-sum-iii-data-structure-design/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_3_impl.py)|two_pointers|
|171|[Excel Sheet Column Number](https://leetcode.com/problems/excel-sheet-column-number/)|[Rust](src/math/excel_sheet_column_title.rs)|
|172|[Factorial Trailing Zeroes](https://leetcode.com/problems/factorial-trailing-zeroes/)|[Rust](src/greedy/factorial_trailing_zeroes.rs)|
|173|[Binary Search Tree Iterator](https://leetcode.com/problems/binary-search-tree-iterator/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_in_order_iterator.py)|
|174|[Dungeon Game](https://leetcode.com/problems/dungeon-game/)|[Rust](src/dp/calculate_minimum_hp.rs)|
|175|[Combine Two Tables](https://leetcode.com/problems/combine-two-tables/)|
|176|[Second Highest Salary](https://leetcode.com/problems/second-highest-salary/)|[SQL](sql_problems/second_highest_salary.sql)|
|181|[Employees Earning More Than Their Managers](https://leetcode.com/problems/employees-earning-more-than-their-managers/)|[SQL](sql_problems/very_easy.sql)|
|182|[Duplicate Emails](https://leetcode.com/problems/duplicate-emails/)|[SQL](sql_problems/duplicate_emails.sql)|
|183|[Customers Who Never Order](https://leetcode.com/problems/customers-who-never-order/)|sql|
|188|[Best Time to Buy and Sell Stock IV](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|189|[Rotate Array](http://leetcode.com/problems/rotate-array/)|[Rust](src/uncategorized/string_in_place.rs)|
|190|[Reverse Bits](https://leetcode.com/problems/reverse-bits/)|[Rust](src/bitwise/reverse_bits.rs)|
|191|[Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/)|[Rust](src/bitwise/hamming_distance_count_ones.rs)|
|198|[House Robber](https://leetcode.com/problems/house-robber/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/house_robber.py)||
|200|[Number of Islands](https://leetcode.com/problems/number-of-islands/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/number_of_islands.py)|special_data_structure.union_find, DFS, BFS|
|202|[Happy Number](https://leetcode.com/problems/happy-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/happy_number.py)|
|203|[Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/remove_linked_list_element.py)|
|204|[Count Primes](https://leetcode.com/problems/count-primes/)|[Rust](src/math/count_primes.rs)|
|206|[Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/reverse_linked_list.py), [Rust](src/linked_list/reverse_linked_list.rs)||
|208|[Implement Trie (Prefix Tree)](https://leetcode.com/problems/implement-trie-prefix-tree/)|[Rust](src/data_structure/trie_edit_distance.rs)|
|207|[Course Schedule](https://leetcode.com/problems/course-schedule/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/course_schedule_2.py)|BFS, topological_sorting|
|209|[Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/minimun_size_subarray_sum.py)|sliding_window|
|210|[Course Schedule II](https://leetcode.com/problems/course-schedule-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/course_schedule_2.py)|BFS, topological_sorting|
|211|[Design Add and Search Words..](https://leetcode.com/problems/design-add-and-search-words-data-structure/)|[Rust](src/data_structure/trie_edit_distance.rs)|
|212|[Word Search II](https://leetcode.com/problems/word-search-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/word_search_2.py)|前缀树|
|213|[House Robber II](https://leetcode.com/problems/house-robber-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/house_robber.py)||
|215|[Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)|quick_select, quick_sort, heap|
|216|[Combination Sum III](https://leetcode.com/problems/combination-sum-iii/)|[Rust](src/bfs_dfs_backtracking/combination_sum_iii.rs)|
|217|[Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)|[Rust](src/easy/leetcode_very_easy.rs)|
|219|[Contains Duplicate II](https://leetcode.com/problems/contains-duplicate-ii/)|[Rust](src/easy/leetcode_very_easy.rs)|
|222|[Count Complete Tree Nodes](https://leetcode.com/problems/count-complete-tree-nodes/)|[Rust](src/binary_tree/level_order_traversal.rs)|
|224|[Basic Calculator](https://leetcode.com/problems/basic-calculator/)|[Rust](src/compiler/mod.rs)|
|225|[Implement Stack using Queues](https://leetcode.com/problems/implement-stack-using-queues/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_stack_by_queue.py)|
|226|[Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/)|[Rust](src/binary_tree/invert_binary_tree.rs)|
|227|[Basic Calculator II](https://leetcode.com/problems/basic-calculator-ii/)|[Rust](src/compiler/basic_calculator_ii_no_parentheses.rs)|
|228|[Summary Ranges](https://leetcode.com/problems/summary-ranges/)|[Rust](src/easy/leetcode_very_easy.rs)|
|230|[Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|231|[Power of Two](https://leetcode.com/problems/power-of-two/)|[Rust](src/bitwise/is_power_of_x.rs)|
|232|[Implement Queue using Stacks](https://leetcode.com/problems/implement-queue-using-stacks/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_stack_by_queue.py)||
|234|[Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/)|[Rust](src/linked_list/linked_list_is_palindrome.rs)|
|235|[Lowest Common Ancestor of a Binary Search Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_lowest_common_ancestor.py)||
|236|[Lowest Common Ancestor of a Binary Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)|divide_and_conquer|
|238|[Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/)|[Rust](src/greedy/product_of_array_except_self.rs)|
|239|[Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/)|[Rust](src/data_structure/monotonic_queue_sliding_window_max.rs)|
|240|[Search a 2D Matrix II](https://leetcode.com/problems/search-a-2d-matrix-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/search_a_2d_matrix_2.py)|
|242|[Valid Anagram](https://leetcode.com/problems/valid-anagram/)|[Rust](src/counter/anagrams.rs)|
|257|[Binary Tree Paths](https://leetcode.com/problems/binary-tree-paths/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/root_to_leaf_paths_find_all.py)|DFS, backtracking|
|258|[Add Digits](https://leetcode.com/problems/add-digits/)|[Rust](src/easy/leetcode_very_easy.rs)|
|260|[Single Number III](https://leetcode.com/problems/single-number-iii/)|[Rust](src/bitwise/find_single_number.rs)|
|263|[Ugly Number](https://leetcode.com/problems/ugly-number/)|[Rust](src/math/mod.rs)|
|264|[Ugly Number II](https://leetcode.com/problems/ugly-number-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/ugly_number_2_nth_ugly.py)|
|266🔒|[Palindrome Permutation](https://leetcode.com/problems/palindrome-permutation/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/palindrome_permutation.py)|greedy|
|269🔒|[Alien Dictionary](https://leetcode.com/problems/alien-dictionary/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/alien_dictionary.py)|heapq, topological_sorting|
|270🔒|[Closest Binary Search Tree Value](https://leetcode.com/problems/closest-binary-search-tree-value/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value.py)||
|272🔒|[Closest Binary Search Tree Value II](https://leetcode.com/problems/closest-binary-search-tree-value-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value_2.py)||
|278|[First Bad Version](https://leetcode.com/problems/first-bad-version/)|[Rust](src/easy/leetcode_very_easy.rs)|
|279|[Perfect Squares](https://leetcode.com/problems/perfect-squares/)|[Rust](src/dp/coin_change.rs)|
|283|[Move Zeros](https://leetcode.com/problems/move-zeroes/)|[Rust](src/easy/array/partition_array.rs)|
|287|[Find the Duplicate Number](https://leetcode.com/problems/find-the-duplicate-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/find_duplicate_number.py)|快慢双指针|
|290|[Word Pattern](https://leetcode.com/problems/word-pattern/)|[Rust](src/easy/leetcode_very_easy.rs)|
|291🔒|[Word Pattern II](https://leetcode.com/problems/word-pattern-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/word_pattern_2.py)|DFS|
|292|[Nim Game](https://leetcode.com/problems/nim-game/)|[Rust](src/bitwise/nim_game.rs)|
|295|[Find Median from Data Stream](https://leetcode.com/problems/find-median-from-data-stream/)|[Rust](src/data_structure/heap/sliding_window_median.rs)|
|297|[Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree.py)|serialize|
|300|[Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/longest_increase_subsequence.py)|接龙型动态规划|
|302🔒|[Smallest Rectangle Enclosing Black Pixels](https://leetcode.com/problems/smallest-rectangle-enclosing-black-pixels)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/smallest_rectangle_enclosing_black_pixels.py)||
|303|[Range Sum Query - Immutable](https://leetcode.com/problems/range-sum-query-immutable/)|[Rust](src/easy/leetcode_very_easy.rs)|
|304|[Range Sum Query 2D - Immutable](https://leetcode.com/problems/range-sum-query-2d-immutable/)|[Rust](https://leetcode.com/problems/range-sum-query-2d-immutable/)|
|307|[Range Sum Query - Mutable](https://leetcode.com/problems/range-sum-query-mutable/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/segment_tree/segment_tree_range_sum.py)|
|309|[Best Time to Buy and Sell Stock with Cooldown](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|311🔒|[Sparse Matrix Multiplication](https://leetcode.com/problems/sparse-matrix-multiplication/)|[Rust](src/easy/leetcode_very_easy.rs)|
|312|[Burst Balloons](https://leetcode.com/problems/burst-balloons/)|[Rust](src/dp/burst_balloons.rs)|
|322|[Coin Change](https://leetcode.com/problems/coin-change/)|[Rust](src/dp/coin_change.rs)|
|326|[Power of Three](https://leetcode.com/problems/power-of-three/)|[Rust](src/bitwise/is_power_of_x.rs)|
|328|[Odd Even Linked List](https://leetcode.com/problems/odd-even-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/odd_even_linked_list.py)|
|337|[House Robber III](https://leetcode.com/problems/house-robber-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/house_robber.py)|
|338|[Counting Bits](https://leetcode.com/problems/counting-bits/)|[Rust](src/dp/counting_bits.rs)|
|341|[Flatten Nested List Iterator](https://leetcode.com/problems/flatten-nested-list-iterator/)|[Rust](src/bfs_dfs_backtracking/flatten_nested_list_iterator.rs)|
|342|[Power of Four](https://leetcode.com/problems/power-of-four/)|[Rust](src/bitwise/is_power_of_x.rs)|
|343|[Integer Break](https://leetcode.com/problems/integer-break/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/integer_break.py)|划分类DP|
|344|[Reverse String](https://leetcode.com/problems/reverse-string/)|[Rust](src/uncategorized/string_in_place.rs)|
|345|[Reverse Vowels of a String](https://leetcode.com/problems/reverse-vowels-of-a-string/)|[Rust](src/easy/leetcode_very_easy.rs)|
|347|[Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/)|[Rust](src/data_structure/heap/top_k_frequent_elements.rs)|
|349|[Intersection of Two Arrays](https://leetcode.com/problems/intersection-of-two-arrays/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/intersection_of_two_arrays.py)||
|350|[Intersection of Two Arrays II](https://leetcode.com/problems/intersection-of-two-arrays-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/intersection_of_two_arrays.py)||
|354|[Russian Doll Envelopes](https://leetcode.com/problems/russian-doll-envelopes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/russian_doll_envelopes.py)|
|355|[Design Twitter](https://leetcode.com/problems/design-twitter/)|[Rust](src/easy/leetcode_very_easy.rs)|
|359🔒|[Logger Rate Limiter](https://leetcode.com/problems/logger-rate-limiter/)|[Rust](src/easy/leetcode_very_easy.rs)|
|367|[Valid Perfect Square](https://leetcode.com/problems/valid-perfect-square/)|[Rust](src/math/sqrt.rs)|
|368|[Largest Divisible Subset](https://leetcode.com/problems/largest-divisible-subset/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/largest_divisible_subset.py)||
|369🔒|[Plus One Linked List](https://leetcode.com/problems/plus-one-linked-list/)|[Rust](src/linked_list/plus_one.rs)|
|371❌|[Sum of Two Integers](https://leetcode.com/problems/sum-of-two-integers/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/binary_addition.py)|binary_addition|
|374|[Guess Number Higher Or Lower](https://leetcode.com/problems/guess-number-higher-or-lower/)|[Rust](src/easy/leetcode_very_easy.rs)|
|377|[Combination Sum IV](https://leetcode.com/problems/combination-sum-iv/)|[Rust](src/dp/coin_change.rs)|
|380|[Insert Delete GetRandom O(1)](https://leetcode.com/problems/insert-delete-getrandom-o1/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/insert_delete_get_random_o1.py)||
|381|[Insert Delete GetRandom O(1) - Duplicates allowed](https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/insert_delete_get_random_o1.py)||
|383|[Ransom Note](https://leetcode.com/problems/ransom-note/)|[Rust](src/counter/mod.rs)|
|386|[Lexicographical Numbers](https://leetcode.com/problems/lexicographical-numbers/)|[Rust](src/bfs_dfs_backtracking/lexicographical_numbers.rs)|
|387|[First Unique Character in a String](https://leetcode.com/problems/first-unique-character-in-a-string/)|[Rust](src/counter/mod.rs)|
|389|[Find The Difference](https://leetcode.com/problems/find-the-difference/)|[Rust](src/bitwise/find_single_number.rs)|
|398|[Random Pick Index](https://leetcode.com/problems/random-pick-index/)|[Rust](src/random/random_pick_index.rs)|
|404|[Sum of Left Leaves](https://leetcode.com/problems/sum-of-left-leaves/)|[Rust](src/binary_tree/sum_of_left_leaves.rs)|
|406|[Queue Reconstruction by Height](https://leetcode.com/problems/queue-reconstruction-by-height/)|[Rust](src/easy/leetcode_very_easy.rs)|
|409|[Longest Palindrome](https://leetcode.com/problems/longest-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_combination.py)|dp(greedy)|
|412|[Fizz Buzz](https://leetcode.com/problems/fizz-buzz/)|[Rust](src/easy/leetcode_very_easy.rs)|
|413|[Arithmetic Slices](https://leetcode.com/problems/arithmetic-slices/)|[Rust](src/easy/leetcode_very_easy.rs)|
|415|[Add String](https://leetcode.com/problems/add-strings/)|[C++](https://github.com/pymongo/learn_cpp/blob/master/leetcode/add_bigint_string.cpp)|
|416|[Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)|0-1背包问题|
|426|[Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/longest_repeating_character_replacement.py)||
|429|[N-ary Tree Level Order Traversal](https://leetcode.com/problems/n-ary-tree-level-order-traversal/)|[C++](https://github.com/pymongo/learn_cpp/blob/master/leetcode/bfs/n_ary_tree_level_order.cpp)||
|448|[Find All Numbers Disappeared in an Array](https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/)|[Rust](src/counter/find_all_numbers_disappeared_in_an_array.rs)|
|449|[Serialize and Deserialize BST](https://leetcode.com/problems/serialize-and-deserialize-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_serialize.py)|DFS, stack|
|451|[Sort Characters By Frequency](https://leetcode.com/problems/sort-characters-by-frequency/)|[Rust](src/counter/mod.rs)|
|454|[4Sum II](https://leetcode.com/problems/4sum-ii/)|[Rust](src/two_sum_two_pointers/four_sum_2.rs)|
|455|[Assign Cookies](https://leetcode.com/problems/assign-cookies/)|[Rust](src/easy/leetcode_very_easy.rs)|
|457|[Circular Array Loop](https://leetcode.com/problems/circular-array-loop/)|[Rust](src/linked_list/is_circular_loop.rs)|
|461|[Hamming Distance](https://leetcode.com/problems/hamming-distance/)|[Rust](src/bitwise/hamming_distance_count_ones.rs)|
|463|[Island Perimeter](https://leetcode.com/problems/island-perimeter/)|[Rust](src/easy/grid_or_matrix/island_perimeter.rs)|
|470|[impl rand10 using rand7](https://leetcode.com/problems/implement-rand10-using-rand7/)|[Rust](src/random/impl_rand10_by_rand7.rs)|
|474|[Ones and Zeroes](https://leetcode.com/problems/ones-and-zeroes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_ones_and_zeros.py)|完全背包问题|
|477|[Total Hamming Distance](https://leetcode.com/problems/total-hamming-distance/)|[Rust](src/bitwise/hamming_distance_count_ones.rs)|
|480|[Sliding Window Median](https://leetcode.com/problems/sliding-window-median/)|[Rust](src/data_structure/heap/sliding_window_median.rs)|
|485|[Max Consecutive Ones](https://leetcode.com/problems/max-consecutive-ones/)|[Rust](src/easy/leetcode_very_easy.rs)|
|486|[Validate IP Address](https://leetcode.com/problems/validate-ip-address/)|[Rust](src/easy/string/parse_ip_address.rs)|
|490🔒|[The Maze](https://leetcode.com/problems/the-maze/)|[Rust](src/bfs_dfs_backtracking/the_maze.rs)|
|494|[Target Sum](https://leetcode.com/problems/target-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_target_sum.py)|0-1背包问题|
|498|[Diagonal Traverse](https://leetcode.com/problems/diagonal-traverse/)|[Rust](src/easy/grid_or_matrix/matrix_diagonal_traverse.rs)|
|500|[Keyboard Row](https://leetcode.com/problems/keyboard-row/)|[Rust](src/easy/leetcode_very_easy.rs)|
|503|[Next Greater Element II](https://leetcode.com/problems/next-greater-element-ii/)|[Rust](src/data_structure/monotonic_stack_next_greater_element_2.rs)|
|504|[Base 7](https://leetcode.com/problems/base-7/)|[Rust](src/easy/leetcode_very_easy.rs)|
|507|[Perfect Number](https://leetcode.com/problems/perfect-number/)|[Rust](src/math/perfect_number.rs)|
|509|[Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)|[Rust](src/dp/fibonacci.rs)|
|514|[Freedom Trail](https://leetcode.com/problems/freedom-trail/)|[Rust](src/dp/freedom_trail.rs)|
|515|[Find Largest Value...](https://leetcode.com/problems/find-largest-value-in-each-tree-row/)|[Rust](src/binary_tree/level_order_traversal.rs)|
|518|[Coin Change 2](https://leetcode.com/problems/coin-change-2/)|[Rust](src/dp/coin_change.rs)|
|523|[Continuous Subarray Sum](https://leetcode.com/problems/continuous-subarray-sum/)|[Rust](src/greedy/prefix_sum_subarray.rs)|
|525|[Contiguous Array](https://leetcode.com/problems/contiguous-array/)|[Rust](src/greedy/prefix_sum_subarray.rs)|
|535|[Encode and Decode TinyURL](https://leetcode.com/problems/encode-and-decode-tinyurl/)|very_easy|
|536🔒|[Construct Binary Tree from String](https://leetcode.com/problems/construct-binary-tree-from-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_string_with_parentheses.py)||
|538|[Convert BST to Greater Tree](https://leetcode.com/problems/convert-bst-to-greater-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_to_gst.py)|
|540|[Single Element in a Sorted Array](https://leetcode.com/problems/single-element-in-a-sorted-array)|[Rust](src/binary_search/single_element_in_a_sorted_array.rs)|
|541|[Reverse String II](https://leetcode.com/problems/reverse-string-ii/)|[Rust](src/uncategorized/string_in_place.rs)|
|547|[Friend Circles](https://leetcode.com/problems/number-of-provinces/)|[Rust](src/data_structure/union_find/friend_circles.rs)|
|554|[Brick Wall](https://leetcode.com/problems/brick_wall/)|[Rust](src/easy/leetcode_very_easy.rs)|
|557|[Reverse Words in a String III](https://leetcode.com/problems/reverse-words-in-a-string-iii/)|[Rust](src/easy/leetcode_very_easy.rs)|
|559|[Maximum Depth of N-ary Tree](https://leetcode.com/problems/maximum-depth-of-n-ary-tree/)|[C++](https://github.com/pymongo/learn_cpp/blob/master/leetcode/n_ary_tree_level_order.cpp)||
|566|[Reshape the Matrix](https://leetcode.com/problems/reshape-the-matrix/)|[Rust](src/easy/leetcode_easy.rs)|
|575|[Distribute Candies](https://leetcode.com/problems/distribute-candies/)|[Rust](src/easy/leetcode_easy.rs)|
|589|[N-ary Tree Preorder Traversal](https://leetcode.com/problems/n-ary-tree-preorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/n_ary_tree_preorder_postorder.py)||
|590|[N-ary Tree Postorder Traversal](https://leetcode.com/problems/n-ary-tree-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/n_ary_tree_preorder_postorder.py)||
|593|[Valid Square](https://leetcode.com/problems/valid-square/)|[Rust](src/easy/leetcode_easy.rs)|
|595|[Big Countries](https://leetcode.com/problems/big-countries/)|sql|
|606|[Construct String from Binary Tree](https://leetcode.com/problems/construct-string-from-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_string_with_parentheses.py)||
|617|[Merge Two Binary Trees](https://leetcode.com/problems/merge-two-binary-trees/)|[Rust](src/binary_tree/merge_two_binary_trees.rs)|
|605|[Can Place Flowers](https://leetcode.com/problems/can-place-flowers/)|[Rust](src/easy/leetcode_easy.rs)|
|610🔒|[Triangle Judgement](https://leetcode.com/problems/triangle-judgement/)|[SQL](sql_problems/very_easy.sql)|
|611|[Valid Triangle Number](https://leetcode.com/problems/valid-triangle-number/)|[Rust](src/two_sum_two_pointers/valid_triangle_number.rs)|
|613🔒|[Shortest Distance in a Line](https://leetcode.com/problems/shortest-distance-in-a-line/)|[SQL](sql_problems/shortest_distance_in_a_line.sql)|
|620|[Not Boring Movies](https://leetcode.com/problems/not-boring-movies/)|[SQL](sql_problems/very_easy.sql)|
|625🔒|[Minimum Factorization](https://leetcode.com/problems/minimum-factorization/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/minimum_factorization.py)|greedy|
|627|[Swap Salary](https://leetcode.com/problems/swap-salary/)|[SQL](sql_problems/invert_sex.sql)|
|628|[Maximum Product of Three Numbers](https://leetcode.com/problems/maximum-product-of-three-numbers/)|[Rust](src/easy/leetcode_easy.rs)|
|633|[Average of Levels in Binary Tree](https://leetcode.com/problems/average-of-levels-in-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_level_order.py)||
|643|[Maximum Average Subarray I](https://leetcode.com/problems/maximum-average-subarray-i/)|[Rust](src/easy/leetcode_very_easy.rs)|
|645|[Set Mismatch](https://leetcode.com/problems/set-mismatch/)|[Rust](src/easy/leetcode_very_easy.rs)|
|650|[2 Keys Keyboard](https://leetcode.com/problems/2-keys-keyboard/)|[Rust](src/easy/leetcode_easy.rs)|
|653|[Tow Sum IV - Input is a BST](https://leetcode.com/problems/two-sum-iv-input-is-a-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_two_sum.py)||
|657|[Robot Return to Origin](https://leetcode.com/problems/robot-return-to-origin/)|[Rust](src/easy/leetcode_very_easy.rs)|
|658|[Find K Closest Elements](https://leetcode.com/problems/find-k-closest-elements/)|[Rust](src/easy/leetcode_very_easy.rs), [Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/find_k_closest_elements.py)|binary_search|
|661|[Image Smoother](https://leetcode.com/problems/image-smoother/)|[Rust](src/easy/leetcode_very_easy.rs)|
|669|[Trim A Binary Search Tree](https://leetcode.com/problems/trim-a-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_trim_in_range.py)|
|674|[Longest Continuous Increasing Subsequence](https://leetcode.com/problems/longest-continuous-increasing-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/longest_continuous_increase_subsequence.py)||
|680|[Valid Palindrome II](https://leetcode.com/problems/valid-palindrome-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome_2.py)|two_pointers, greedy|
|682|[Baseball Game](https://leetcode.com/problems/baseball-game/)|[Rust](src/easy/leetcode_very_easy.rs)|
|690|[Employee Importance](https://leetcode.com/problems/employee-importance/)|[java](https://github.com/pymongo/java_leetcode/blob/master/src/main/java/com/leetcode/Easy.java)|
|692|[Top K Frequent Words](https://leetcode.com/problems/top-k-frequent-words/)|[Rust](src/data_structure/heap/top_k_frequent_elements.rs)|
|693|[Binary Number With Alternating Bits](https://leetcode.com/problems/binary-number-with-alternating-bits/)|[Rust](src/easy/leetcode_very_easy.rs)|
|695|[Max Area of Island](https://leetcode.com/problems/max-area-of-island/)|[Rust](src/bfs_dfs_backtracking/max_area_of_island.rs)|
|696|[Count Binary Substrings](https://leetcode.com/problems/count-binary-substrings/)|[Rust](src/easy/leetcode_very_easy.rs)|
|700|[Search in a Binary Search Tree](https://leetcode.com/problems/search-in-a-binary-search-tree/)|[Rust](src/binary_tree/search_val_or_range_in_bst.rs)|
|702🔒|[Search in a Sorted Array of Unknown Size](https://leetcode.com/problems/search-in-a-sorted-array-of-unknown-size/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search_unknown_size_sorted_array.py)|binary_search_first, 倍增法|
|703|[Kth Largest Element in a Stream](https://leetcode.com/problems/kth-largest-element-in-a-stream/)|[Rust](src/data_structure/heap/kth_largest_element_in_a_stream.rs)|
|704|[Binary Search](https://leetcode.com/problems/binary-search/)|[Rust](src/binary_search/mod.rs)|
|705|[Design HashSet](https://leetcode.com/problems/design-hashset/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/impl_hashmap.py)||
|706|[Design HashMap](https://leetcode.com/problems/design-hashmap/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/impl_hashmap.py)||
|709|[To Lower Case](https://leetcode.com/problems/to-lower-case/)|[Rust](src/easy/leetcode_very_easy.rs)|
|713|[1-bit and 2-bit Characters](https://leetcode.com/problems/1-bit-and-2-bit-characters/)|[Rust](src/easy/leetcode_easy.rs)|
|714|[Best Time to Buy and Sell Stock with Transaction Fee](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|716🔒|[Max Stack](https://leetcode.com/problems/max_stack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/max_stack.py)||
|724|[Find Pivot Index](https://leetcode.com/problems/find-pivot-index/)|[Rust](src/easy/leetcode_very_easy.rs)|
|728|[Self Dividing Numbers](https://leetcode.com/problems/self-dividing-numbers/)|[Rust](src/easy/leetcode_very_easy.rs)|
|743|[Network Delay Time](https://leetcode.com/problems/network-delay-time/)|[Rust](src/bfs_dfs_backtracking/dijkstra.rs)|
|746|[Min Cost Climbing Stairs](https://leetcode.com/problems/min-cost-climbing-stairs/)|[Rust](src/dp/dp_easy.rs)|
|760🔒|[Find Anagram Mappings](https://leetcode.com/problems/find-anagram-mappings/)|[Rust](src/counter/anagrams.rs)|
|763|[Partition Labels](https://leetcode.com/problems/partition-labels/)|[Rust](src/greedy/partition_labels.rs)|
|766|[Toeplitz Matrix](https://leetcode.com/problems/toeplitz-matrix/)|[Rust](src/easy/grid_or_matrix/matrix_diagonal_traverse.rs)|
|771|[Jewels and Stones](https://leetcode.com/problems/jewels-and-stones/)|[Rust](src/easy/leetcode_very_easy.rs)|
|787|[Cheapest Flights Within K Stops](https://leetcode.com/problems/cheapest-flights-within-k-stops/)|[Rust](src/bfs_dfs_backtracking/dijkstra.rs)|
|788|[Rotated Digits](https://leetcode.com/problems/rotated-digits/)|[Rust](src/easy/leetcode_easy.rs)|
|796|[Rotate String](https://leetcode.com/problems/rotate-string/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/rotate_string.py)|Rabin-Karp(rolling_hash), kmp|
|797|[All Paths From Source to Target](https://leetcode.com/problems/all-paths-from-source-to-target/)|[Rust](src/bfs_dfs_backtracking/all_paths_from_source_to_target.rs)|
|807|[Max Increase to Keep City Skyline](https://leetcode.com/problems/max-increase-to-keep-city-skyline/)|[Rust](src/easy/leetcode_very_easy.rs)|
|830|[Positions of Large Groups](https://leetcode.com/problems/positions-of-large-groups/)|[Rust](src/easy/leetcode_very_easy.rs)|
|832|[Flipping an Image](https://leetcode.com/problems/flipping-an-image/)|[Rust](src/easy/leetcode_very_easy.rs)|
|841|[Keys and Rooms](https://leetcode.com/problems/keys-and-rooms/)|[Rust](src/easy/leetcode_very_easy.rs)|
|844|[Backspace String Compare](https://leetcode.com/problems/backspace-string-compare/)|[Rust](src/easy/leetcode_very_easy.rs)||
|845|[Longest Mountain in Array](https://leetcode.com/problems/longest-mountain-in-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_longest.py)|mountain_array|
|852|[Peak Index in a Mountain Array](https://leetcode.com/problems/peak-index-in-a-mountain-array/)|[Rust](src/binary_search/mountain_array.rs)|
|860|[Lemonade Change](https://leetcode.com/problems/lemonade-change/)|[Rust](src/easy/leetcode_very_easy.rs)|
|861|[Score After Flipping Matrix](https://leetcode.com/problems/score-after-flipping-matrix/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/score_after_flipping_matrix.py)|greedy|
|867|[Transpose Matrix](https://leetcode.com/problems/transpose-matrix/)|[Rust](src/easy/leetcode_very_easy.rs), [Go](https://github.com/pymongo/go_leetcode/blob/master/easy/transpose_matrix_test.go)|
|869|[Reordered Power of 2](https://leetcode.com/problems/reordered-power-of-2/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/reordered_power_of_2.py)|permutation|
|872|[Leaf Similar Trees](https://leetcode.com/problems/leaf-similar-trees/)|[Rust](src/binary_tree/leaf_similar_trees.rs)|
|875|[Koko Eating Bananas](https://leetcode.com/problems/koko-eating-bananas/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/koko_eating_bananas.py)|
|876|[Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list/)|[Rust](src/linked_list/middle_of_linked_list.rs)|
|877|[Stone Game](https://leetcode.com/problems/stone-game/)|[Rust](src/dp/stone_game.rs)|
|881|[Boats to Save People](https://leetcode.com/problems/boats-to-save-people/)|[Rust](src/easy/leetcode_very_easy.rs)|
|887|[Super Egg Drop](https://leetcode.com/problems/super-egg-drop)|[Rust](src/dp/drop_eggs.rs)|
|888|[Fair Candy Swap](https://leetcode.com/problems/fair-candy-swap/)|[Rust](src/two_sum_two_pointers/two_sum_relative.rs)|
|889|[Construct Binary Tree from Preorder and Postorder...](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/construct_from_pre_order_and_post_order.py)|DFS|
|905|[Sort Array By Parity](https://leetcode.com/problems/sort-array-by-parity/)|[Rust](src/easy/array/partition_array.rs)|
|912|[Sort An Array](https://leetcode.com/problems/sort-an-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/basic_sorting.py)|
|914|[Partition Array Into Disjoint...](https://leetcode.com/problems/partition-array-into-disjoint-intervals/)|[Rust](/src/easy/leetcode_easy.rs)|
|922|[Sort Array By Parity II](https://leetcode.com/problems/sort-array-by-parity-ii/)|[Rust](src/easy/array/partition_array.rs)|
|925|[Long Pressed Name](https://leetcode.com/problems/long-pressed-name/)|[Rust](src/easy/string/long_pressed_name.rs)|
|930|[Binary Subarrays With Sum](https://leetcode.com/problems/binary-subarrays-with-sum/)|[Rust](src/greedy/prefix_sum_subarray.rs)|
|938|[Range Sum of BST](https://leetcode.com/problems/range-sum-of-bst/)|[Rust](src/binary_tree/search_val_or_range_in_bst.rs)|
|941|[Valid Mountain Array](https://leetcode.com/problems/find-in-mountain-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_valid.py)|mountain_array|
|942|[DI String Match](https://leetcode.com/problems/di-string-match/)|[Rust](src/greedy/di_string_match.rs)|
|944|[Delete Columns to Make Sorted](https://leetcode.com/problems/delete-columns-to-make-sorted/)|[Rust](src/easy/leetcode_very_easy.rs)|
|950|[Reveal Cards In Increasing Order](https://leetcode.com/problems/reveal-cards-in-increasing-order/)|[Rust](src/easy/leetcode_easy.rs)|
|953|[Verifying an Alien Dictionary](https://leetcode.com/problems/verifying-an-alien-dictionary/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/verifying_an_alien_dictionary.py)|
|961|[N-Repeated Element in Size 2N Array](https://leetcode.com/problems/n-repeated-element-in-size-2n-array/)|[Rust](src/easy/leetcode_very_easy.rs)|
|965|[Univalued Binary Tree](https://leetcode.com/problems/univalued-binary-tree/)|[Rust](src/binary_tree/univalued_binary_tree.rs)|
|973|[K Closest Points to Origin](https://leetcode.com/problems/k-closest-points-to-origin/)|[Rust](src/easy/leetcode_very_easy.rs)|quick_select|
|976|[Largest Perimeter Triangle](https://leetcode.com/problems/largest-perimeter-triangle/)|[Rust](src/easy/leetcode_easy.rs)|
|977|[Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/)|[Rust](src/easy/array/squares_of_a_sorted_array.rs)|
|989|[Add to Array-Form of Integer](https://leetcode.com/problems/add-to-array-form-of-integer/)|[Rust](src/easy/leetcode_very_easy.rs)|
|993|[Cousins In Binary Tree](https://leetcode.com/problems/cousins-in-binary-tree/)|[Rust](src/binary_tree/level_order_traversal.rs)|
|1002|[Find Common Characters](https://leetcode.com/problems/find-common-characters/)|[Rust](src/easy/string/find_common_characters.rs)|
|1006|[Clumsy Factorial](https://leetcode.com/problems/clumsy-factorial/)|[Rust](src/compiler/mod.rs)|
|1018|[Binary Prefix Divisible By 5](https://leetcode.com/problems/binary-prefix-divisible-by-5/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1030|[Matrix Cells in Distance Order](https://leetcode.com/problems/matrix-cells-in-distance-order/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1038|[Binary Search Tree to Greater Sum Tree](https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_to_gst.py)||
|1046|[Last Stone Weight](https://leetcode.com/problems/last-stone-weight/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/last_stone_weight.py)||
|1049|[Last Stone Weight II](https://leetcode.com/problems/last-stone-weight-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)|0-1背包问题|
|1051|[Height Checker](https://leetcode.com/problems/height-checker/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1068🔒|[Product Sales Analysis I](https://leetcode.com/problems/product-sales-analysis-i/)|[SQL](sql_problems/very_easy.sql)|
|1069🔒|[Product Sales Analysis II](https://leetcode.com/problems/product-sales-analysis-ii/)|[SQL](sql_problems/very_easy.sql)|
|1089|[Duplicate Zeros](https://leetcode.com/problems/duplicate-zeros/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1095|[Find in Mountain Array](https://leetcode.com/problems/find-in-mountain-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/mountain_array_find.py)|binary_search, mountain_array|
|1099🔒|[Two Sum Less Than K](https://leetcode.com/problems/two-sum-less-than-k/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_le_count.py)|two_pointers|
|1108|[Defanging an IP Address](https://leetcode.com/problems/defanging-an-ip-address/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1128|[Number of Equivalent Domino Pairs](https://leetcode.com/problems/number-of-equivalent-domino-pairs/)|[Rust](src/counter/number_of_equivalent_domino_pairs.rs)|
|1109|[Corporate Flight Bookings](https://leetcode.com/problems/corporate-flight-bookings/)|[Rust](src/easy/leetcode_easy.rs)|
|1119🔒|[Remove Vowels from a String](https://leetcode.com/problems/remove-vowels-from-a-string/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1134🔒|[Armstrong Number](https://leetcode.com/problems/armstrong-number/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1143|[Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/longest_common_subsequence.py)|
|1160|[Find Words That Can Be Formed by Ch...](https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/)|[Rust](src/counter/mod.rs)|
|1167🔒|[Minimum Cost To Connect Sticks](https://leetcode.com/problems/minimum-cost-to-connect-sticks/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1180🔒|[Count Substrings with Only One Distinct Letter](https://leetcode.com/problems/count-substrings-with-only-one-distinct-letter/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1189|[Maximum Number of Balloons](https://leetcode.com/problems/maximum-number-of-balloons)|[Rust](src/easy/leetcode_very_easy.rs)|
|1207|[Unique Number of Occurrences](https://leetcode.com/problems/unique-number-of-occurrences/)|[Rust](src/counter/mod.rs)|
|1213🔒|[Intersection of Three Sorted Arrays](https://leetcode.com/problems/intersection-of-three-sorted-arrays/)|[Rust](src/counter/mod.rs)|
|1221|[Split a String In..](https://leetcode.com/problems/split-a-string-in-balanced-strings/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1226|[The Dining Philosophers](https://leetcode.com/problems/the-dining-philosophers/)|[C++](https://github.com/pymongo/learn_cpp/blob/master/leetcode/dining_philosophers_problem.cpp)|
|1227|[Airplane Seat Assignment Probability](https://leetcode.com/problems/airplane-seat-assignment-probability/)|[Rust](src/greedy/airplane_seat_assignment_probability.rs)|
|1232|[Check If It Is a Straight Line](https://leetcode.com/problems/check-if-it-is-a-straight-line/)|[Rust](src/easy/leetcode_easy.rs)|
|1248|[Count Number of Nice Subarrays](https://leetcode.com/problems/count-number-of-nice-subarrays/)|[Rust](src/greedy/count_number_of_nice_subarrays.rs)|
|1251🔒|[Average Selling Price](https://leetcode.com/problems/average-selling-price/)|[SQL](sql_problems/very_easy.sql)|
|1252|[Cells with Odd Values in a Matrix](https://leetcode.com/problems/cells-with-odd-values-in-a-matrix/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1260|[Shift 2D Grid](https://leetcode.com/problems/shift-2d-grid/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1265🔒|[Print Immutable Linked List in Reverse](https://leetcode.com/problems/print-immutable-linked-list-in-reverse/)|[C](https://github.com/pymongo/learn_cpp/blob/master/leetcode/easy/print_immutable_linked_list_in_reverse.c)|
|1266|[Minimum Time Visiting All Points](https://leetcode.com/problems/minimum-time-visiting-all-points/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1269|[Number of Ways to Stay in the Same...](https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps/)|[Rust](src/dp/number_of_ways_to_stay_in_the_same_place_after_some_steps.rs)|
|1281|[Subtract the Product and Sum of Digits...](https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/)|[Rust](src/easy/leetcode_very_easy.rs)
|1282|[Group the people...](https://leetcode.cn/problems/group-the-people-given-the-group-size-they-belong-to)|[Rust](src/counter/mod.rs)
|1295|[Find Numbers with Even Number of Digits](https://leetcode.com/problems/find-numbers-with-even-number-of-digits/)|[Rust](src/easy/array/find_numbers_with_even_number_of_digits.rs)|
|1299|[Replace Elements with Greatest Element on Right...](https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1302|[Deepest Leaves Sum](https://leetcode.com/problems/deepest-leaves-sum/)|[Rust](src/binary_tree/level_order_traversal.rs)|
|1303🔒|[Find the Team Size](httpsn://leetcode.com/problems/find-the-team-size/)|[SQL](sql_problems/very_easy.sql)|
|1313|[Decompress Run-Length Encoded List](https://leetcode.com/problems/decompress-run-length-encoded-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/decompress_run_length_encoded_list.py)|
|1329|[Sort The Matrix Diagonally](https://leetcode.com/problems/sort-the-matrix-diagonally/)|[Rust](src/easy/grid_or_matrix/matrix_diagonal_traverse.rs)|
|1331|[Rank Transform Of An Array](https://leetcode.com/problems/rank-transform-of-an-array/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1337|[The K Weakest Rows in a Matrix](https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1342|[Number of Steps to Reduce a Number to Zero](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/)|[Rust](src/easy/leetcode_easy.rs)|
|1346|[Check If N and Its Double Exist](https://leetcode.com/problems/check-if-n-and-its-double-exist/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1351|[Count Negative Numbers in a Sorted Matrix](https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/)|[Rust](src/easy/grid_or_matrix/count_negative_numbers_in_a_sorted_matrix.rs)|
|1356|[Sort Integers by The Number of 1 Bits](https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/)|[Rust](src/bitwise/hamming_distance_count_ones.rs)|
|1365|[How Many Numbers Are Smaller Than the Current Number](https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/)|[Rust](src/counter/mod.rs)|
|1370|[Increasing Decreasing String](https://leetcode.com/problems/increasing-decreasing-string/)|[Rust](src/counter/increasing_decreasing_string.rs)|
|1374|[Generate a String With Characters That Have Odd Counts](https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/)|[Rust](src/counter/mod.rs)|
|1379|[Find a Corresponding Node of a Binary Tree ...](https://leetcode.com/problems/find-a-corresponding-node-of-a-binary-tree-in-a-clone-of-that-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/find_a_corresponding_node_in_a_cloned_tree.py)||
|1380|[Lucky Numbers in a Matrix](https://leetcode.com/problems/lucky-numbers-in-a-matrix/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1389|[Create Target Array in the Given Order](https://leetcode.com/problems/create-target-array-in-the-given-order/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1395|[Count Number of Teams](https://leetcode.com/problems/count-number-of-teams/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1403|[Minimum Subsequence...](https://leetcode.com/problems/minimum-subsequence-in-non-increasing-order/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1409|[Queries on a Permutation With Key](https://leetcode.com/problems/queries-on-a-permutation-with-key/)|[Rust](src/easy/array/queries_on_a_permutation_with_key.rs)|
|1418|[Display Table Of Food Order..](https://leetcode.com/problems/build-array-from-permutation)|[Rust](src/easy/leetcode_very_easy.rs)|
|1429🔒|[First Unique Number](https://leetcode.com/problems/first-unique-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/first_unique_number_in_stream.py)|
|1431|[Kids With the Greatest Number of Candies](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1436|[Destination City](https://leetcode.com/problems/destination-city/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1441|[Build An Array With Stack Op...](https://leetcode.com/problems/build-an-array-with-stack-operations/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1445🔒|[Apples Oranges](https://leetcode.com/problems/apples-oranges/)|[SQL](sql_problems/apples_oranges.sql)|
|1450|[Number of Students Doing Homework at ...](https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1464|[Maximum Product of Two Elements in an Array](https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1470|[Shuffle the Array](https://leetcode.com/problems/shuffle-the-array/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1475|[Final Prices With a Special Discount...](https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1476|[Subrectangle Queries](https://leetcode.com/problems/subrectangle-queries/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1480|[Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1486|[XOR Operation in an Array](https://leetcode.com/problems/xor-operation-in-an-array/)|[Rust](src/easy/leetcode_very_easy.rs), [Racket](https://github.com/pymongo/leetcode_racket/blob/main/leetcode_1486_xor_operation_in_an_array.rkt)|
|1502|[Can Make Arithmetic Progression From Sequence](https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1512|[Number of Good Pairs](https://leetcode.com/problems/number-of-good-pairs/)|[Rust](src/counter/number_of_good_pairs.rs)|
|1528|[Shuffle String](https://leetcode.com/problems/shuffle-string/)|[Rust](src/uncategorized/string_in_place.rs)|
|1534|[Count Good Triplets](https://leetcode.com/problems/count-good-triplets/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1551|[Minimum Operations to Make Array Equal](https://leetcode.com/problems/minimum-operations-to-make-array-equal/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1570|[Dot Product of Two Sparse Vectors](https://leetcode.com/problems/dot-product-of-two-sparse-vectors/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1572|[Matrix Diagonal Sum](https://leetcode.com/problems/matrix-diagonal-sum/)|[Rust](src/easy/grid_or_matrix/mod.rs)|
|1576|[Replace All ?'s to Avoid Consecutive ...](https://leetcode.com/problems/replace-all-s-to-avoid-consecutive-repeating-characters/)|[Rust](src/uncategorized/string_in_place.rs)|
|1577|[Number of Ways Where Square of ...](https://leetcode.com/problems/number-of-ways-where-square-of-number-is-equal-to-product-of-two-numbers/)|[Rust](src/counter/count_num1_square_eq_two_num2_product.rs)|
|1578|[Minimum Deletion Cost to Avoid Repeating ...](https://leetcode.com/problems/minimum-deletion-cost-to-avoid-repeating-letters/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1582|[Special Positions in a Binary Matrix](https://leetcode.com/problems/special-positions-in-a-binary-matrix/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1584|[Min Cost to Connect All Points](https://leetcode.com/problems/min-cost-to-connect-all-points/)|[Rust](src/data_structure/union_find/min_cost_to_connect_all_points.rs)|
|1588|[Sum of All Odd Length Subarrays](https://leetcode.com/problems/sum-of-all-odd-length-subarrays/)|[Rust](src/easy/array/sum_of_all_odd_length_subarrays.rs)|
|1592|[Rearrange Spaces Between Words](https://leetcode.com/problems/rearrange-spaces-between-words/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1598|[Crawler Log Folder](https://leetcode.com/problems/crawler-log-folder/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1601|[Design Parking System](https://leetcode.com/problems/design-parking-system/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1614|[Maximum Nesting Depth of the Parentheses](https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/)|[Rust](src/easy/leetcode_easy.rs)|
|1619|[Mean Of Array...](https://leetcode.com/problems/mean-of-array-after-removing-some-elements/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1636|[Sort Array by Increasing Frequency](https://leetcode.com/problems/sort-array-by-increasing-frequency/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1637|[Widest Vertical Area Between Two Points ...](https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1640|[Check Array Formation Through Concatenation](https://leetcode.com/problems/check-array-formation-through-concatenation/)|[Rust](src/easy/leetcode_easy.rs)|
|1646|[Get Maximum In Gen...](https://leetcode.com/problems/get-maximum-in-generated-array/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1656|[Design an Ordered Stream](https://leetcode.com/problems/design-an-ordered-stream/)|[Rust](src/easy/leetcode_easy.rs)|
|1658|[Defuse The Bomb](https://leetcode.com/problems/defuse-the-bomb/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1662|[Check If Two String Arrays are Equivalent](https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1668|[Maximum Repeating Substring](https://leetcode.com/problems/maximum-repeating-substring/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1672|[Richest Customer Wealth](https://leetcode.com/problems/richest-customer-wealth/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1678|[Goal Parser Interpretation](https://leetcode.com/problems/goal-parser-interpretation/)|[Rust](src/easy/leetcode_easy.rs)|
|1683🔒|[Invalid Tweets](https://leetcode.com/problems/invalid-tweets/)|[SQL](sql_problems/very_easy.sql)|
|1684|[Count the Number of Consistent Strings](https://leetcode.com/problems/count-the-number-of-consistent-strings/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1688|[Count of Matches in Tournament](https://leetcode.com/problems/count-of-matches-in-tournament/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1694|[Reformat Phone Number](https://leetcode.com/problems/reformat-phone-number/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1700|[Number of Students Unable to Eat Lunch](https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/)|[Rust](src/easy/leetcode_easy.rs)|
|1704|[Determine If String...](https://leetcode.com/problems/determine-if-string-halves-are-alike/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1710|[Maximum Units on a Truck](https://leetcode.com/problems/maximum-units-on-a-truck/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1711|[Count Good Meals](https://leetcode.com/problems/count-good-meals/)|[Rust](src/counter/count_good_meals.rs)|
|1716|[Calculate Money in Leetcode Bank](https://leetcode.com/problems/calculate-money-in-leetcode-bank/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1720|[Decode XORed Array](https://leetcode.com/problems/decode-xored-array/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1725|[Number Of Rectangles That Can ...](https://leetcode.com/problems/number-of-rectangles-that-can-form-the-largest-square/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1732|[Find the Highest Altitude](https://leetcode.com/problems/find-the-highest-altitude/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1736|[Latest Time by Replacing ...](https://leetcode.com/problems/latest-time-by-replacing-hidden-digits/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1741🔒|[Find Total Time Spent by Each Employee](https://leetcode.com/problems/find-total-time-spent-by-each-employee/)|[SQL](sql_problems/very_easy.sql)|
|1742|[Maximum Number of Ball...](https://leetcode.cn/problems/maximum-number-of-balls-in-a-box/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1744|[Can You Eat Your Favorite Candy...](https://leetcode.com/problems/can-you-eat-your-favorite-candy-on-your-favorite-day/)|[Rust](src/easy/leetcode_easy.rs)|
|1748|[Sum of Unique Elements](https://leetcode.com/problems/sum-of-unique-elements/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1752|[Check if Array Is Sorted and Rotated](https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1753|[Maximum Score From Removing Stones](https://leetcode.com/problems/maximum-score-from-removing-stones/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1754|[Largest Merge Of Two Strings](https://leetcode.com/problems/largest-merge-of-two-strings/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1773|[Count Items Matching a Rule](https://leetcode.com/problems/count-items-matching-a-rule/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1779|[Find Nearest...](https://leetcode.cn/problems/find-nearest-point-that-has-the-same-x-or-y-coordinate/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1790|[Check if One String Swap Can Make String...](https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1791|[Find Center of Star Graph](https://leetcode.com/problems/find-center-of-star-graph/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1796|[Second Largest...](https://leetcode.cn/problems/second-largest-digit-in-a-string)|[Rust](src/easy/leetcode_very_easy.rs)|
|1805|[Number Of Diff...](https://leetcode.cn/problems/number-of-different-integers-in-a-string/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1812|[Determine Color...](https://leetcode.cn/problems/determine-color-of-a-chessboard-square/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1816|[Truncate Sentence](https://leetcode.com/problems/truncate-sentence/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1822|[Sign Of The Product...](https://leetcode.com/problems/sign-of-the-product-of-an-array/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1828|[Queries on Number of Points Inside a Circle](https://leetcode.com/problems/queries-on-number-of-points-inside-a-circle/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1832|[Check if the Sentence Is Pangram](https://leetcode.com/problems/check-if-the-sentence-is-pangram/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1833|[Maximum Ice Cream Bars](https://leetcode.com/problems/maximum-ice-cream-bars/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1834|[Single Threaded Cpu](https://leetcode.com/problems/single-threaded-cpu/)|[Rust](src/data_structure/heap/single_threaded_cpu.rs)|
|1837|[Sum of Digits in Base K](https://leetcode.com/problems/sum-of-digits-in-base-k/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1839|[Longest Substring Of All Vowels in Order](https://leetcode.com/problems/longest-substring-of-all-vowels-in-order/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1844|[Replace All Digits with Characters](https://leetcode.com/problems/replace-all-digits-with-characters/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1845|[Seat Reservation Manager](https://leetcode.com/problems/seat-reservation-manager/)|[Rust](src/data_structure/heap/seat_reservation_manager.rs)|
|1848|[Minimum Distance to the...t](https://leetcode.com/problems/minimum-distance-to-the-target-element/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1854|[Maximum Population Year](https://leetcode.com/problems/maximum-population-year/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1863|[Sum of All Subset XOR Totals](https://leetcode.com/problems/sum-of-all-subset-xor-totals/)|[Rust](src/bfs_dfs_backtracking/combinations_subsets.rs)|
|1877|[Minimize Maximum Pair...](https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1880|[Check if Word Equals...](https://leetcode.com/problems/check-if-word-equals-summation-of-two-words/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1893|[Check if All The Integer...](https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1913|[Maximum Product Diff...](https://leetcode.com/problems/maximum-product-difference-between-two-pairs/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1920|[Build Array from Permutation](https://leetcode.com/problems/build-array-from-permutation)|[Rust](src/easy/leetcode_very_easy.rs)|
|1925|[Count Square Sum Triples](https://leetcode.com/problems/count-square-sum-triples/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1929|[Concatenation of Array](https://leetcode.com/problems/concatenation-of-array/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1952|[Three Divisors](https://leetcode.com/problems/three-divisors/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1984|[Minimum Difference Between...](https://leetcode.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1985|[Find The Kth Large...](https://leetcode.com/problems/find-the-kth-largest-integer-in-the-array/)|[Rust](src/easy/leetcode_very_easy.rs)|
|1995|[Count Special Quadruplets](https://leetcode.com/problems/count-special-quadruplets/)|[Rust](src/easy/leetcode_very_easy.rs)|
|2000|[Count Special Quadruplets](https://leetcode.com/problems/reverse-prefix-of-word/)|[Rust](src/easy/leetcode_very_easy.rs)|
|2011|[Find Value Of...](https://leetcode.cn/problems/final-value-of-variable-after-performing-operations/)|[Rust](src/easy/leetcode_very_easy.rs)|
|2027|[Minimum moves...](https://leetcode.cn/problems/minimum-moves-to-convert-string/)|[Rust](src/easy/leetcode_very_easy.rs)|
|2032|[Two Out Of Three](https://leetcode.cn/problems/two-out-of-three/)|[Rust](src/easy/leetcode_very_easy.rs)|
|2042|[Check If Numbers...](https://leetcode.cn/problems/check-if-numbers-are-ascending-in-a-sentence/)|[Rust](src/easy/leetcode_very_easy.rs)|
|2057|[Smallest Index With Equal Value](https://leetcode.com/problems/smallest-index-with-equal-value/)|[Rust](src/easy/leetcode_very_easy.rs)|
|2114|[Maximum Number of...](https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/)|[Rust](src/easy/leetcode_very_easy.rs)|
|2169|[Count Operations to Obtain Zero](https://leetcode.com/problems/count-operations-to-obtain-zero)|[Rust](src/easy/leetcode_very_easy.rs)|

---

剑指Offer(lcof) && LCP(Leetcode Competition Programing)
========

| # | Title | Solutions | Category |
|---| ----- | -------- | ---------- |
|LCP.7|[传递信息](https://leetcode-cn.com/problems/chuan-di-xin-xi/)[Rust](src/bfs_dfs_backtracking/num_ways.rs)|
|59|[队列的最大值](https://leetcode-cn.com/problems/dui-lie-de-zui-da-zhi-lcof/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/max_queue.py)|
|60|[n个骰子的点数](https://leetcode-cn.com/problems/nge-tou-zi-de-dian-shu-lcof/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/dices_sum.py)|
|61|[扑克牌中的顺子](https://leetcode-cn.com/problems/bu-ke-pai-zhong-de-shun-zi-lcof/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/straight_in_playing_cards.py)|
|62|[圆圈中最后剩下的数字](https://leetcode-cn.com/problems/yuan-quan-zhong-zui-hou-sheng-xia-de-shu-zi-lcof/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/josephus.py)|约瑟夫环|

---

| # | Title | Solutions | Category |
|---| ----- | --------- | -------- |
|1|[A + B Problem](https://lintcode.com/problem/1)|leetcode_372|
|3|[Digit Counts](https://lintcode.com/problem/3)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/very_easy.py)|
|5|[Kth Largest Element](https://lintcode.com/problem/5)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)|quick_select, heap|
|20|[Dices Sum](https://lintcode.com/problem/dices-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/dices_sum.py)|
|31|[Partition Array](https://lintcode.com/problem/partition-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/partition_array.py)|two_pointers|
|40|[Implement Queue by Two Stacks](https://lintcode.com/problem/implement-queue-by-two-stacks/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_queue_by_stack.py)||
|52|[Next Permutation](https://lintcode.com/problem/next-permutation/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/next_permutation.py)||
|76|[Longest Increasing Subsequence](https://lintcode.com/problem/longest-increasing-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/longest_increase_subsequence.py)|接龙型动态规划|
|77|[Longest Common Subsequence](https://lintcode.com/problem/longest-common-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/longest_common_subsequence.py)||
|79|[Longest Common Substring](https://lintcode.com/problem/longest-common-substring/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/longest_common_substring.py)||
|80|[Median](https://lintcode.com/problem/median/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)|quick_select|
|86|[Binary Search Tree Iterator](https://lintcode.com/problem/binary-search-tree-iterator/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_in_order_iterator.py)||
|88|[Lowest Common Ancestor of a Binary Tree](https://lintcode.com/problem/lowest-common-ancestor-of-a-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)|divide_and_conquer|
|90|[k Sum II](https://lintcode.com/problem/k-sum-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/k_sum_2.py)||
|92|[Backpack](https://lintcode.com/problem/backpack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)|
|93|[Balanced Binary Tree](https://lintcode.com/problem/balanced-binary-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/depth_is_balance_binary_tree.py)|divide_and_conquer|
|95|[Validate Binary Search Tree](https://lintcode.com/problem/validate-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_valid.py)|
|105|[Copy List with Random Pointer](https://lintcode.com/problem/copy-list-with-random-pointer/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/clone_linked_list_with_random_ptr.py)||
|106|[Convert Sorted List to Binary Search Tree](https://lintcode.com/problem/convert-sorted-list-to-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/sorted_list_or_array_to_bst.py)|divide_and_conquer|
|107|[Word Break](https://lintcode.com/problem/word-break/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_word_break_coin_change.py)|完全背包问题|
|110|[Minimum Path Sum](https://lintcode.com/problem/minimum-path-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/grid_min_path_sum.py)||
|124|[Longest Consecutive Sequence](https://lintcode.com/problem/longest-consecutive-sequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/longest_consecutive_sequence.py)|并查集|
|125|[Backpack II](https://lintcode.com/problem/backpack-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)||
|127|[Topological Sorting](https://lintcode.com/problem/topological-sorting/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/topological_sorting.py)|BFS, topological_sorting|
|128|[Hash Function](https://lintcode.com/problem/hash-function/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/hash_function.py)||
|129|[Rehashing](https://lintcode.com/problem/rehashing/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/rehashing.py)||
|135|[Combination Sum](https://lintcode.com/problem/combination-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/combination_target_sum.py), [Rust](src/bfs_dfs_backtracking/combination_sum_1_2.rs)||
|137|[Clone Graph](https://lintcode.com/problem/clone-graphs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/clone_graph.py)|DFS, BFS|
|138|[Subarray Sum](https://lintcode.com/problem/subarray-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/unclassified/subarray_sum_zero.py)|
|143|[Sort Colors II](https://lintcode.com/problem/sort-colors-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/sort_colors_2.py)|quick_sort, counting_sort|
|144|[interleaving_positive_and_negative_numbers](https://lintcode.com/problem/sort-colors-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/interleaving_positive_and_negative_numbers.py)||
|147🔒|[Narcissistic Number](https://lintcode.com/problem/narcissistic-number)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/narcissistic_number.py)||
|148|[Sort Colors](https://lintcode.com/problem/sort-colors/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/sort_colors.py)|three_pointers, partition_array|
|153|[Combination Sum II](https://lintcode.com/problem/combination-sum-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/combination_target_sum.py), [Rust](src/bfs_dfs_backtracking/combination_sum_1_2.rs)||
|160|[Find Minimum in Rotated Sorted Array II](https://lintcode.com/problem/find-minimum-in-rotated-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/rotated_sorted_array_min_2_with_duplicate.py)|binary_search|
|161|[Rotate Image](https://lintcode.com/problem/rotate-image/)|[Rust](src/easy/grid_or_matrix/rotate_matrix.rs)|
|168|[Burst Balloons](https://lintcode.com/problem/burst-balloons/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/burst_ballon.py)||
|171|[Anagrams](https://lintcode.com/problem/anagrams/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/group_anagrams.py)||
|177|[Convert Sorted Array to Binary Search Tree With Minimal Height](https://lintcode.com/problem/convert-sorted-array-to-binary-search-tree-with-minimal-height/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/sorted_list_or_array_to_bst.py)|divide_and_conquer|
|183|[Wood Cut](https://lintcode.com/problem/wood-cut/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/wood_cut.py)|greedy|
|190|[Next Permutation II](https://lintcode.com/problem/next-permutation-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/next_permutation.py)|
|192|[Wildcard Matching](https://lintcode.com/problem/wildcard-matching/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/wildcard_matching.py)||
|197|[Permutation Index](https://lintcode.com/problem/permutation-index/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/permutation_index.py)||
|235|[Prime Factorization](https://lintcode.com/problem/prime-factorization/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/prime_factorization.py)|分解质因数|
|249|[Count of Smaller Number before itself](https://lintcode.com/problem/count-of-smaller-number-before-itself/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/segment_tree/count_of_smaller_number_before_itself.py)|sqrt_n|
|254|[Drop Eggs](https://lintcode.com/problem/drop-eggs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/drop_eggs.py)|sqrt_n|
|272🔒|[Climbing Stairs II](https://lintcode.com/problem/climbing-stairs-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/climb_stairs_2.py)|
|376|[Binary Tree Path Sum](https://lintcode.com/problem/binary-tree-path-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/root_to_leaf_paths_target_sum.py)|DFS, backtracking|
|380|[Intersection of Two Linked Lists](https://lintcode.com/problem/intersection-of-two-linked-lists/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/linked_list_cycle.py)||
|386|[Longest Substring with At Most K Distinct Characters](https://lintcode.com/problem/longest-substring-with-at-most-k-distinct-characters/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/longest_substr_with_longest_k_distinct_chars.py)||
|388|[Permutation Sequence](https://lintcode.com/problem/permutation-sequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/permutation.py)||
|393|[Best Time to Buy and Sell Stock IV](https://lintcode.com/problem/best-time-to-buy-and-sell-stock-iv/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)||
|397|[Longest Continuous Increasing Subsequence](https://lintcode.com/problem/longest-continuous-increasing-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/longest_continuous_increase_subsequence.py)||
|406|[Minimum Size Subarray Sum](https://lintcode.com/problem/minimum-size-subarray-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/minimun_size_subarray_sum.py)|sliding_window|
|415|[Valid Palindrome](https://lintcode.com/problem/valid-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome.py)|two_pointers|
|437|[Copy Books](https://lintcode.com/problem/copy-bookes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/copy_books.py)|dp, binary_search|
|440🔒|[Backpack III](https://lintcode.com/problem/backpack-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_3_item_can_select_multi.py)||
|443🔒|[Two Sum - Greater than target](https://lintcode.com/problem/two-sum-greater-than-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_le_count.py)|two_pointers|
|447🔒|[Search in a Big Sorted Array](https://lintcode.com/problem/search-in-a-big-sorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/binary_search_unknown_size_sorted_array.py)|binary_search_first, 倍增法|
|453|[Flatten Binary Tree to Linked List](https://lintcode.com/problem/flatten-binary-tree-to-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/in_place_flatten_binary_tree_to_linked_list.py)|
|461|[Kth Smallest Numbers in Unsorted Array](https://lintcode.com/problem/kth-smallest-numbers-in-unsorted-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)||
|464|[Sort Integers II](https://lintcode.com/problem/sort-integers-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/basic_sorting.py)|
|466|[Count Linked List Nodes](https://lintcode.com/problem/count-linked-list-nodes/)|too_easy|
|474🔒|[Lowest Common Ancestor II](https://lintcode.com/problem/lowest-common-ancestor-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)||
|476|[Stone Game(diff to leetcode)](https://lintcode.com/problem/stone-game/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stone_game_merge.py)||
|480|[Binary Tree Paths](https://lintcode.com/problem/binary-tree-paths/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/root_to_leaf_paths_find_all.py)|DFS, backtracking|
|486|[Merge K Sorted Arrays](https://lintcode.com/problem/merge-k-sorted-arrays/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_two_sorted_arrays.py)||
|492|[Implement Queue by Linked List](https://lintcode.com/problem/implement-queue-by-linked-list/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_queue_by_linked_list.py)||
|494|[Implement Stack by Two Queues](https://lintcode.com/problem/implement-stack-by-two-queues/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_stack_by_queue.py)||
|495|[Implement Stack](https://lintcode.com/problem/implement-stack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/impl_stack_by_queue.py)||
|512|[Decode Ways](https://lintcode.com/problem/decode-ways/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/decode_ways.py)||
|521🔒|[Remove Duplicate Numbers in Array](https://lintcode.com/problem/remove-duplicate-numbers-in-array/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/partition_array/remove_duplicate_numbers_in_array.py)|partition_array|
|533🔒|[Two Sum - Closest to target](https://lintcode.com/problem/two-sum-closest-to-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_unique_pairs.py)|two_sum|
|544|[Top k Largest Numbers](https://lintcode.com/problem/top-k-largest-numbers/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/top_k_largest_stream_min_heap.py)|min_heap|
|545|[Top k Largest Numbers II](https://lintcode.com/problem/top-k-largest-numbers-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/top_k_largest_stream_min_heap.py)|min_heap|
|547|[Intersection of Two Arrays](https://lintcode.com/problem/intersection-of-two-arrays/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/intersection_of_two_arrays.py)||
|548|[Intersection of Two Arrays II](https://lintcode.com/problem/intersection-of-two-arrays-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/intersection_of_two_arrays.py)||
|562|[Backpack IV](https://lintcode.com/problem/backpack-iv/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)|完全背包问题|
|563|[Backpack V](https://lintcode.com/problem/backpack-v/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_5_plans_count.py)||
|577|[Merge k Sorted Interval Lists](https://lintcode.com/problem/merge-k-sorted-interval-lists/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_two_sorted_interval_lists.py)||
|578|[Lowest Common Ancestor III](https://lintcode.com/problem/lowest-common-ancestor-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/binary_tree_lowest_common_ancestor.py)||
|584|[Drop Eggs II](https://lintcode.com/problem/drop-eggs-ii/)|[Rust](src/dp/drop_eggs.rs)|dp|
|587🔒|[Two Sum - Unique pairs](https://lintcode.com/problem/two-sum-unique-pairs/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_unique_pairs.py)|two_sum|
|588|[Partition Equal Subset Sum](https://lintcode.com/problem/partition-equal-subset-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)|0-1背包问题|
|594|[Implement strStr() II](https://lintcode.com/problem/strstr-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/string/find_substr.py)|kmp, Rabin-Karp(rolling_hash)|
|596🔒|[Minimum Subtree](https://lintcode.com/problem/maximum-subtree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/subtree_max_sum.py)|divide_and_conquer|
|600|[Smallest Rectangle Enclosing Black Pixels](https://lintcode.com/problem/smallest-rectangle-enclosing-black-pixels)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_search/smallest_rectangle_enclosing_black_pixels.py)||
|603|[Largest Divisible Subset](https://lintcode.com/problem/largest-divisible-subset/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/largest_divisible_subset.py)|
|606|[Kth Largest Element II](https://lintcode.com/problem/kth-largest-element-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/sorting_and_query/quick_select_kth_largest.py)|quick_select, quick_sort, heap|
|607|[Two Sum III - Data structure design](https://lintcode.com/problem/two-sum-iii-data-structure-design/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_3_impl.py)|two_pointers|
|608|[Two Sum II - Input array is sorted](https://lintcode.com/problem/two-sum-ii-input-array-is-sorted/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_2_input_is_sorted.py)|two_pointers|
|609|[Two Sum - Less than or equal to target](https://lintcode.com/problem/two-sum-less-than-or-equal-to-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_le_count.py)|two_pointers|
|610|[Two Sum - Difference equals to target](https://lintcode.com/problem/two-sum-difference-equals-to-target/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/two_sum_diff.py)|two_pointers|
|611🔒|[Knight Shortest Path](https://lintcode.com/problem/knight-shortest-path/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/knight_shortest_path.py)|bfs|
|615|[Course Schedule](https://lintcode.com/problem/course-schedule/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/course_schedule_2.py)|BFS, topological_sorting|
|616|[Course Schedule II](https://lintcode.com/problem/course-schedule-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/course_schedule_2.py)|BFS, topological_sorting|
|627|[Longest Palindromic Combination](https://lintcode.com/problem/longest-palindrome/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_combination.py)|greedy|
|628|[Maximum Subtree](https://lintcode.com/problem/maximum-subtree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/subtree_max_sum.py)|divide_and_conquer|
|630🔒|[Knight Shortest Path II](https://lintcode.com/problem/knight-shortest-path-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/knight_shortest_path_2.py)|bfs|
|633|[Find the Duplicate Number](https://lintcode.com/problem/find-the-duplicate-number/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/find_duplicate_number.py)|快慢双指针|
|657|[Insert Delete GetRandom O(1)](https://lintcode.com/problem/insert-delete-getrandom-o1/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/insert_delete_get_random_o1.py)||
|661|[Convert BST to Greater Tree](https://lintcode.com/problem/convert-bst-to-greater-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_to_gst.py)||
|667|[Longest Palindromic Subsequence](https://lintcode.com/problem/longest-palindromic-subsequence/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/longest_palindromic_subsequence.py)|dp(greedy)|
|683|[Word Break III](https://lintcode.com/problem/word-break-iii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/word_break_3.py)||
|685|[First Unique Number in Data Stream](https://lintcode.com/problem/685)|[Python](https://github.com/pymongo/python_leetcode/blob/master/linked_list/first_unique_number_in_stream.py)||
|691|[Recover Binary Search Tree](https://lintcode.com/problem/recover-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_recover_swap_two_nodes.py)||
|701|[Trim A Binary Search Tree](https://lintcode.com/problem/trim-a-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_trim_in_range.py)||
|702|[Russian Doll Envelopes](https://lintcode.com/problem/russian-doll-envelopes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/russian_doll_envelopes.py)||
|719|[Calculate Maximum Value](https://lintcode.com/problem/calculate-maximum-value/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/calculate_max_value_2.py)||
|724|[Minimum Partition](https://lintcode.com/problem/minimum-partition/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_2.py)|0-1背包问题|
|741|[Calculate Maximum Value II](https://lintcode.com/problem/calculate-maximum-value-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/calculate_max_value_2.py)|
|749|[John's backyard garden](https://lintcode.com/problem/johns-backyard-garden/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_4_coin_change_combination_sum_4.py)|
|793|[Intersection of Arrays](https://lintcode.com/problem/intersection-of-arrays/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/intersection_of_two_arrays.py)||
|802|[Soduku Solver](https://lintcode.com/problem/soduku-solver/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/soduku_solver.py)||
|816|[Traveling Salesman Problem](https://lintcode.com/problem/traveling-salesman-problem/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dfs_perm_comb/traveling_salesman_problem.py)||
|839|[Merge Two Sorted Interval Lists](https://lintcode.com/problem/merge-two-sorted-interval-lists/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/merge_two_sorted_interval_lists.py)||
|841|[String Replace](https://lintcode.com/problem/string-replace/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/trie/string_replace.py)||
|845|[Greatest Common Divisor](https://lintcode.com/problem/greatest-common-divisor/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/math/greatest_common_divisor.py)|
|859|[Max Stack](https://lintcode.com/problem/max_stack/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/max_stack.py)||
|871|[Minimum Factorization](https://lintcode.com/problem/minimum-factorization/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/factorization/minimum_factorization.py)|greedy|
|891|[Valid Palindrome II](https://lintcode.com/problem/valid-palindrome-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/palindrome/valid_palindrome_2.py)|two_pointers, greedy|
|892|[Alien Dictionary](https://lintcode.com/problem/alien-dictionary/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/graph/alien_dictionary.py)|heapq, topological_sorting|
|900|[Closest Binary Search Tree Value](https://lintcode.com/problem/closest-binary-search-tree-value/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value.py)||
|901|[Closest Binary Search Tree Value II](https://lintcode.com/problem/closest-binary-search-tree-value-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_closest_value_2.py)||
|902|[Kth Smallest Element in a BST](https://lintcode.com/problem/kth-smallest-element-in-a-bst/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/traversal_pre_order.py)|DFS, stack|
|954|[Insert Delete GetRandom O(1) - Duplicates allowed](https://lintcode.com/problem/insert-delete-getrandom-o1-duplicates-allowed/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/collections/insert_delete_get_random_o1.py)||
|995|[Best Time ... Stock with Cooldown](https://lintcode.com/problem/best-time-to-buy-and-sell-stock-with-cooldown/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/stock.py)|
|1147|[Work Plan](https://lintcode.com/problem/work-plan/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/house_robber.py)|
|1208|[Target Sum](https://lintcode.com/problem/target-sum/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/backpack_1_target_sum.py)|0-1背包问题|
|1276|[Sum of Two Integers](https://lintcode.com/problem/sum-of-two-integers/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bitwise/binary_addition.py)|binary_addition|
|1284|[Integer Break](https://lintcode.com/problem/integer-break/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/dp/integer_break.py)|划分类DP|
|1311|[Lowest Common Ancestor of a Binary Search Tree](https://lintcode.com/problem/lowest-common-ancestor-of-a-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/binary_tree/bst_lowest_common_ancestor.py)||
|1343|[Sum of Two Strings](https://lintcode.com/problem/sum-of-two-strings/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/easy/sum_of_to_strings.py)|
|1359|[Convert Sorted Array to Binary Search Tree](https://lintcode.com/problem/convert-sorted-array-to-binary-search-tree/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/divide_conquer/sorted_list_or_array_to_bst.py)|divide_and_conquer|
|1375|[Substring With At Least K Distinct Characters](https://lintcode.com/problem/substring-with-at-least-k-distinct-characters/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/substring_with_at_least_k_distinct_characters.py)||
|1479|[Can Reach The Endpoint](https://lintcode.com/problem/can-reach-the-endpoint/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/bfs/can_reach_the_end_point.py)|bfs|
|1499|[Reordered Power of 2](https://lintcode.com/problem/reordered-power-of-2/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/brain_twists/reordered_power_of_2.py)|permutation|
|1508|[Score After Flipping Matrix](https://lintcode.com/problem/score-after-flipping-matrix/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/greedy/score_after_flipping_matrix.py)|greedy|
|1790🔒|[Rotate String II](https://lintcode.com/problem/rotate-string-ii/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/rotate_reverse_circle_shift/rotate_string_2.py)|reverse, circle_shift|
|1870|[number of substrings with all zeroes](https://lintcode.com/problem/number-of-substrings-with-all-zeroes/)|[Python](https://github.com/pymongo/python_leetcode/blob/master/two_sum_two_pointers/number_of_substrings_with_all_zeroes.py)|

todo_dp:
- 划分型动态规划:戳气球、石子归并(dp/stone_game_merge.py)
- 选或不选类DP: 打家劫舍系列/股票买卖系列
- 计数型动态规划: k sum
- [ ] 43/415 字符串整数相乘/相加
- [ ] 415是经典面试题利用字符串模拟竖式加法进行大数相加
- [ ] 有兴趣但是困难级别的题: 1. 推箱子 2. 华容道
- [ ]最大直方图矩阵(Longest Histogram Rectangle), 单调栈O(n^3)->O(n)

## some awesome leetcode rust solutions repos
- https://github.com/warycat/rustgym
- https://gitee.com/ZIP97/leetcode/ by 苦瓜小仔
- https://stevenbai.top/rust-leetcode/

## other awesome leetcode repos I recommend
- https://github.com/halfrost/LeetCode-Go
- https://github.com/haoel/leetcode

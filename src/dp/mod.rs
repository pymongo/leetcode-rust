/*!
TODO 动态规划能解决 求最值、求可行性、求方案总数 的问题
TODO 动态规划的题型:
[x] 坐标/矩阵型: Unique Path, Triangle
[ ] 字符串1前缀型(前i个字符划分成j段的最值/方案数/可行性): Word Break I/III
[x] 字符串3输入两个字符串匹配型(): Longest Common Subsequence, Wildcard Matching
[ ] 背包型(i表示前i个物品, j表示价值之和是否可行):
[ ] 01背包问题: 每个物品只能选或不选，不能选多份
[ ] 区间型子数组/子序列/子串(i-j表示区间i-j的子数组/子串): Stone Game, Burst Balloons, 最长回文子串/子序列、两个字符串最长公共部分
[ ] 状态压缩型: TSP、Unique Path III的非DFS解法
[ ] 打劫类的题: 打家劫舍(今天打劫哪一家本周的所有打劫才能最赚钱 )
TODO 动态规划的实现方法:
[ ] 记忆化搜索(DFS回溯): Unique Path III的简单的DFS解法
[x] 递推迭代: Triangle

## 动态规划知识
动态规划的递归依赖不能有「循环依赖」
记忆化搜索只是动态规划的一种实现方式
*/
mod dp_easy;
mod drop_eggs;
mod fibonacci;
mod freedom_trail;
mod jump_game_2;
mod longest_common_substr;
mod longest_palindromic_substr;
mod trapping_rain_water;
mod unique_paths;
mod triangle;

/** https://leetcode.com/problems/dungeon-game/
有点像triangle一题自下而上这样"逆向思维"的DP解法
这题不能像unique_path那样左上到右下去遍历，这样的话要记录两个状态，一个是从起点到(i,j)的最小初始值，另一个是最大路径和
希望初始值尽可能小的同时最大路径和又尽可能大，两个状态取最值很难进行dp
而且每次行走的方向都是有状态的，必须保证行走的过程中生命值始终>=1
*/
fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (dungeon.len(), dungeon[0].len());
    // 为了处理边界情况: 右边扩宽一列，下方扩宽一行
    // dp的默认值是i32::MAX，这样最底下一行和最右侧列抉择时不会选上
    let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];
    // 终点的右侧和下侧的生命值为1
    dp[m - 1][n] = 1;
    dp[m][n - 1] = 1;
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            // dp[i+1][j].min(dp[i][j+1])表示当前位置选择往右或往下走能更节约生命值的选择
            // 我也想到这里逆向思维要用减，但是没想到边界情况的处理和至少要dp[i][j]保留1点生命值不能是负数生命值吃了回血后诈尸
            dp[i][j] = (dp[i + 1][j].min(dp[i][j + 1]) - dungeon[i][j]).max(1);
        }
    }
    dp[0][0]
}

#[test]
fn test_calculate_minimum_hp() {
    let test_cases = vec![(vec_vec![[-2, -3, 3], [-5, -10, 1], [10, 30, -5],], 7)];
    for (input, output) in test_cases.into_iter() {
        assert_eq!(calculate_minimum_hp(input), output);
    }
}

/** https://leetcode.com/problems/unique-paths/
这题虽然以我今天的水平来看非常简单，但是由于除了DP外还有组合数解法而且还是经典题，值得单独归档成一个文件而非纳入dp_easy中

## DP解法
本题类似70题(爬楼梯)，爬到第N格梯子的路径=f(n-1)+f(n-2)
由于从左下角(0,0)走到右上角(m,n)的最短路径只能往上或往右走
所以得到递推式(状态转移方程):
f(m,n)=f(m-1,n)+f(m,n-1)

### DP遍历方向
初始条件：第一列和最底下的一行全为1
遍历方向：从底下往上数第二行的第2个元素开始往右扫，每行是从左往右，整体是从下往上

## 组合数解法
从(0,0)走到(m,n)总共需要走m+n步，
要在m+n中选m次往右走，或者选n次往左走
极致地简化了问题，将问题抽象成组合数问题
组合问题(要从班上m+n个同学中选m个做值日，或者选n个不做值日)
知识补充：排列问题(从班上m+n个同学中列出田径100米赛跑前3名的可能情况)

> return math.comb(m-1 + n-1, n-1) if m > n else math.comb(n-1 + m-1, m-1)

```go
func UniquePaths(m int, n int) int {
    // 端点是4x4，但是棋盘的格子就3x3
    max, min := m-1, n-1
    if min > max {
        max, min = min, max
    }
    result := 1
    sum := max+min
    for i:=0; i<min; i++ {
        result *= sum-i
        // 先乘后除，避免result溢出
        result /= i+1
    }
    return result
}
```

### 组合数解法int溢出问题
在Go语言中，遍历计算组合数时先乘后除不会溢出，而在Rust中即便用了u32类型也会在(52,9)这个测试用例上溢出
我在本题采用了u64类型，也算避免了溢出
关于能否整除的问题，我试了下，如果分子的阶乘是从大到小，分母的阶乘是从小到大，大概率不会溢出

### 端点与格子的问题
由于输入的m、n表示的是端点数(联想围棋的点)
但是走路的方式却是在网格上走，所以实际的m和n应该是m-1和n-1

## 参看
[走格子/棋盘问题 有多少条路径可走](https://blog.csdn.net/yusiguyuan/article/details/12875415)
*/
fn unique_paths(m: i32, n: i32) -> i32 {
    let m: u64 = (m - 1) as u64;
    let mut n: u64 = (n - 1) as u64;
    let mut res: u64 = 1;
    let sum: u64 = m + n;
    if n > m {
        n = m;
    }
    // Example: 组合数C(5,2)=5*4/1*2
    for i in 0..n {
        res *= sum - i;
        res /= i + 1;
    }
    res as i32
}

#[test]
fn test_unique_paths() {
    const TEST_CASES: [(i32, i32, i32); 2] = [(23, 12, 193_536_720), (51, 9, 1_916_797_311)];
    for (m, n, expected) in TEST_CASES {
        assert_eq!(unique_paths(m, n), expected);
    }
}

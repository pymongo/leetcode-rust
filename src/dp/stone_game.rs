//! https://leetcode.com/problems/stone-game/
fn stone_game_wrong_answer(nums: Vec<i32>) -> bool {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut total = 0;
    let mut is_player_1_turn = true;
    while left < right {
        let (left_val, right_val) = (nums[left], nums[right]);
        let score = match left_val.cmp(&right_val) {
            std::cmp::Ordering::Less => {
                right -= 1;
                right_val
            }
            // [3,7,2,3], player_1 need to remove right's 3 to win
            std::cmp::Ordering::Equal => {
                let (mut left_next, mut right_next) = (left + 1, right - 1);
                // if equal default use left
                let mut score = left_val;
                left += 1;
                while left_next < right_next {
                    match nums[left_next].cmp(&nums[right_next]) {
                        std::cmp::Ordering::Less => {
                            break;
                        }
                        std::cmp::Ordering::Equal => {
                            left_next += 1;
                            right_next += 1;
                        }
                        std::cmp::Ordering::Greater => {
                            left -= 1;
                            right -= 1;
                            score = right_val;
                            break;
                        }
                    }
                }
                score
            }
            std::cmp::Ordering::Greater => {
                left += 1;
                left_val
            }
        };
        if is_player_1_turn {
            total += score;
        } else {
            total -= score;
        }
        is_player_1_turn = !is_player_1_turn;
    }
    total > 0
}

struct DpHelper {
    nums: Vec<i32>,
}

impl DpHelper {}

fn dp_time_limit_exceeded(
    left: usize,
    right: usize,
    palyer1_score: i32,
    is_player1_turn: bool,
    nums: &[i32],
) -> bool {
    if left >= right {
        return palyer1_score > 0;
    }
    if is_player1_turn {
        dp_time_limit_exceeded(
            left + 1,
            right,
            palyer1_score + nums[left],
            !is_player1_turn,
            nums,
        ) || dp_time_limit_exceeded(
            left,
            right - 1,
            palyer1_score + nums[right],
            !is_player1_turn,
            nums,
        )
    } else {
        dp_time_limit_exceeded(
            left + 1,
            right,
            palyer1_score - nums[left],
            !is_player1_turn,
            nums,
        ) || dp_time_limit_exceeded(
            left,
            right - 1,
            palyer1_score - nums[right],
            !is_player1_turn,
            nums,
        )
    }
}

fn stone_game_dp_time_limit_exceeded(nums: Vec<i32>) -> bool {
    dp_time_limit_exceeded(0, nums.len() - 1, 0, true, &nums)
}

/// time_limit_exceeded
fn dp(left: usize, right: usize, palyer1_score: i32, target_score: i32, nums: &[i32]) -> bool {
    if palyer1_score >= target_score {
        return true;
    }
    if left >= right {
        return false;
    }
    // player1 select left, player2 select left+1 or right
    dp(left+2, right, palyer1_score+nums[left], target_score, nums) ||
    dp(left+1, right-1, palyer1_score+nums[left], target_score, nums) ||
    // player1 select right, player2 select right+1 or left
    dp(left, right-2, palyer1_score+nums[right], target_score, nums) ||
    dp(left+1, right-1, palyer1_score+nums[right], target_score, nums)
}

fn stone_game_dp(nums: Vec<i32>) -> bool {
    dp(0, nums.len() - 1, 0, nums.iter().sum::<i32>() / 2, &nums)
}

/**
dp[l][r]表示石子剩余nums[l..=r]时，先手方能取得的最大收益
取左端的nums[l]时，先手收益nums[l]，此时后手方变先手，后手最大收益dp[l+1][r]，分差是 nums[l]-dp[l+1][r]
同理去右端也类似，所以 dp[l][r] = max(nums[l]-dp[l+1][r], nums[r]-dp[l][r-1])

填表顺序: 从下到上，从左到右

0 1 2 3 (l)
1 x 2 3
2   x 1
3     x
(r)
*/
#[allow(clippy::manual_memcpy)]
fn stone_game(nums: Vec<i32>) -> bool {
    let len = nums.len();
    let mut dp = vec![vec![0; len]; len];
    // 剩余区间大小为1时，最大收益就只能是nums[i]
    for i in 0..len {
        dp[i][i] = nums[i];
    }
    for l in (0..len - 1).rev() {
        for r in l + 1..len {
            dp[l][r] = std::cmp::max(nums[l] - dp[l + 1][r], nums[r] - dp[l][r - 1]);
        }
    }
    dp[0][len - 1] > 0
}

#[test]
fn test_stone_game() {
    const TEST_CASES: [(&[i32], bool); 4] = [
        (&[6, 7, 5, 10, 8, 9, 8, 9, 1, 2], true),
        (&[5, 3, 4, 5], true),
        (&[3, 7, 2, 3], true),
        (&[3, 2, 10, 4], true),
    ];
    for (nums, is_left_win) in TEST_CASES {
        //assert_eq!(stone_game_wrong_answer(nums.to_vec()), is_left_win);
        assert_eq!(
            stone_game_dp_time_limit_exceeded(nums.to_vec()),
            is_left_win
        );
        assert_eq!(stone_game_dp(nums.to_vec()), is_left_win);
        assert_eq!(stone_game(nums.to_vec()), is_left_win);
    }
}

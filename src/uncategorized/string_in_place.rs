/// https://leetcode.com/problems/replace-all-s-to-avoid-consecutive-repeating-characters/
fn replace_question_mark_to_avoid_consecutive_repeating_char(s: String) -> String {
    let mut s = s.into_bytes();
    let n = s.len();
    for i in 0..n {
        if s[i] != b'?' {
            continue;
        }
        for letter in b'a'..=b'z' {
            if let Some(left) = s.get(i.wrapping_sub(1)) {
                if letter.eq(left) {
                    continue;
                }
            }
            if let Some(right) = s.get(i + 1) {
                if letter.eq(right) {
                    continue;
                }
            }
            s[i] = letter;
        }
    }
    unsafe { String::from_utf8_unchecked(s) }
}

/// https://leetcode.com/problems/shuffle-string/
/// 能不能用In-Place的swap操作完成重排？我联想到rotate_string那题教室换座位的情况，A的新座位在B，A挤到B的slot，把B挤出教室，然后B再去挤自己的新座位...
fn shuffle_string_normal_solution(s: String, indices: Vec<i32>) -> String {
    let mut ret = vec![0u8; s.len()];
    for (i, byte) in indices.into_iter().zip(s.into_bytes().into_iter()) {
        ret[i as usize] = byte;
    }
    unsafe { String::from_utf8_unchecked(ret) }
}

/**
codeleet => lodeceet 换来的l之前下标是4，正好是indices[0]指示的正确下标值，所以这轮结束
*/
fn shuffle_string_classroom_swap_seats(s: String, indices: Vec<i32>) -> String {
    let mut s = s.into_bytes();
    let mut indices: Vec<usize> = indices.into_iter().map(|x| x as usize).collect();
    let len = s.len();
    for i in 0..len {
        // 当前遍历到的数组下标i不在正确的位置上
        if indices[i] != i {
            // 放到了错误位置的字符ch(unallocated_char)
            let mut ch = s[i];
            // ch的正确位置
            let mut ch_correct_idx = indices[i];
            while ch_correct_idx != i {
                std::mem::swap(&mut s[ch_correct_idx], &mut ch);
                // ch被放到正确位置后，将indices[ch_correct_idx]标记成已修正(indices[i]=i时表示s[i]已修正)
                // 迭代: ch=s[ch_correct_idx](相当于换座位场景A的新座位是B，A把B的桌子移到走廊，此时待分配的ch就变成B)
                //      ch_correct_idx = indices[indices[ch_correct_idx]]
                std::mem::swap(&mut indices[ch_correct_idx], &mut ch_correct_idx);
            }
            // 此时的ch的正确位置是i
            s[i] = ch;
            // 将下标i标记成已修正
            indices[i] = i;
        }
    }
    unsafe { String::from_utf8_unchecked(s) }
}

#[test]
fn test_shuffle_string() {
    const TEST_CASES: [(&str, &[i32], &str); 1] =
        [("codeleet", &[4, 5, 6, 7, 0, 2, 1, 3], "leetcode")];
    for &(s, indices, expected) in TEST_CASES.iter() {
        assert_eq!(
            shuffle_string_classroom_swap_seats(s.to_string(), indices.to_vec()),
            expected
        );
    }
}

/// https://leetcode.com/problems/reverse-words-in-a-string/
/// 先反转整个句子再反转每个单词的字母=反转句子中的单词，非In-Place解法是split后reverse再join或者利用insert(0)可以"逆序"的特点，用deque队列辅助完成单词反转操作
/// s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
fn reverse_words_in_a_sentence_in_place(s: String) -> String {
    let mut s: Vec<u8> = s.into_bytes();
    s.reverse();
    let len = s.len();
    let mut free_space_idx = 0;

    let mut word_start = 0;
    while word_start < len {
        if s[word_start] == b' ' {
            word_start += 1;
            continue;
        }

        // 操作前: "AliceilA  sevoL    boB  "
        // 操作后: "Alice lA  sevoL    boB  "
        if free_space_idx > 0 {
            s[free_space_idx] = b' ';
            free_space_idx += 1;
        }
        // println!("{:?}", unsafe { String::from_utf8_unchecked(s.clone()) });

        // 操作前: "   ecilA  sevoL    boB  "
        // 操作后: "ecilAilA  sevoL    boB  "
        let mut word_end = word_start;
        while word_end < len && s[word_end] != b' ' {
            // 先把单词整体往左边挪动(去掉单词间可能超过1个空格的情况)
            s[free_space_idx] = s[word_end];
            free_space_idx += 1;
            word_end += 1;
        }
        // println!("{:?}", unsafe { String::from_utf8_unchecked(s.clone()) });

        // 操作前: "ecilAilA  sevoL    boB  "
        // 操作后: "AliceilA  sevoL    boB  "
        s[free_space_idx - (word_end - word_start)..free_space_idx].reverse();

        word_start = word_end;

        word_start += 1;
    }
    // remain 0..free_space_idx
    s.truncate(free_space_idx); // C++ erase
    unsafe { String::from_utf8_unchecked(s) }
}

#[test]
fn test_reverse_words_in_a_sentence() {
    const TEST_CASES: [(&str, &str); 1] = [("  Bob    Loves  Alice   ", "Alice Loves Bob")];
    for &(input, output) in TEST_CASES.iter() {
        assert_eq!(
            reverse_words_in_a_sentence_in_place(input.to_string()),
            output
        );
    }
}

/// https://leetcode.com/problems/reverse-string/
fn reverse_string(s: &mut Vec<char>) {
    s.reverse();
}

/** https://leetcode.com/problems/rotate-array/
Input:  nums = [1,2,3,4,5,6,7], k = 3
Output: [5,6,7,1,2,3,4]
Explanation: nums数组向右循环移位 3位

原地换位(In place)算法主要有两种:
1. 环状替换
   参考学生时代换座位时，A同学要换到D同学的位置上，先把D同学移出教室(temp)，再将A同学移到D同学的位置上
   以此类推，重复上述过程N次。向右移位k次后的`new_index=(old_index+k)%len`
   参考: https://leetcode-cn.com/problems/rotate-array/solution/xuan-zhuan-shu-zu-yuan-di-huan-wei-xiang-xi-tu-jie/
   教室换座位的解法在shuffle-string一题中有演示，这里就不重复了
2. 先整体反转，再举办反转
   借助反转的数学原理，向右移k位等于整个数组reverse后，nums[0:k]和nums[k+1:len]再reverse一次
*/
fn rotate_array_solution(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    nums.rotate_left(k as usize % len);
}

/// 三次反转的解法，先整体反转，再反转0..k和k..len
fn rotate_array_reverse_solution(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let k = k as usize % len;
    nums.reverse();
    nums[0..k].reverse();
    nums[k..len].reverse();
}

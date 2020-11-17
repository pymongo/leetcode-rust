//! https://leetcode.com/problems/partition-labels/
fn partition_labels(s: String) -> Vec<i32> {
    const NOT_FOUND: usize = 501;
    let s = s.into_bytes();
    // s.len in range [1,500]
    let mut last_occur = vec![0usize; b'z' as usize + 1];
    for (i, byte) in s.iter().enumerate() {
        last_occur[*byte as usize] = i;
    }
    let mut res = Vec::new();
    let mut cur_chunk_start = 0;
    // 已扫描的字符能去到的最远位置
    let mut cur_chunk_end = 0;
    for (i, byte) in s.iter().enumerate() {
        cur_chunk_end = cur_chunk_end.max(last_occur[*byte as usize]);
        // 当前char不能纳入cur_chunk，因为与前面的某个char重复了，切一刀后当前char作为新的chunk的起点
        if i == cur_chunk_end {
            res.push((cur_chunk_end - cur_chunk_start + 1) as i32);
            cur_chunk_start = cur_chunk_end + 1;
        }
    }
    res
}

#[test]
fn test_partition_labels() {
    const TEST_CASES: [(&str, &[i32]); 1] = [("ababcbacadefegdehijhklij", &[9, 7, 8])];
    for &(s, expected) in TEST_CASES.iter() {
        assert_eq!(partition_labels(s.to_string()), expected.to_vec());
    }
}

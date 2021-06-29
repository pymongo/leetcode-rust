/*! https://leetcode.com/problems/excel-sheet-column-title/
26进制，例如excel的第26列，要输出成AB
要注意26进制是从[1,26]而不是[0,25]

## Some Math
remainder∈[0,25]
(1) num=quotient*26+remainder
(2) remainder=num-quotient*26

but we wanted_remainder∈[1,26]
(3) num=quotient_2*26+wanted_remainder
(4) (num-1)=quotient_2*26+(wanted_remainder-1)
(5) wanted_remainder-1=(num-1)%26
(6) wanted_remainder=(num-1)%26+1
So quotient_2 = (num-wanted_remainder)/26
*/
fn excel_sheet_column_title(mut column_number: i32) -> String {
    const POSITION_NATION: i32 = 26;
    let mut s = vec![];
    while column_number != 0 {
        // 例如26要显示成Z而非`10`
        let rem = (column_number - 1) % POSITION_NATION + 1;
        s.push(b'A' + rem as u8 - 1);
        column_number = (column_number - rem) / POSITION_NATION;
    }
    s.reverse();
    unsafe { String::from_utf8_unchecked(s) }
}

/// https://leetcode.com/problems/excel-sheet-column-number/
fn excel_sheet_column_number(column_title: String) -> i32 {
    let mut num = 0;
    for each in column_title.into_bytes() {
        let rem = each - b'A' + 1;
        num = num * 26 + i32::from(rem);
    }
    num
}

#[test]
fn test_excel_sheet_column_title() {
    const TEST_CASES: [(i32, &str); 3] = [(701, "ZY"), (1, "A"), (28, "AB")];
    for (column_number, column_title) in TEST_CASES {
        assert_eq!(excel_sheet_column_title(column_number), column_title);
        assert_eq!(
            excel_sheet_column_number(column_title.to_string()),
            column_number
        );
    }
}

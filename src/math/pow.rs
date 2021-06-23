/// https://leetcode.com/problems/powx-n/
fn my_pow(base: f64, mut exponent: i32) -> f64 {
    if exponent == i32::MIN {
        if base > 1.0 {
            return 0.0;
        }
        return 1.0;
    }
    let exponent_is_negative = if exponent < 0 {
        exponent = -exponent;
        true
    } else {
        false
    };

    let mut ret = 1.0;
    let mut mask = base;
    while exponent != 0 {
        // 3^5=3^(2^2+2^0)=(3^2)^2+3，所以只有3和3^(2*2)对结果有贡献
        if exponent % 2 == 1 {
            ret *= mask;
        }
        mask *= mask;
        exponent /= 2;
    }
    if exponent_is_negative {
        ret = 1.0 / ret;
    }
    ret
}

#[test]
fn test_my_pow() {
    #[allow(clippy::decimal_literal_representation)]
    const TEST_CASES: [(f64, i32, f64); 3] = [
        (2.10, 3, 9.261),
        (2.0, -2, 0.25),
        (2.0, -2_147_483_648, 0.0),
    ];
    for (x, n, pow_output) in TEST_CASES {
        assert!((my_pow(x, n) - pow_output).abs() < 10e-6);
    }
}

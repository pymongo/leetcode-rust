/**
一个动态链接库(dynamic linking library, 也叫so文件)可以对应一个或多个头文件，例如boost库
例如 libc.so 就有 time.h, stdlib.h 等多个头文件
TODO 为什么 openssl库 既有libcrypto.so也有libssl.so
可以通过`nm`命令工具获知某个so文件内有没有某个函数
```text
[w@w-manjaro ~]$ nm -D /usr/lib/libc.so.6 | grep gmtime
00000000000bb480 T gmtime@@GLIBC_2.2.5
00000000000bb460 T __gmtime_r@@GLIBC_2.2.5
00000000000bb460 W gmtime_r@@GLIBC_2.2.5
```

注意C语言的rand并不覆盖i32的范围，随即数的生成范围是[0, RAND_MAX]
而 RAND_MAX = i32::MAX
*/
pub fn random_i32() -> i32 {
    #[allow(non_camel_case_types)]
    type time_t = i64; // 根据libc源码中time_t的类型定义
    #[link(name = "c", kind = "dylib")]
    extern "C" {
        /// https://en.cppreference.com/w/cpp/chrono/c/time
        fn time(arg: *mut time_t) -> time_t;
        fn rand() -> i32;
        fn srand(seed: u32);
    }

    use std::sync::Once;
    static INIT_RAND_SEED: Once = Once::new();
    INIT_RAND_SEED.call_once(|| unsafe {
        let mut current_timestamp: time_t = std::mem::zeroed();
        time(&mut current_timestamp as *mut time_t);
        srand(current_timestamp as u32);
    });

    unsafe { rand() }
}

pub fn rand_range(min: i32, max: i32) -> i32 {
    const RAND_MAX: i32 = 0x7fffffff;
    let random_num = random_i32();
    // 更精准点的随机数范围生成过程: min + random_num / (RAND_MAX / (max - min + 1) + 1)
    // rand() % 7的范围在[0,6]，加上offset 1正好是[1,7]
    // 一般只记忆这个简单的 用MOD生成一定范围内的随机数
    random_num % max + min
}

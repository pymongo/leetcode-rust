#![feature(test)]
extern crate test;

const NUMS: [i32; 1000] = [
    986, 280, 908, 24, 458, 741, 378, 682, 443, 878, 252, 255, 82, 557, 317, 921, 929, 484, 65,
    414, 416, 921, 221, 607, 888, 969, 712, 718, 528, 678, 778, 372, 143, 406, 94, 990, 122, 33,
    901, 139, 23, 483, 201, 169, 608, 670, 832, 725, 126, 223, 836, 76, 372, 990, 159, 96, 183,
    281, 557, 974, 2, 745, 234, 979, 478, 536, 517, 567, 535, 640, 178, 892, 415, 880, 286, 931,
    673, 675, 188, 37, 194, 725, 308, 232, 600, 78, 238, 59, 784, 346, 263, 921, 829, 11, 345, 343,
    419, 559, 145, 431, 906, 151, 113, 753, 115, 157, 866, 705, 362, 127, 309, 486, 192, 938, 312,
    370, 529, 505, 653, 469, 347, 553, 332, 902, 452, 591, 700, 646, 493, 796, 778, 166, 785, 599,
    128, 513, 855, 523, 233, 367, 109, 811, 685, 452, 829, 913, 331, 810, 886, 219, 968, 941, 817,
    85, 70, 210, 766, 739, 483, 719, 294, 346, 258, 870, 680, 808, 630, 394, 591, 410, 768, 513,
    218, 298, 532, 18, 565, 25, 438, 708, 450, 782, 212, 313, 408, 914, 595, 876, 86, 553, 637,
    763, 989, 25, 780, 171, 989, 206, 800, 991, 507, 239, 562, 430, 309, 356, 664, 762, 712, 36,
    330, 967, 644, 293, 544, 460, 449, 825, 872, 624, 571, 464, 232, 863, 76, 335, 239, 503, 650,
    917, 769, 252, 212, 744, 724, 117, 991, 729, 867, 477, 186, 596, 113, 358, 526, 22, 158, 723,
    462, 383, 224, 934, 918, 701, 784, 814, 76, 278, 395, 82, 892, 606, 865, 15, 649, 0, 949, 650,
    298, 274, 812, 578, 858, 937, 265, 999, 973, 66, 786, 379, 751, 254, 195, 294, -4, 372, 512,
    677, 996, 750, 75, 712, 346, 827, -1, 706, 762, 115, 596, 488, 722, 650, 208, 994, 466, 316,
    524, 968, 36, 652, 586, 238, 238, 443, 928, 597, 251, 203, 369, 936, 533, 302, 851, 744, 898,
    333, 442, 572, 736, 208, 598, 613, 926, 383, 581, 900, 87, 719, 772, 829, 84, 99, 845, 889,
    855, 571, 313, 344, 446, 610, 442, 95, 816, 509, 276, 495, 450, 481, 333, 832, 929, 949, 592,
    733, 858, 607, 453, 332, 199, 856, 686, 460, 97, 117, 549, 253, 815, -2, 324, 249, 96, 548,
    317, 120, 18, 477, 50, 281, 819, 627, 628, 896, 935, 507, 332, 898, 371, 255, 243, 89, 901,
    889, 946, 745, 875, 678, 782, 31, 518, 544, 3, 451, -9, 536, 677, 711, 838, 57, 977, 850, 144,
    740, 949, 340, 854, 91, 909, 570, 466, 983, 711, 780, 469, 499, 879, 776, 805, 495, 857, 519,
    663, 317, 515, 721, 438, 120, 470, 122, 453, 579, 337, 699, 693, 313, 806, 250, 1000, 342, 954,
    169, 354, 507, 618, 328, 8, 912, 125, -7, 793, 948, 786, 173, 204, 351, 41, 695, 140, 75, 46,
    710, 461, 856, 167, 304, 732, 531, 838, 48, 662, 955, 743, 64, 127, 661, 567, 312, 593, 106,
    839, 316, 773, 266, 705, 496, 1000, 389, 517, 88, 430, 482, 246, 924, 50, 966, 971, 632, 746,
    48, 13, 440, 27, 517, 620, 715, 984, 300, 408, 520, 459, 880, 293, 93, 996, 520, 532, 498, 737,
    849, 833, 226, 454, 33, 40, 524, 11, 830, 724, 304, 19, 1000, 651, 47, 668, 166, 828, 766, 494,
    846, 585, 231, 502, 925, 316, 241, 94, 350, 201, 747, 881, 845, 135, 341, 129, 677, 612, 113,
    402, 578, 731, 745, 166, 12, 366, 917, 125, 568, 997, 575, 948, 494, 29, 461, 206, 38, 649,
    877, 46, 225, 615, 716, 215, 505, 322, 591, 211, 219, 564, 1, 453, 896, 923, 385, 394, 561,
    384, 329, 322, 723, 414, 265, 779, 81, 577, -8, 688, 964, 154, 692, 563, -10, 351, 823, 826,
    698, 540, 566, 505, 383, 221, 624, 558, 960, 406, 799, 275, 396, 381, 60, 674, 650, 488, 685,
    843, 865, 24, 138, 118, 81, 322, 678, 307, 370, 77, 288, 11, 445, 335, 791, 441, 87, 12, 593,
    379, 476, 505, 665, 532, 569, 165, 949, 45, 828, 335, 743, 732, 936, 242, 576, 967, 306, 311,
    245, 900, 602, 613, 234, 430, 843, 405, 579, 900, 32, 566, 796, 851, 897, 654, 584, 331, 250,
    367, 779, 320, 894, 781, 221, 910, 356, -3, 807, 173, 590, 22, 849, 710, 276, 157, 306, 151,
    486, 144, 438, 570, 709, 687, 617, 527, -2, 351, 93, 703, 220, 919, 679, 35, 232, 162, 714,
    815, 257, 972, 724, 589, 694, 758, 236, 856, 611, 732, 834, 294, 799, 864, 785, 684, 318, 323,
    692, 181, 320, 157, 733, 822, 820, 73, 583, 28, 577, 725, 712, 784, 337, 567, 216, 436, 462,
    279, 389, 1, 196, 458, 191, 361, 486, 469, 934, 606, 908, 819, 501, 692, 123, 995, 818, 507,
    230, 694, 345, 16, 177, 272, 114, 226, 182, 721, 562, 523, 422, 603, 92, 101, 525, 342, 951,
    898, 721, 19, 82, 539, 110, 262, 297, 327, 312, 377, 461, 726, 504, -7, 939, 397, 974, 486,
    167, 468, 95, 216, 128, 685, 405, 498, 831, 610, 268, 337, 470, 391, 661, 968, 217, 899, 810,
    699, 433, 431, 183, 140, 227, 945, 945, 736, 604, 583, 705, 192, 678, 662, 242, 734, 364, 740,
    384, 65, 243, 322, 80, 778, 587, 65, 880, 52, 985, 866, 241, 388, 160, 806, 742, 600, 471, 757,
    377, 531, 737, -9, 994, 812, 672, 892, 574, 99, 821, 704, 801, 557, 944, 720, 409, 456, 514,
    429, 713, 57, 301, 949, 489, 80, 860, 92, 161, 176, 498, 482, 875, 839, 229, 594, 931, 928,
    350, 174, 595, 511, 81, 907, 32, 541, 399, 92, 672, 8, 608, 826, 176, 461, 964, 550, 390, 745,
    759, 84, 13, 648, 258, 343, 162, 223, 442, 592, 615, 933, 760, 509, 57, 632, 257, 0, 581, 68,
    919, 472, 430, 892, 739, 778, 106, 105, 232, 708, 125, 664, 759, 6, 186, 670, 130, 329, 859,
    13, 219,
];

const LEN: usize = 1000;

#[bench]
fn bench_stdlib_merge_sort(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut nums = NUMS.to_vec();
        nums.sort();
    });
}

#[bench]
fn bench_stdlib_quick_sort(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut nums = NUMS.to_vec();
        nums.sort_unstable();
    });
}

fn quick_sort(start: usize, end: usize, nums: &mut Vec<i32>) {
    if start >= end {
        return;
    }
    let pivot = nums[start + (end - start) / 2];
    let mut left = start;
    let mut right = end;
    while left <= right {
        while left <= right && nums[left] < pivot {
            left += 1;
        }
        while left <= right && nums[right] > pivot {
            right -= 1;
        }
        if left <= right {
            let temp = nums[right];
            nums[right] = nums[left];
            nums[left] = temp;
            left += 1;
            right -= 1;
        }
    }
    quick_sort(start, right, nums);
    quick_sort(left, end, nums);
}

#[bench]
fn bench_my_quick_sort(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut nums = NUMS.to_vec();
        quick_sort(0, LEN-1, &mut nums);
    });
}
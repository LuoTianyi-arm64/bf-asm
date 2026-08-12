// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 LuoTianyi-arm64

pub static mut POINT: usize = 0;

// 仅用于缩短bf代码长度, 并不会提高运行速度
pub const PRE_COM:[[i32; 3]; 114] = [[3, 5, 0], [4, 4, 0], [4, 4, 1], [3, 6, 0], [3, 6, 1], [4, 5, 0], [3, 7, 0], [3, 7, 1], [4, 6, -1], [4, 6, 0], [5, 5, 0], [5, 5, 1], [5, 5, 2], [4, 7, 0], [4, 7, 1], [5, 6, 0], [5, 6, 1], [4, 8, 0], [4, 8, 1], [5, 7, -1], [5, 7, 0], [6, 6, 0], [6, 6, 1], [6, 6, 2], [5, 8, -1], [5, 8, 0], [5, 8, 1], [6, 7, 0], [6, 7, 1], [5, 9, -1], [5, 9, 0], [5, 9, 1], [6, 8, -1], [6, 8, 0], [6, 8, 1], [5, 10, 0], [5, 10, 1], [5, 10, 2], [6, 9, -1], [6, 9, 0], [7, 8, -1], [7, 8, 0], [7, 8, 1], [7, 8, 2], [6, 10, -1], [6, 10, 0], [6, 10, 1], [7, 9, -1], [7, 9, 0], [8, 8, 0], [8, 8, 1], [8, 8, 2], [8, 8, 3], [7, 10, -2], [7, 10, -1], [7, 10, 0], [7, 10, 1], [8, 9, 0], [8, 9, 1], [8, 9, 2], [5, 15, 0], [7, 11, -1], [7, 11, 0], [6, 13, 0], [8, 10, -1], [8, 10, 0], [9, 9, 0], [9, 9, 1], [9, 9, 2], [7, 12, 0], [5, 17, 0], [8, 11, -2], [8, 11, -1], [8, 11, 0], [9, 10, -1], [9, 10, 0], [9, 10, 1], [9, 10, 2], [9, 10, 3], [9, 10, 4], [6, 16, -1], [6, 16, 0], [7, 14, -1], [7, 14, 0], [9, 11, 0], [10, 10, 0], [10, 10, 1], [10, 10, 2], [8, 13, -1], [8, 13, 0], [8, 13, 1], [8, 13, 2], [9, 12, -1], [9, 12, 0], [9, 12, 1], [10, 11, 0], [10, 11, 1], [7, 16, 0], [7, 16, 1], [6, 19, 0], [6, 19, 1], [9, 13, -1], [9, 13, 0], [9, 13, 1], [10, 12, -1], [10, 12, 0], [11, 11, 0], [11, 11, 1], [11, 11, 2], [11, 11, 3], [9, 14, -1], [9, 14, 0], [9, 14, 1], [8, 16, 0]];

#[macro_export]
macro_rules! bf_asm {
    (mov ram $addr: expr, input, target $target: ident) => {
        unsafe {
            let mut output = String::new();
            if POINT > $addr {
                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
            } else if POINT < $addr {
                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
            }
            output.push_str(&format!(","));
            if POINT > $addr {
                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
            } else if POINT < $addr {
                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
            }
            $target.push_str(&output);
        }
    };
    (mov input, target $target: ident) => {
        let mut output = String::new();
        output.push_str(&format!(","));
        $target.push_str(&output);
    };
    (mov output $(, ram $addr: expr)+ , target $target: ident) => {
        $(unsafe {
            let mut output = String::new();
            if POINT > $addr {
                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
            } else if POINT < $addr {
                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
            }
            output.push_str(&format!("."));
            if POINT > $addr {
                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
            } else if POINT < $addr {
                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
            }
            $target.push_str(&output);
        })+

    };
    (mov output, target $target: ident) => {
        let mut output = String::new();
        output.push_str(&format!("."));
        $target.push_str(&output);
    };
    (mov $(ram $addr0:expr ,)+ from_ram $addr1: expr , target $target: ident, clean_target_ram $tag: ident) => {
        $(assert!($addr0 != $addr1, "错误,源和目标不能相同");)+
        let mut output = String::new();
        if $tag {
            $(unsafe {
                if POINT > $addr0 {
                    output.push_str(&format!("{}[-]{}", "<".repeat(POINT - $addr0), ">".repeat(POINT - $addr0)));
                } else if POINT < $addr0 {
                    output.push_str(&format!("{}[-]{}", ">".repeat($addr0 - POINT), "<".repeat($addr0 - POINT)));
                }
            })+
        }
        unsafe {
            if POINT > $addr1 {
                output.push_str(&format!("{}", "<".repeat(POINT - $addr1)));
            } else if POINT < $addr1 {
                output.push_str(&format!("{}", ">".repeat($addr1 - POINT)));
            }
        }
        output.push_str(&format!("[-"));
        $(
            if $addr1 > $addr0 {
                output.push_str(&format!("{}+{}", "<".repeat($addr1 - $addr0), ">".repeat($addr1 - $addr0)));
            } else if $addr1 < $addr0 {
                output.push_str(&format!("{}+{}", ">".repeat($addr0 - $addr1), "<".repeat($addr0 - $addr1)));
            }
        )+
        output.push_str(&format!("]"));
        unsafe {
            if POINT > $addr1 {
                output.push_str(&format!("{}", ">".repeat(POINT - $addr1)));
            } else if POINT < $addr1 {
                output.push_str(&format!("{}", "<".repeat($addr1 - POINT)));
            }
        }
        $target.push_str(&output);
    };
    (mov $(ram $addr: expr ,)+ number $num: expr $(, tmp ram $tmp:expr)? , target $target: ident, clean_target_ram $tag0: ident $(, clean_tmp_ram $tag1: ident)?) => {
        let tmp = {
            let mut tmp: Option<usize> = None;
            $(
                tmp = Some($tmp);
            )?
            tmp
        };
        let tag1 = {
            let mut tag1 = false;
            $(
                tag1 = $tag1;
            )?
            tag1
        };
        let mut output = String::new();
        match tmp {
            None => {
                $(
                    unsafe {
                        if POINT > $addr {
                            output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                        } else if POINT < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                        }
                    }
                    if $tag0 {
                        output.push_str(&format!("[-]"));
                    }
                    output.push_str(&format!("{}", "+".repeat($num)));
                    unsafe {
                        if POINT > $addr {
                            output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                        } else if POINT < $addr {
                            output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                        }
                    }
                )*
            }
            Some(t) => {

                $(match $num{
                    0 => {},
                    1..=14 | 242..= 255  => {
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                            }
                        }
                        if $tag0 {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("{}", "+".repeat($num)));
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                    },
                    15..=128 => {
                        unsafe {
                            if POINT > t {
                                output.push_str(&format!("{}", "<".repeat(POINT - t)));
                            } else if POINT < t {
                                output.push_str(&format!("{}", ">".repeat(t - POINT)));
                            }
                        }
                        if tag1 {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[$num - 15][0] as usize)));
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                            }
                        }
                        if $tag0 {
                            output.push_str(&format!("[-]"));
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                        output.push_str(&format!("[-"));
                        if $addr > t {
                                output.push_str(&format!("{}", ">".repeat($addr - t)));
                        } else if $addr < t {
                                output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[$num - 15][1] as usize)));
                        if $addr > t {
                            output.push_str(&format!("{}", "<".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", ">".repeat(t - $addr)));
                        }
                        output.push_str(&format!("]"));
                        if t > $addr {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        } else if t < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        }
                        if PRE_COM[$num - 15][2] > 0 {
                            output.push_str(&format!("{}","+".repeat(PRE_COM[$num - 15][2] as usize)));
                        } else if PRE_COM[$num - 15][2] < 0 {
                            output.push_str(&format!("{}","-".repeat((0 - PRE_COM[$num - 15][2]) as usize)));
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                    },
                    129..=241 => {
                        unsafe {
                            if POINT > t {
                                output.push_str(&format!("{}", "<".repeat(POINT - t)));
                            } else if POINT < t {
                                output.push_str(&format!("{}", ">".repeat(t - POINT)));
                            }
                        }
                        if tag1 {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[241 - $num][0] as usize)));
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                            }
                        }
                        if $tag0 {
                            output.push_str(&format!("[-]"));
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                        output.push_str(&format!("[-"));
                        if $addr > t {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        }
                        output.push_str(&format!("{}","-".repeat(PRE_COM[241 - $num][1] as usize)));
                        if $addr > t {
                            output.push_str(&format!("{}", "<".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", ">".repeat(t - $addr)));
                        }
                        output.push_str(&format!("]"));
                        if t > $addr {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        } else if t < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        }
                        if PRE_COM[241 - $num][2] > 0 {
                            output.push_str(&format!("{}","-".repeat(PRE_COM[241 - $num][2] as usize)));
                        } else if PRE_COM[241 - $num][2] < 0 {
                            output.push_str(&format!("{}","+".repeat((0 - PRE_COM[241 - $num][2]) as usize)));
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                    },
                    _ => {},
                })*
            }
        }
        $target.push_str(&output);
    };
    (clean $(ram $addr: expr ,)+ target $target: ident) => {
        let mut output = String::new();
        $(
            unsafe {
                if POINT > $addr {
                    output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                } else if POINT < $addr {
                    output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                }
            }
            output.push_str(&format!("[-]"));
            unsafe {
                if POINT > $addr {
                    output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                } else if POINT < $addr {
                    output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                }
            }
        )*
        $target.push_str(&output);
    };
    (add $(ram $addr0:expr ,)+ from_ram $addr1: expr , target $target: ident) => {
        $(assert!($addr0 != $addr1, "错误,源和目标不能相同");)+
        let mut output = String::new();
        unsafe {
            if POINT > $addr1 {
                output.push_str(&format!("{}", "<".repeat(POINT - $addr1)));
            } else if POINT < $addr1 {
                output.push_str(&format!("{}", ">".repeat($addr1 - POINT)));
            }
        }
        output.push_str(&format!("[-"));
        $(
            if $addr1 > $addr0 {
                output.push_str(&format!("{}+{}", "<".repeat($addr1 - $addr0), ">".repeat($addr1 - $addr0)));
            } else if $addr1 < $addr0 {
                output.push_str(&format!("{}+{}", ">".repeat($addr0 - $addr1), "<".repeat($addr0 - $addr1)));
            }
        )+
        output.push_str(&format!("]"));
        unsafe {
            if POINT > $addr1 {
                output.push_str(&format!("{}", ">".repeat(POINT - $addr1)));
            } else if POINT < $addr1 {
                output.push_str(&format!("{}", "<".repeat($addr1 - POINT)));
            }
        }
        $target.push_str(&output);
    };
    (add $(ram $addr: expr ,)+ number $num: expr $(, tmp ram $tmp:expr)? , target $target: ident $(, clean_tmp_ram $tag: ident)?) => {
        let tmp = {
            let mut tmp: Option<usize> = None;
            $(
                tmp = Some($tmp);
            )?
            tmp
        };
        let tag = {
            let mut tag = false;
            $(
                tag = $tag;
            )?
            tag
        };
        let mut output = String::new();
        match tmp {
            None => {
                $(
                    unsafe {
                        if POINT > $addr {
                            output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                        } else if POINT < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                        }
                    }
                    output.push_str(&format!("{}", "+".repeat($num)));
                    unsafe {
                        if POINT > $addr {
                            output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                        } else if POINT < $addr {
                            output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                        }
                    }
                )*
            }
            Some(t) => {

                $(match $num{
                    0 => {},
                    1..=14 | 242..= 255  => {
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                            }
                        }
                        output.push_str(&format!("{}", "+".repeat($num)));
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                    },
                    15..=128 => {
                        unsafe {
                            if POINT > t {
                                output.push_str(&format!("{}", "<".repeat(POINT - t)));
                            } else if POINT < t {
                                output.push_str(&format!("{}", ">".repeat(t - POINT)));
                            }
                        }
                        if tag {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[$num - 15][0] as usize)));
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                            }
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                        output.push_str(&format!("[-"));
                        if $addr > t {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[$num - 15][1] as usize)));
                        if $addr > t {
                            output.push_str(&format!("{}", "<".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", ">".repeat(t - $addr)));
                        }
                        output.push_str(&format!("]"));
                        if t > $addr {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        } else if t < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        }
                        if PRE_COM[$num - 15][2] > 0 {
                            output.push_str(&format!("{}","+".repeat(PRE_COM[$num - 15][2] as usize)));
                        } else if PRE_COM[$num - 15][2] < 0 {
                            output.push_str(&format!("{}","-".repeat((0 - PRE_COM[$num - 15][2]) as usize)));
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                    },


                    129..=241 => {
                        unsafe {
                            if POINT > t {
                                output.push_str(&format!("{}", "<".repeat(POINT - t)));
                            } else if POINT < t {
                                output.push_str(&format!("{}", ">".repeat(t - POINT)));
                            }
                        }
                        if tag {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[241 - $num][0] as usize)));
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                            }
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                        output.push_str(&format!("[-"));
                        if $addr > t {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        }
                        output.push_str(&format!("{}","-".repeat(PRE_COM[241 - $num][1] as usize)));
                        if $addr > t {
                            output.push_str(&format!("{}", "<".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", ">".repeat(t - $addr)));
                        }
                        output.push_str(&format!("]"));
                        if t > $addr {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        } else if t < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        }
                        if PRE_COM[241 - $num][2] > 0 {
                            output.push_str(&format!("{}","-".repeat(PRE_COM[241 - $num][2] as usize)));
                        } else if PRE_COM[241 - $num][2] < 0 {
                            output.push_str(&format!("{}","+".repeat((0 - PRE_COM[241 - $num][2]) as usize)));
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                    },
                    _ => {},
                })*
            }
        }
        $target.push_str(&output);
    };
    (add number $num: expr $(, tmp ram $tmp:expr)? , target $target: ident $(, clean_tmp_ram $tag: ident)?) => {
        let tmp = {
            let mut tmp: Option<usize> = None;
            $(
                tmp = Some($tmp);
            )?
            tmp
        };
        let tag = {
            let mut tag = false;
            $(
                tag = $tag;
            )?
            tag
        };
        let mut output = String::new();
        match tmp {
            None => {
                output.push_str(&format!("{}", "+".repeat($num)));
            }
            Some(t) => {

                $(match $num{
                    0 => {},
                    1..=14 | 242..= 255  => {
                        output.push_str(&format!("{}", "+".repeat($num)));
                    },
                    15..=128 => {
                        unsafe {
                            if POINT > t {
                                output.push_str(&format!("{}", "<".repeat(POINT - t)));
                            } else if POINT < t {
                                output.push_str(&format!("{}", ">".repeat(t - POINT)));
                            }
                        }
                        if tag {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[$num - 15][0] as usize)));
                        if tag1 {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("[-"));
                        if $addr > t {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[$num - 15][1] as usize)));
                        if $addr > t {
                            output.push_str(&format!("{}", "<".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", ">".repeat(t - $addr)));
                        }
                        output.push_str(&format!("]"));
                        if t > $addr {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        } else if t < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        }
                        if PRE_COM[$num - 15][2] > 0 {
                            output.push_str(&format!("{}","+".repeat(PRE_COM[$num - 15][2] as usize)));
                        } else if PRE_COM[$num - 15][2] < 0 {
                            output.push_str(&format!("{}","-".repeat((0 - PRE_COM[$num - 15][2]) as usize)));
                        }
                    },


                    129..=241 => {
                        unsafe {
                            if POINT > t {
                                output.push_str(&format!("{}", "<".repeat(POINT - t)));
                            } else if POINT < t {
                                output.push_str(&format!("{}", ">".repeat(t - POINT)));
                            }
                        }
                        if tag {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[241 - $num][0] as usize)));
                        output.push_str(&format!("[-"));
                        if $addr > t {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        }
                        output.push_str(&format!("{}","-".repeat(PRE_COM[241 - $num][1] as usize)));
                        if $addr > t {
                            output.push_str(&format!("{}", "<".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", ">".repeat(t - $addr)));
                        }
                        output.push_str(&format!("]"));
                        if t > $addr {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        } else if t < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        }
                        if PRE_COM[241 - $num][2] > 0 {
                            output.push_str(&format!("{}","-".repeat(PRE_COM[241 - $num][2] as usize)));
                        } else if PRE_COM[241 - $num][2] < 0 {
                            output.push_str(&format!("{}","+".repeat((0 - PRE_COM[241 - $num][2]) as usize)));
                        }
                    },
                    _ => {},
                })*
            }
        }
        $target.push_str(&output);
    };
    (sub number $num: expr $(, tmp ram $tmp:expr)? , target $target: ident $(, clean_tmp_ram $tag: ident)?) => {
        let tmp = {
            let mut tmp: Option<usize> = None;
            $(
                tmp = Some($tmp);
            )?
            tmp
        };
        let tag = {
            let mut tag = false;
            $(
                tag = $tag;
            )?
            tag
        };
        let mut output = String::new();
        match tmp {
            None => {
                output.push_str(&format!("{}", "-".repeat($num)));
            }
            Some(t) => {

                $(match $num{
                    0 => {},
                    1..=14 | 242..= 255  => {
                        output.push_str(&format!("{}", "-".repeat($num)));
                    },
                    15..=128 => {
                        unsafe {
                            if POINT > t {
                                output.push_str(&format!("{}", "<".repeat(POINT - t)));
                            } else if POINT < t {
                                output.push_str(&format!("{}", ">".repeat(t - POINT)));
                            }
                        }
                        if tag {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[$num - 15][0] as usize)));
                        if tag1 {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("[-"));
                        if $addr > t {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        }
                        output.push_str(&format!("{}","-".repeat(PRE_COM[$num - 15][1] as usize)));
                        if $addr > t {
                            output.push_str(&format!("{}", "<".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", ">".repeat(t - $addr)));
                        }
                        output.push_str(&format!("]"));
                        if t > $addr {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        } else if t < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        }
                        if PRE_COM[$num - 15][2] > 0 {
                            output.push_str(&format!("{}","-".repeat(PRE_COM[$num - 15][2] as usize)));
                        } else if PRE_COM[$num - 15][2] < 0 {
                            output.push_str(&format!("{}","+".repeat((0 - PRE_COM[$num - 15][2]) as usize)));
                        }
                    },


                    129..=241 => {
                        unsafe {
                            if POINT > t {
                                output.push_str(&format!("{}", "<".repeat(POINT - t)));
                            } else if POINT < t {
                                output.push_str(&format!("{}", ">".repeat(t - POINT)));
                            }
                        }
                        if tag {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[241 - $num][0] as usize)));
                        output.push_str(&format!("[-"));
                        if $addr > t {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        }
                        output.push_str(&format!("{}","-".repeat(PRE_COM[241 - $num][1] as usize)));
                        if $addr > t {
                            output.push_str(&format!("{}", "<".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", ">".repeat(t - $addr)));
                        }
                        output.push_str(&format!("]"));
                        if t > $addr {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        } else if t < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        }
                        if PRE_COM[241 - $num][2] > 0 {
                            output.push_str(&format!("{}","+".repeat(PRE_COM[241 - $num][2] as usize)));
                        } else if PRE_COM[241 - $num][2] < 0 {
                            output.push_str(&format!("{}","-".repeat((0 - PRE_COM[241 - $num][2]) as usize)));
                        }
                    },
                    _ => {},
                })*
            }
        }
        $target.push_str(&output);
    };
    (sub $(ram $addr0:expr ,)+ from_ram $addr1: expr , target $target: ident) => {
        $(assert!($addr0 != $addr1, "错误,源和目标不能相同");)+
        let mut output = String::new();
        unsafe {
            if POINT > $addr1 {
                output.push_str(&format!("{}", "<".repeat(POINT - $addr1)));
            } else if POINT < $addr1 {
                output.push_str(&format!("{}", ">".repeat($addr1 - POINT)));
            }
        }
        output.push_str(&format!("[-"));
        $(
            if $addr1 > $addr0 {
                output.push_str(&format!("{}-{}", "<".repeat($addr1 - $addr0), ">".repeat($addr1 - $addr0)));
            } else if $addr1 < $addr0 {
                output.push_str(&format!("{}-{}", ">".repeat($addr0 - $addr1), "<".repeat($addr0 - $addr1)));
            }
        )+
        output.push_str(&format!("]"));
        unsafe {
            if POINT > $addr1 {
                output.push_str(&format!("{}", ">".repeat(POINT - $addr1)));
            } else if POINT < $addr1 {
                output.push_str(&format!("{}", "<".repeat($addr1 - POINT)));
            }
        }
        $target.push_str(&output);
    };
    (sub $(ram $addr: expr ,)+ number $num: expr $(, tmp ram $tmp:expr)? , target $target: ident $(, clean_tmp_ram $tag: ident)?) => {
        let tmp = {
            let mut tmp: Option<usize> = None;
            $(
                tmp = Some($tmp);
            )?
            tmp
        };
        let tag = {
            let mut tag = false;
            $(
                tag = $tag;
            )?
            tag
        };
        let mut output = String::new();
        match tmp {
            None => {
                $(
                    unsafe {
                        if POINT > $addr {
                            output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                        } else if POINT < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                        }
                    }
                    output.push_str(&format!("{}", "-".repeat($num)));
                    unsafe {
                        if POINT > $addr {
                            output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                        } else if POINT < $addr {
                            output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                        }
                    }
                )*
            }
            Some(t) => {

                $(match $num{
                    0 => {},
                    1..=14 | 242..= 255  => {
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                            }
                        }
                        output.push_str(&format!("{}", "-".repeat($num)));
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                    },
                    15..=128 => {
                        unsafe {
                            if POINT > t {
                                output.push_str(&format!("{}", "<".repeat(POINT - t)));
                            } else if POINT < t {
                                output.push_str(&format!("{}", ">".repeat(t - POINT)));
                            }
                        }
                        if tag {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[$num - 15][0] as usize)));
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                            }
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                        output.push_str(&format!("[-"));
                        if $addr > t {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        }
                        output.push_str(&format!("{}","-".repeat(PRE_COM[$num - 15][1] as usize)));
                        if $addr > t {
                            output.push_str(&format!("{}", "<".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", ">".repeat(t - $addr)));
                        }
                        output.push_str(&format!("]"));
                        if t > $addr {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        } else if t < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        }
                        if PRE_COM[$num - 15][2] > 0 {
                            output.push_str(&format!("{}","-".repeat(PRE_COM[$num - 15][2] as usize)));
                        } else if PRE_COM[$num - 15][2] < 0 {
                            output.push_str(&format!("{}","+".repeat((0 - PRE_COM[$num - 15][2]) as usize)));
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                    },
                    128..=241 => {
                        unsafe {
                            if POINT > t {
                                output.push_str(&format!("{}", "<".repeat(POINT - t)));
                            } else if POINT < t {
                                output.push_str(&format!("{}", ">".repeat(t - POINT)));
                            }
                        }
                        if tag {
                            output.push_str(&format!("[-]"));
                        }
                        output.push_str(&format!("{}","+".repeat(PRE_COM[241 - $num][0] as usize)));
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
                            }
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                        output.push_str(&format!("[-"));
                        if $addr > t {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        }
                        output.push_str(&format!("{}","-".repeat(PRE_COM[241 - $num][1] as usize)));
                        if $addr > t {
                            output.push_str(&format!("{}", "<".repeat($addr - t)));
                        } else if $addr < t {
                            output.push_str(&format!("{}", ">".repeat(t - $addr)));
                        }
                        output.push_str(&format!("]"));
                        if t > $addr {
                            output.push_str(&format!("{}", "<".repeat(t - $addr)));
                        } else if t < $addr {
                            output.push_str(&format!("{}", ">".repeat($addr - t)));
                        }
                        if PRE_COM[241 - $num][2] > 0 {
                            output.push_str(&format!("{}","+".repeat(PRE_COM[241 - $num][2] as usize)));
                        } else if PRE_COM[241 - $num][2] < 0 {
                            output.push_str(&format!("{}","-".repeat((0 - PRE_COM[241 - $num][2]) as usize)));
                        }
                        unsafe {
                            if POINT > $addr {
                                output.push_str(&format!("{}", ">".repeat(POINT - $addr)));
                            } else if POINT < $addr {
                                output.push_str(&format!("{}", "<".repeat($addr - POINT)));
                            }
                        }
                    },
                    _ => {},
                })*
            }
        }
        $target.push_str(&output);
    };
    (add_ptr $number: expr, target $target: ident) => {
        unsafe {
            let mut output = String::new();
            output.push_str(&format!("{}", ">".repeat($number)));
            $target.push_str(&output);
            POINT += $number;
        }
    };
    (sub_ptr $number: expr, target $target: ident) => {
        unsafe {
            let mut output = String::new();
            output.push_str(&format!("{}", "<".repeat($number)));
            $target.push_str(&output);
            POINT -= $number;
        }
    };
    (if ram $addr: expr, target $target: ident) => {
        unsafe {
            let mut output = String::new();
            if POINT > $addr {
                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
            } else if POINT < $addr {
                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
            }
            output.push_str(&format!("["));
            $target.push_str(&output);
            POINT = $addr;
        }
    };
    (if target $target: ident) => {
        let mut output = String::new();
        output.push_str(&format!("["));
        $target.push_str(&output);
    };
    (endif ram $addr: expr, target $target: ident) => {
        unsafe {
            let mut output = String::new();
            if POINT > $addr {
                output.push_str(&format!("{}", "<".repeat(POINT - $addr)));
            } else if POINT < $addr {
                output.push_str(&format!("{}", ">".repeat($addr - POINT)));
            }
            output.push_str(&format!("]"));
            $target.push_str(&output);
            POINT = $addr;
        }
    };
    (endif target $target: ident) => {
        let mut output = String::new();
        output.push_str(&format!("]"));
        $target.push_str(&output);
    };
}

#[macro_export]
macro_rules! simplify_bf {
    (code $code: ident, target $target:ident) => {
        let mut stack0 = Vec::new();
        for ch in $code.chars() {
            match ch {
                '>' | '<' => {
                    if let Some(&last) = stack0.last() {
                        if (last == '>' && ch == '<') || (last == '<' && ch == '>') {
                            stack0.pop();
                            continue;
                        }
                    }
                    stack0.push(ch);
                }
                _ => stack0.push(ch),
            }
        }
        let mut stack1 = Vec::new();
        for ch in stack0 {
            match ch {
                '+' | '-' => {
                    if let Some(&last) = stack1.last() {
                        if (last == '+' && ch == '-') || (last == '-' && ch == '+') {
                            stack1.pop();
                            continue;
                        }
                    }
                    stack1.push(ch);
                }
                _ => stack1.push(ch),
            }
        }
        let mut stack2 = Vec::new();
        let mut tag0 = false;
        let mut tag1 = 0;
        let mut tag2 = false;
        let mut tag3 = 0;
        let mut i = 0;
        for &ch in stack1.iter().rev() {
            if tag0 {
                stack2.push(ch);
            } else {
                match ch {
                    '+' | '-' | ',' | '<' | '>' => {
                        if tag2 {
                            tag3 += 1;
                            stack2.push(ch);
                        } else {
                            tag3 = 0;
                        }
                    },
                    ']' => {
                        tag1 += 1;
                        tag3 += 1;
                        tag2 = true;
                        stack2.push(ch);
                    },
                    '[' => {
                        tag1 -= 1;
                        if tag1 == 0 {
                            tag3 += 1;
                            stack2.push(ch);
                            tag2 = false;
                            stack2 = stack2[0..(stack2.len() - tag3)].to_vec();
                        } else {
                            tag3 += 1;
                            stack2.push(ch);
                        }
                    },
                    _ => {
                        stack2.push(ch);
                        tag0 = true;
                        if tag2 {
                            tag3 += 1;
                        }
                    }
                }
            }
            i += 1;
        }
        $target = stack2.iter().rev().collect();
    };
}

// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 LuoTianyi-arm64

pub static mut POINT: usize = 0;

#[macro_export] macro_rules! bf_asm {
    (mov r $addr: expr, in, tar $target: ident) => {
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
    (mov out $(, r $addr: expr)+ , tar $target: ident) => {
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
    (mov $(r $addr0:expr ,)+ fr $addr1: expr , tar $target: ident) => {
        $(assert!($addr0 != $addr1, "错误,源和目标不能相同");)+
        let mut output = String::new();
        $(unsafe {
            if POINT > $addr0 {
                output.push_str(&format!("{}[-]{}", "<".repeat(POINT - $addr0), ">".repeat(POINT - $addr0)));
            } else if POINT < $addr0 {
                output.push_str(&format!("{}[-]{}", ">".repeat($addr0 - POINT), "<".repeat($addr0 - POINT)));
            }
        })+
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
    (mov $(r $addr: expr ,)+ n $num: expr $(, tmp $tmp:expr)? , tar $target: ident) => {
        let tmp = {
            let tmp:Option<usize> = None;
            $(
                tmp = Some($tmp);
            )?
            tmp
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
                    output.push_str(&format!("[-]{}", "+".repeat($num)));
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
                // 仅用于缩短bf代码长度, 并不会提高运行速度
                let pre_com = [[3, 5, 0], [4, 4, 0], [4, 4, 1], [3, 6, 0], [3, 6, 1], [4, 5, 0], [3, 7, 0], [3, 7, 1], [4, 6, -1], [4, 6, 0], [5, 5, 0], [5, 5, 1], [5, 5, 2], [4, 7, 0], [4, 7, 1], [5, 6, 0], [5, 6, 1], [4, 8, 0], [4, 8, 1], [5, 7, -1], [5, 7, 0], [6, 6, 0], [6, 6, 1], [6, 6, 2], [5, 8, -1], [5, 8, 0], [5, 8, 1], [6, 7, 0], [6, 7, 2], [5, 9, -1], [5, 9, 0], [5, 9, 1], [6, 8, -1], [6, 8, 0], [6, 8, 1], [5, 10, 0], [5, 10, 1], [5, 10, 2]];
                todo!()
            }
        }
        $target.push_str(&output);
    };
}


#[macro_export] macro_rules! simplify_bf {
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

        if let Some(pos) = stack1.iter().rposition(|&c| c == '.') {
            $target = stack1[..=pos].iter().collect();
        } else {
            $target = stack1.into_iter().collect();
        }
    }
}



static mut POINT: usize = 0;

macro_rules! bf_asm {
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
                let pre_com = [[3, 5, 0], [4, 4, 0], [4, 4, 1], [3, 6, 0], [3, 6, 1], [4, 5, 0], [3, 7, 0], [3, 7, 1], [4, 6, -1], [4, 6, 0], [5, 5, 0], [5, 5, 1], [5, 5, 2], [4, 7, 0], [4, 7, 1], [5, 6, 0], [5, 6, 1], [4, 8, 0], [4, 8, 1], [5, 7, -1], [5, 7, 0], [6, 6, 0], [6, 6, 1]];
                todo!()
            }
        }
        $target.push_str(&output);
    };
}



fn main() {
    let mut bf_code = String::new();
    bf_asm!(mov r 1, in, tar bf_code);
    bf_asm!(mov r 2, r 3, fr 1, tar bf_code);
    bf_asm!(mov out, r 2, r 3, tar bf_code);
    bf_asm!(mov r 2, r 3, n 2, tar bf_code);
    let bf_code = simplify_bf(&bf_code);
    println!("{bf_code}");
}

fn simplify_bf(code: &str) -> String {
    let mut stack = Vec::new();
    for ch in code.chars() {
        match ch {
            '>' | '<' => {
                if let Some(&last) = stack.last() {
                    if (last == '>' && ch == '<') || (last == '<' && ch == '>') {
                        stack.pop();
                        continue;
                    }
                }
                stack.push(ch);
            }
            _ => stack.push(ch),
        }
    }

    if let Some(pos) = stack.iter().rposition(|&c| c == '.') {
        stack[..=pos].iter().collect()
    } else {
        stack.into_iter().collect()
    }
}

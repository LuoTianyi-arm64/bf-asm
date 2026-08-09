// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 LuoTianyi-arm64
use bf_asm::{bf_asm, POINT};

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
    let mut stack0 = Vec::new();
    for ch in code.chars() {
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
        stack1[..=pos].iter().collect()
    } else {
        stack1.into_iter().collect()
    }
}

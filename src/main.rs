// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 LuoTianyi-arm64
use bf_asm::*;

fn main() {
    let mut bf_code = String::new();
    bf_asm!(mov r 1, in, tar bf_code);
    bf_asm!(mov r 2, r 3, fr 1, tar bf_code);
    bf_asm!(mov out, r 2, r 3, tar bf_code);
    bf_asm!(mov r 2, r 3, n 2, tar bf_code);
    simplify_bf!(code bf_code, target bf_code);
    println!("{bf_code}");
}


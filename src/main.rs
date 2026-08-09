// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 LuoTianyi-arm64

use bf_asm::*;

fn main() {
    let mut bf_code = String::new();
    bf_asm!(mov ram 1, input, target bf_code);
    bf_asm!(mov ram 2, ram 3, from_ram 1, target bf_code);
    bf_asm!(mov output, ram 2, ram 3, target bf_code);
    bf_asm!(mov ram 2, ram 3, number 2, target bf_code);
    simplify_bf!(code bf_code, target bf_code);
    println!("{bf_code}");
}


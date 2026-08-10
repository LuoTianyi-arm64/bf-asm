// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 LuoTianyi-arm64

pub mod asm;
pub use asm::*;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test0() {
        let mut bf_code = String::new();
        bf_asm!(mov ram 0, input, target bf_code);
        bf_asm!(mov ram 1, ram 2, from_ram 0, target bf_code, clean_target_ram false);
        bf_asm!(mov output, ram 1, ram 2, target bf_code);
        simplify_bf!(code bf_code, target bf_code);
        assert_eq!(bf_code, ",[->+>+<<]>.>.<<")
    }
    #[test]
    fn test_print() {
        let mut bf_code = String::new();
        bf_asm!(mov ram 0, number 108, target bf_code, clean_target_ram false);
        bf_asm!(mov ram 1, number 116, target bf_code, clean_target_ram false);
        bf_asm!(mov ram 2, number 121, target bf_code, clean_target_ram false);
        bf_asm!(mov output, ram 0, ram 1, ram 2, target bf_code);
        simplify_bf!(code bf_code, target bf_code);
        assert_eq!(
            bf_code,
            "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++>+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++<<.>.>.<<"
        )
    }
    #[test]
    fn test_mov_num() {
        let mut bf_code0 = String::new();
        bf_asm!(mov ram 0, number 0, tmp ram 1, target bf_code0, clean_target_ram false, clean_tmp_ram false);
        simplify_bf!(code bf_code0, target bf_code0);
        assert_eq!(bf_code0, "");
        let mut bf_code1 = String::new();
        bf_asm!(mov ram 0, number 14, tmp ram 1, target bf_code1, clean_target_ram false, clean_tmp_ram false);
        simplify_bf!(code bf_code1, target bf_code1);
        assert_eq!(bf_code1, "++++++++++++++");
        let mut bf_code2 = String::new();
        bf_asm!(mov ram 0, number 23, tmp ram 1, target bf_code2, clean_target_ram false, clean_tmp_ram false);
        simplify_bf!(code bf_code2, target bf_code2);
        assert_eq!(bf_code2, ">++++[-<++++++>]<-");
        let mut bf_code3 = String::new();
        bf_asm!(mov ram 0, number 155,tmp ram 1, target bf_code3, clean_target_ram false, clean_tmp_ram false);
        simplify_bf!(code bf_code3, target bf_code3);
        assert_eq!(bf_code3, ">++++++++++[-<---------->]<-");
    }
}

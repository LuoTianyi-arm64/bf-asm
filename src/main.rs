use bf_asm::*;

fn main() {
    let mut bf_code = String::new();
    // H
    bf_asm!(mov ram 1, number 72, tmp 0, target bf_code, clean_target_ram false, clean_tmp_ram false);
    bf_asm!(mov output, ram 1, target bf_code);
    // e
    bf_asm!(add ram 1, number 29, tmp 0, target bf_code, clean_tmp_ram false);
    bf_asm!(mov output, ram 1, target bf_code);
    // ll
    bf_asm!(add ram 1, number 7, tmp 0, target bf_code, clean_tmp_ram false);
    bf_asm!(mov output, ram 1, target bf_code);
    bf_asm!(mov output, ram 1, target bf_code);
    // o
    bf_asm!(add ram 1, number 3, tmp 0, target bf_code, clean_tmp_ram false);
    bf_asm!(mov output, ram 1, target bf_code);
    // ,
    bf_asm!(sub ram 1, number 67, tmp 0, target bf_code, clean_tmp_ram false);
    bf_asm!(mov output, ram 1, target bf_code);
    // w
    bf_asm!(add ram 1, number 43, tmp 0, target bf_code, clean_tmp_ram false);
    bf_asm!(mov output, ram 1, target bf_code);
    // o
    bf_asm!(add ram 1, number 24, tmp 0, target bf_code, clean_tmp_ram false);
    bf_asm!(mov output, ram 1, target bf_code);
    // r
    bf_asm!(add ram 1, number 3, tmp 0, target bf_code, clean_tmp_ram false);
    bf_asm!(mov output, ram 1, target bf_code);
    // l
    bf_asm!(sub ram 1, number 6, tmp 0, target bf_code, clean_tmp_ram false);
    bf_asm!(mov output, ram 1, target bf_code);
    // d
    bf_asm!(sub ram 1, number 8, tmp 0, target bf_code, clean_tmp_ram false);
    bf_asm!(mov output, ram 1, target bf_code);
    // !
    bf_asm!(sub ram 1, number 67, tmp 0, target bf_code, clean_tmp_ram false);
    bf_asm!(mov output, ram 1, target bf_code);

    simplify_bf!(code bf_code, target bf_code);
    println!("{bf_code}");
}

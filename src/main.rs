use bf_asm::*;

fn main() {
    let mut bf_code = String::new();
    bf_asm!(mov ram 0, number 129, tmp ram 1, target bf_code, clean_target_ram false, clean_tmp_ram false);
    bf_asm!(mov output, ram 0, target bf_code);
    bf_asm!(clean ram 0, target bf_code);
    simplify_bf!(code bf_code, target bf_code);
    println!("{bf_code}");
}

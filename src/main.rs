use bf_asm::*;

fn main() {
    let mut bf_code = String::new();
    bf_asm!(mov ram 0, number 108,tmp ram 1, target bf_code, clean_target_ram false, clean_tmp_ram false);
    bf_asm!(mov output, ram 0,target bf_code);
    bf_asm!(add ram 0, number 8, target bf_code);
    bf_asm!(mov output, ram 0,target bf_code);
    bf_asm!(add ram 0, number 5, target bf_code);
    bf_asm!(mov output, ram 0,target bf_code);
    println!("{bf_code}");
}

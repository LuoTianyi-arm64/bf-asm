use bf_asm::*;

fn main() {
    let mut bf_code = String::new();
    bf_asm!(mov ram 1, number 108,tmp 0, target bf_code);
    bf_asm!(mov ram 2, number 116,tmp 0, target bf_code);
    bf_asm!(mov ram 3, number 121,tmp 0, target bf_code);
    bf_asm!(mov output, ram 1, ram 2, ram 3, target bf_code);
    simplify_bf!(code bf_code, target bf_code);
    println!("{bf_code}");
}

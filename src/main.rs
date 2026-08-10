use bf_asm::*;

fn main() {
    let mut bf_code = String::new();
    bf_asm!(mov ram 1, number 239,tmp 0, target bf_code);
    simplify_bf!(code bf_code, target bf_code);
    println!("{bf_code}");
}

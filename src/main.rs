use bf_asm::*;

fn main() {
    let mut bf_code = String::new();
    bf_asm!(if ram 0, target bf_code);
    bf_asm!(mov ram 0,input, target bf_code);
    bf_asm!(mov ram 1,input, target bf_code);
    bf_asm!(mov output,ram 1, target bf_code);
    bf_asm!(mov output,ram 0, target bf_code);
    bf_asm!(endif ram 0, target bf_code);
    simplify_bf!(code bf_code, target bf_code);
    println!("{bf_code}");
}

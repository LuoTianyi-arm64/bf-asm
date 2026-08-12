# bf_asm

改项目定义了一个适用于brainfuck的asm,采用100% rust编写

如需在项目中集成此项目,可以用
```toml
[dependencies]
bf_asm = { git = "https://github.com/LuoTianyi-arm64/bf-asm" }
```

使用方法
|  bf_asm!   | 功能  |
|  :----:  | :----:  |
| mov ram N, input, target code  | 将地址为N的内存设为输入, 并将生成的brainfuck代码追加到code(下同) |
| mov input, target code  | 将指针所指内存设为输入 |
| mov output, ram a, ram b, ..., target code  | 将地址为a, b, ...的内存的值输出 |
| mov output, target code  | 将指针所指内存的值输出 |
| mov ram a, ram b, ..., from_ram N, target code, clean_target_ram bool | 将地址为N的内存移动到地址a, b, ..., 并指定是否清空目标内存 |
| mov ram a, ram b, ..., number N(, tmp ram K), target code, clean_target_ram bool(, clean_tmp_ram bool) | 将数值N的内存存储到地址a, b, ..., 指定是否清空目标内存, 同时, 可选的, 指定使用的临时内存以牺牲运行速度来缩短代码长度, 是否清空临时内存 
|
| clean ram a, ram b, ..., target code | 清空地址为a, b , ...的内存 |

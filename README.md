# bf_asm

改项目定义了一个适用于brainfuck的asm,采用100% rust编写

如需在项目中集成此项目,可以用
```toml
[dependencies]
bf_asm = { git = "https://github.com/LuoTianyi-arm64/bf-asm" }
```

使用方法
|  Rust代码   | 功能  |
|  :----:  | :----:  |
| mov ram N, input, target code  | 将地址为N的内存设为输入,并将生成的brainfuck代码追加到code |
| mov input, target code  | 将指针所指内存设为输入,并将生成的brainfuck代码追加到code |

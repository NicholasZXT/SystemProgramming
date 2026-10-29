# System Programming
系统编程练习，包含C、C++、Rust.

项目结构

```txt
system-programming/
├── build/                # C/C++（CMake）产物编译输出目录
├── cmake-build-debug/    # 如果使用CMake和IDE如CLion, 这里存放构建文件
├── docs/                 # 文档（设计文档、API文档等）
├── include/              # 头文件 (公开接口)
│   └── basic/            # 防止头文件名冲突，通常会再嵌套一层目录
│       ├── module1.h
│       └── module2.h
├── src/                  # 源文件 (.c 或 .cpp 文件)
│   ├── module1.cpp
│   ├── module2.cpp
│   └── main.cpp          # 主程序入口
├── lib/                  # 第三方库或预编译库
├── test/                 # 测试代码
├── scripts/              # 脚本文件（构建脚本、部署脚本等）
├── target/               # Rust产物编译输出目录
├── tools/                # 开发工具或配置文件
├── .gitignore            # Git忽略规则
├── CMakeLists.txt        # CMake构建系统配置文件
└── README.md             # 项目描述及使用指南
```

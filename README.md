# logstat

logstat是一个用来统计日志文件行数的命令行工具，同时也是一个入门级的Rust项目。

## 使用
```bash
logstat [OPTIONS] [FILES]...

OPTIONS:
    -h, --help       打印帮助信息
    -F, --filter <FILTER>   过滤日志文件中的行，可以使用多个过滤条件
    -t, --total       统计所有文件的总行数
    -v, --verbose     详细模式，显示每个文件的统计详情
    -f, --format <FORMAT>   输出格式，支持json和text，默认为text
    -P, --pattern <PATTERN>   正则表达式模式，用于匹配日志文件中的行
```

## 功能列表
- [X] 统计文件行数
- [X] 支持多个过滤条件
- [X] 支持标准输入流
- [X] 支持通配符
- [X] 默认输出简洁摘要（文件路径 + 总行数 + 匹配数）
- [X] 详细模式（`-v`）显示每个文件的统计详情
- [X] 支持JSON格式输出（`--format json`）
- [X] 处理100MB日志文件时间 < 1秒（SSD环境）(实测67M文件读取时间约128ms，728M文件读取时间约2.0s)
- [X] 内存占用 < 文件大小的10%
- [X] 支持正则表达式过滤
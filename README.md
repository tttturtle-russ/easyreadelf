### 自己实现一个readelf

用rust实现的简易readelf, 用于学习elf文件格式
实现了如下功能:
- [x] 读取elf文件头
- [x] 读取elf程序头表
- [x] 读取elf节头

#### 使用：
```
cargo build --Release
./target/release/readelf --help // 查看帮助
```
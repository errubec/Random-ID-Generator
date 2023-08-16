# Random ID Generator 随机ID生成器

基于系统当前时间生成一个较短的随机ID。系统时间秒与秒之间生成的ID一定不同，但同一秒内生成的ID有可能重复。主要适用于办公或学习时需要打随机标识符，但[UUID](https://en.wikipedia.org//wiki/Universally_unique_identifier)又因太长而不方便的场景。

## 算法

1. 生成当前的Unix时间戳（从UTC时间1970年1月1日0时起到现在为止的秒数）
2. 将时间戳按10进制转换为字符串并倒序
3. 在倒序的字符串首添加一位非零10进制数字字符
4. 将该字符串用10进制转换为数字
5. 将所得数字用36进制转换回字符串
6. 返回所得结果

## 源代码

本仓库提供了JavaScript和Rust的两种实现，分别放在`JavaScript.js`文件中以及`Rust`文件夹下。
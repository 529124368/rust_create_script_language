## rust_create_script_language

 1. 使用rust 实现的一个简单的脚本语法解释器。
 2. 使用了 rust的nom库。

例子：
```
    global wh = 122.3;
    fn main()
    {
        s=5+(2-3)+(3+4);
        println(s);
        s=5-3+(3+4);
        println(s);
        s=5*3 / (3+4);
        println(s);
        v=122.33;
        println(123.2421);
        println(wh);
        println(v);
        println("sfsdfs  sdf");
    }
```    
 
 ### 目前支持
  1. 全局变量 global
  2. 加减乘除 + - * / 
  3. 换行打印 println（）
  4. 字符串定义  " "
  
  
 ### TODO
  1. 方法调用
  2. 类定义
  3. 错误提示
  4. 循环遍历
  5. if判断
  6. 内置函数功能实现
  ...........等等

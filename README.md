## rust_create_script_language

 1. 使用rust 实现的一个简单的脚本语法解释器。
 2. 使用了 rust的nom库。

例子：
```
    global wh = 122.3;
    fn test() {
        println("你说什么 sfsfs");
    }
    fn main()
    {
        s = 1 +2-3;
        println(s);
        println("hello world !!!");
        s=7+(2-3)+(3+4);
        println(s);
        s=5-3+(3+4);
        test();
        println(s);
        s=5*3 / (3+4);
        println(s);
        v=122.33;
        println(123.2421);
        println(wh);
        println(v);
        println("你好吗 兄弟们");
    }
```    
# AST 抽象语法树 格式
```
Tree {
    root: [
        GlobalParmDef {
            name: "wh",
            token: Zval {
                type_name: Number,
                float: 122.3,
                string: "",
            },
        },
        FunctionDef(
            Function {
                name: "test",
                args: [],
                content: Block {
                    elements: [
                        PrintLn {
                            token: Zval {
                                type_name: String,
                                float: 0.0,
                                string: "你说什么 sfsfs",
                            },
                        },
                    ],
                },
            },
        ),
        FunctionDef(
            Function {
                name: "main",
                args: [],
                content: Block {
                    elements: [
                        Assignment {
                            name: "s",
                            token: Express {
                                opcode: Subtract,
                                left: Express {
                                    opcode: Add,
                                    left: Zval {
                                        type_name: Number,
                                        float: 1.0,
                                        string: "",
                                    },
                                    right: Zval {
                                        type_name: Number,
                                        float: 2.0,
                                        string: "",
                                    },
                                },
                                right: Zval {
                                    type_name: Number,
                                    float: 3.0,
                                    string: "",
                                },
                            },
                        },
                        PrintLn {
                            token: Flg {
                                name: "s",
                            },
                        },
                        PrintLn {
                            token: Zval {
                                type_name: String,
                                float: 0.0,
                                string: "hello world !!!",
                            },
                        },
                        Assignment {
                            name: "s",
                            token: Express {
                                opcode: Add,
                                left: Express {
                                    opcode: Add,
                                    left: Zval {
                                        type_name: Number,
                                        float: 7.0,
                                        string: "",
                                    },
                                    right: Express {
                                        opcode: Subtract,
                                        left: Zval {
                                            type_name: Number,
                                            float: 2.0,
                                            string: "",
                                        },
                                        right: Zval {
                                            type_name: Number,
                                            float: 3.0,
                                            string: "",
                                        },
                                    },
                                },
                                right: Express {
                                    opcode: Add,
                                    left: Zval {
                                        type_name: Number,
                                        float: 3.0,
                                        string: "",
                                    },
                                    right: Zval {
                                        type_name: Number,
                                        float: 4.0,
                                        string: "",
                                    },
                                },
                            },
                        },
                        PrintLn {
                            token: Flg {
                                name: "s",
                            },
                        },
                        Assignment {
                            name: "s",
                            token: Express {
                                opcode: Add,
                                left: Express {
                                    opcode: Subtract,
                                    left: Zval {
                                        type_name: Number,
                                        float: 5.0,
                                        string: "",
                                    },
                                    right: Zval {
                                        type_name: Number,
                                        float: 3.0,
                                        string: "",
                                    },
                                },
                                right: Express {
                                    opcode: Add,
                                    left: Zval {
                                        type_name: Number,
                                        float: 3.0,
                                        string: "",
                                    },
                                    right: Zval {
                                        type_name: Number,
                                        float: 4.0,
                                        string: "",
                                    },
                                },
                            },
                        },
                        CallFunction {
                            name: "test",
                            args: [],
                        },
                        PrintLn {
                            token: Flg {
                                name: "s",
                            },
                        },
                        Assignment {
                            name: "s",
                            token: Express {
                                opcode: Divide,
                                left: Express {
                                    opcode: Multiply,
                                    left: Zval {
                                        type_name: Number,
                                        float: 5.0,
                                        string: "",
                                    },
                                    right: Zval {
                                        type_name: Number,
                                        float: 3.0,
                                        string: "",
                                    },
                                },
                                right: Express {
                                    opcode: Add,
                                    left: Zval {
                                        type_name: Number,
                                        float: 3.0,
                                        string: "",
                                    },
                                    right: Zval {
                                        type_name: Number,
                                        float: 4.0,
                                        string: "",
                                    },
                                },
                            },
                        },
                        PrintLn {
                            token: Flg {
                                name: "s",
                            },
                        },
                        Assignment {
                            name: "v",
                            token: Zval {
                                type_name: Number,
                                float: 122.33,
                                string: "",
                            },
                        },
                        PrintLn {
                            token: Zval {
                                type_name: Number,
                                float: 123.2421,
                                string: "",
                            },
                        },
                        PrintLn {
                            token: Flg {
                                name: "wh",
                            },
                        },
                        PrintLn {
                            token: Flg {
                                name: "v",
                            },
                        },
                        PrintLn {
                            token: Zval {
                                type_name: String,
                                float: 0.0,
                                string: "你好吗 兄弟们",
                            },
                        },
                    ],
                },
            },
        ),
    ],
}

```

 ### 目前支持
  1. 全局变量 global
  2. 加减乘除 + - * / 
  3. 换行打印 println（）
  4. 字符串定义  " "
  5. 无参数 方法调用。（新支持）
  
  
 ### TODO
  1. 带参数方法调用
  2. 类定义
  3. 错误提示
  4. 循环遍历
  5. if判断
  6. 内置函数功能实现
  7. 带入参数计算等。。
  8. ...........（待发掘）

#include <stdio.h>
#include <string.h>

// 这里不声明函数也能编译成功，不过会warn
void printf_summary();
void scanf_summary();

int main()
{
    printf_summary();
    scanf_summary();
    return 0;
}


// 整理 printf() 函数使用要点
void printf_summary(){
    /*
    函数签名
    int printf (const char *__format, ...)
    函数参数：
    printf(格式字符串, 待打印项目1, 待打印项目2, ...)

    printf() 要求打印数据的指令与待打印数据的类型相匹配。
    常用的类型控制符如下：
    %d: 有符号十进制整数
    %u: 无符号十进制整数
    %f: 浮点数，十进制
    %e: 浮点数，e计数法
    %c: 单个字符
    %s: 字符串
    %p: 指针
    %%: 打印一个百分号

    类型控制符中间可以有更加精细的控制，常用控制符如下：
    %4d: 整型数字宽度=4
    %-4d: 整型数字宽度=4，左对齐
    %5.2f: 浮点型，宽度=5，小数点保留2位
    */
    printf("int value: %d, %d\n", 12, 13);
    printf("float value: %f, %f\n", 3.14, 7.68);
    printf("char value: %c, %c, %c, %c\n", 'c', 'h', 'a', 'r');
    printf("string value: %s, %s\n", "s1", "s2");
    printf("percent char: %d%%\n", 90);
    printf("int value, width 3: *%3d*\n", 15);
    printf("int value, width 3: *%3d*\n", 1500);
    printf("int value, width 5, left aligh: *%-5d*\n", 150);
    printf("float value, width 6, e 2: *%6.2f*\n", 3.1415);
    printf("float value, width 6, e 2, left aligh: *%-6.2f*\n", 3.1415);

    // 直接打印char数组
    char c_array[60] = "print char array directly\n";
    printf(c_array);
    // 直接打印字符串
    printf("print string directly\n");
    // 直接打印整数是不行的
    // printf("print int number dircectly: ");
    // printf(100);

}

// 整理 scanf() 函数使用要点
void scanf_summary(){
    /*
    函数签名
    int scanf(const char *__format, ...)
    函数参数：
    scanf(格式字符串, 变量1地址, 变量2地址, ...)

    scanf也要将数据读入对应类型的变量中，并且传入的是变量地址（除了字符数组）
    */
   int number;
   float deciminal;
   char char_array[60];
   printf("please input int and float value:\n");
   scanf("%d %f", &number, &deciminal);
   printf("echo int value: %d, float value: %5.2f\n", number, deciminal);
   printf("please input string(no more than 60 char):");
   scanf("%s", char_array);
   printf("echo char array: %s", char_array);

}
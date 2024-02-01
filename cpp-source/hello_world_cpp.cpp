#include <iostream>
#include <vector>
#include <string>
using namespace std;

// main函数签名其他写法
// int main(void)
// main 函数签名标准写法
int main()
{
    vector<string> msg {"Hello",  "World", "for", "C++", "Program"};
    for (const string& word : msg)
    {
        cout << word << " ";
    }
    cout << endl;
    // getchar();

    // 结尾的return 0 可以省略，仅限于 main 函数
    return 0;
}
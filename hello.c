#include <stdio.h>
typedef void (*rust_callback)(__int32_t);

void run_callback(__int32_t data, rust_callback callback) {
    callback(data + 1); // 调用传过来的回调函数
}

char *char_func(){
    char *str = "hello,world!" ;
    return str ;
}

void my_printer(char *content){
     printf("rust 's string : %s\n", content);
}
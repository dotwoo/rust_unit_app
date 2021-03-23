###
# @Author: your name
# @Date: 2021-03-22 14:39:17
# @LastEditTime: 2021-03-23 17:38:33
# @LastEditors: Please set LastEditors
# @Description: In User Settings Edit
# @FilePath: /csrc/compile.sh
###

# gcc -o unit_app_test -I ../../src -I ../../build -L ../../build/ -lunit nxt_unit_app_test.c
cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=1 ./
c2rust transpile --main

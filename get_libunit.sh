#!/usr/bin/env bash
###
# @Author: dotwoo
# @Date: 2021-03-23 15:06:46
# @LastEditTime: 2021-03-23 15:13:17
# @LastEditors: Please set LastEditors
# @Description: fetch libunit.a
# @FilePath: /rust_unit_app/get_libunit.sh
###
# git clone https://github.com/nginx/unit
cd unit
./configure --prefix=/usr --tests
make build/libunit.a
cp build/libunit.a ../src

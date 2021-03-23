cmake_minimum_required (VERSION 2.6)
project (nxt_app_test)
# 版本号.
set (Tutorial_VERSION_MAJOR 1)
set (Tutorial_VERSION_MINOR 0)


include_directories("../unit/src", "../unit/build")
link_directories("../unit/build")

add_executable(nxt_app_test nxt_unit_app_test.c)
target_link_libraries(nxt_app_test "unit")
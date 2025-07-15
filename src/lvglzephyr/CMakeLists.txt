# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

#set(SHIELD st7789v_tl019fqv01)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(lvgl)

#FILE(GLOB app_sources src/main.c)
#target_sources(app PRIVATE ${app_sources})

FILE(GLOB app_sources src/mainplot.c src/lvgl_demo_ui.c)
target_sources(app PRIVATE ${app_sources})

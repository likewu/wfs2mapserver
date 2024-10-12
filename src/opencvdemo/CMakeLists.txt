cmake_minimum_required(VERSION 3.26.0 FATAL_ERROR)

#set(CMAKE_TOOLCHAIN_FILE D:/Programs/vcpkg/scripts/buildsystems/vcpkg.cmake
#  CACHE STRING "Vcpkg toolchain file")

project("example"
  LANGUAGES CXX
)

#
# Set language version used
#

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED YES)
set(CMAKE_CXX_EXTENSIONS OFF)
#add_definitions(-D_GLIBCXX_USE_CXX11_ABI=1)

#
# Enable modules in CMake
#

# This is required to write your own modules in your project.
if(CMAKE_VERSION VERSION_LESS "3.28.0")
  if(CMAKE_VERSION VERSION_LESS "3.27.0")
    set(CMAKE_EXPERIMENTAL_CXX_MODULE_CMAKE_API "2182bf5c-ef0d-489a-91da-49dbc3090d2a")
  else()
    set(CMAKE_EXPERIMENTAL_CXX_MODULE_CMAKE_API "aa1f7df0-828a-4fcd-9afc-2dc80491aca7")
  endif()
  set(CMAKE_EXPERIMENTAL_CXX_MODULE_DYNDEP 1)
else()
  cmake_policy(VERSION 3.28)
endif()

#
# Import the modules from libc++
#
#include(FetchContent)
#FetchContent_Declare(
#  std
#  URL "file://${LIBCXX_BUILD}/share/libc++/v1/"
#  DOWNLOAD_EXTRACT_TIMESTAMP TRUE
#  SYSTEM
#)
#FetchContent_MakeAvailable(std)

add_compile_definitions(-D_LIBCPP_REMOVE_TRANSITIVE_INCLUDES)
add_compile_options(-fdiagnostics-color -pedantic-errors -Wall -Wextra -Wdeprecated
  -fexperimental-library -stdlib=libc++ -finput-charset=utf-8 -fexec-charset=utf-8
  #-cxx-isystem ${LIBCXX_BUILD}/include/c++/v1
  )
add_link_options(-fuse-ld=lld -stdlib=libc++ -Wl,--color-diagnostics -fdiagnostics-color
  -fexperimental-library)

if(CMAKE_C_COMPILER_ID STREQUAL "MSVC")
  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} /source-charset:utf-8 /execution-charset:gbk")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /source-charset:utf-8 /execution-charset:gbk")
endif()

#message(STATUS "std_SOURCE_DIR: ${std_SOURCE_DIR}")

#add_library(std-cxx-modules STATIC)
#target_sources(std-cxx-modules
#  PUBLIC
#    FILE_SET moduleStd
#    TYPE CXX_MODULES
#    BASE_DIRS ${std_SOURCE_DIR}
#    FILES
#      # Added by https://github.com/compiler-explorer/clang-builder/pull/59
#      ${std_SOURCE_DIR}/std.cppm
#      ${std_SOURCE_DIR}/std.compat.cppm)
#target_compile_options(std-cxx-modules
#  PRIVATE
#    -Wno-reserved-module-identifier
#    -Wno-reserved-user-defined-literal)
#
#link_libraries(std-cxx-modules)

find_package( OpenCV REQUIRED )
find_package( OpenGL )

find_package(Ceres REQUIRED)
#Eigen库只需要添加头文件
#include_directories("D:/Programs/vcpkg/packages/eigen3_x64-windows/eigen3/include")

find_package(Iconv REQUIRED)

if(OPENGL_FOUND)
  include_directories( ${OpenCV_INCLUDE_DIRS} ${OPENGL_INCLUDE_DIRS} )
else()
  include_directories( ${OpenCV_INCLUDE_DIRS} )
endif()

#add_library(coro)
#target_sources(coro
#  PRIVATE coro.cpp
#  PUBLIC
#    FILE_SET CXX_MODULES FILES coro.cppm)

message(STATUS "OpenCV_CUDA: ${OpenCV_CUDA}")

#
# Add the project
#
add_executable( example_12-01 src/example_12-01.cpp )
target_link_libraries( example_12-01 ${OpenCV_LIBS} )
add_executable( example_12-01_dft src/example_12-01_dft.cpp )
target_link_libraries( example_12-01_dft ${OpenCV_LIBS} )
target_link_libraries(example_12-01_dft Iconv::Iconv)

add_executable( example_13-01 src/example_13-01.cpp )
target_link_libraries( example_13-01 ${OpenCV_LIBS} )

add_executable( example_15-02 src/example_15-02.cpp )
#add_dependencies(main std.compat)
#target_link_libraries(main std.compat)
target_link_libraries( example_15-02 ${OpenCV_LIBS} )

add_executable( example_17-01 src/example_17-01.cpp )
target_link_libraries( example_17-01 ${OpenCV_LIBS} )
add_executable( example_17-02 src/example_17-02.cpp )
target_link_libraries( example_17-02 ${OpenCV_LIBS} )

add_executable( example_19-03 src/example_19-03.cpp )
target_link_libraries( example_19-03 ${OpenCV_LIBS} )

add_executable( simple-kalman-tracker src/simple-kalman-tracker.cpp )
target_link_libraries( simple-kalman-tracker ${OpenCV_LIBS} )
add_executable( BallTracker src/BallTracker.cpp )
target_link_libraries( BallTracker ${OpenCV_LIBS} )
#add_executable( GolfBallOpenCV src/GolfBallOpenCV.cpp )
#target_link_libraries( GolfBallOpenCV ${OpenCV_LIBS} )
add_executable( ballTrackingWithColor src/ballTrackingWithColor.cpp )
target_link_libraries( ballTrackingWithColor ${OpenCV_LIBS} )
add_executable( ballTrackingWithHoughCircles src/ballTrackingWithHoughCircles.cpp )
target_link_libraries( ballTrackingWithHoughCircles ${OpenCV_LIBS} )

add_executable( pose_estimation_2d2d src/pose_estimation_2d2d.cpp )
target_link_libraries( pose_estimation_2d2d ${OpenCV_LIBS} )
#add_executable( bundle_adjustment_ceres src/bundle_adjustment_ceres.cpp src/common.cpp )
#target_link_libraries( bundle_adjustment_ceres ${OpenCV_LIBS} ${CERES_LIBRARIES} )
cmake_minimum_required(VERSION 3.26.0 FATAL_ERROR)

#set(CMAKE_TOOLCHAIN_FILE D:/Programs/vcpkg/scripts/buildsystems/vcpkg.cmake
#  CACHE STRING "Vcpkg toolchain file")

project("example"
  LANGUAGES C CXX
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

add_executable( hello11 examples/hello11.c )
#target_link_libraries( hello11 gmp gmpxx )

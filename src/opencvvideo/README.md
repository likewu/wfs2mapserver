# opencvvideo

$env.LIBCLANG_PATH = F:\clang+llvm-18.1.6-x86_64-pc-windows-msvc\bin\libclang.dll
$env.Path = ($env.Path | prepend 'F:\clang+llvm-18.1.6-x86_64-pc-windows-msvc\bin' | prepend 'D:\Programs\opencv\build\x64\vc16\bin' | prepend 'D:\Programs\opencv\build\bin')
$env.OPENCV_LINK_LIBS = opencv_world490
$env.OPENCV_LINK_PATHS = D:\Programs\opencv\build\x64\vc16\lib
$env.OPENCV_INCLUDE_PATHS = D:\Programs\opencv\build\include

vs 2022:
$env:LIBCLANG_PATH = "F:\clang+llvm-18.1.6-x86_64-pc-windows-msvc\bin\libclang.dll"
$env:PATH += ";F:\clang+llvm-18.1.6-x86_64-pc-windows-msvc\bin;D:\Programs\opencv\build\x64\vc16\bin;D:\Programs\opencv\build\bin"
$env:OPENCV_LINK_LIBS = "opencv_world490"
$env:OPENCV_LINK_PATHS = "D:\Programs\opencv\build\x64\vc16\lib"
$env:OPENCV_INCLUDE_PATHS = "D:\Programs\opencv\build\include"

cargo test -p opencvvideo orb
cargo run -p opencvvideo --example orb
lldb-server gdbserver *:1234 ./target/debug/examples/dense_mapping.exe
//*

lldb-server platform --server --listen "*:1234"
(lldb) platform select remote-linux
(lldb) platform connect connect://remote:1234
(lldb) file a.out
(lldb) run
(lldb) platform settings -w /usr/local/bin
(lldb) platform status
//*

HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Environment

计算机\HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\UsoSvc


https://kkgithub.com/gigahidjrikaaa/Engineering-Books/blob/main/Computer Vision/Kaehler%2C Bradski - Learning OpenCV 3 Computer Vision in C%2B%2B With the OpenCV Library.pdf?raw=true


D:\.cargo\registry\src\mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd\ceres-solver-sys-0.3.0\build.rs
/std:c++17
D:\.cargo\registry\src\mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd\ceres-solver-sys-0.3.0\src\lib.cpp
#define GLOG_NO_ABBREVIATED_SEVERITIES
D:\.cargo\registry\src\mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd\ceres-solver-sys-0.3.0\src\lib.h
#define GLOG_NO_ABBREVIATED_SEVERITIES

D:\.cargo\registry\src\mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd\ceres-solver-src-0.3.0+ceres2.2.0-eigen3.4.0-glog0.7.0\build.rs
.define("GLOG_USE_GLOG_EXPORT", "ON")
.cflag("/std:c++17")
D:\.cargo\registry\src\mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd\ceres-solver-src-0.3.0+ceres2.2.0-eigen3.4.0-glog0.7.0\vendor\ceres-solver\CMakeLists.txt
add_definitions("-DGLOG_USE_GLOG_EXPORT")

D:\Programs\vcpkg\vcpkg integrate install


https://github.com/GreptimeTeam/greptimedb/blob/develop/src/common/mem-prof/README.md
https://github.com/tikv/jemallocator
MALLOC_CONF=prof:true,lg_prof_interval:28
jeprof --svg <path_to_greptimedb_binary> --base=<baseline_prof> <profile_data> > output.svg
jeprof --show_bytes --pdf <path_to_binary> ./profile.out > ./profile.pdf


https://github.com/tokio-rs/console
tokio-console

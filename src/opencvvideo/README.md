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

HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Environment

计算机\HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\UsoSvc

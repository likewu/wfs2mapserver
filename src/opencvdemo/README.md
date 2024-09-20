# opencvdemo

vs 2022:
$env:LIBCLANG_PATH = "F:\clang+llvm-18.1.6-x86_64-pc-windows-msvc\bin\libclang.dll"
$env:PATH += ";D:\LLVM-19.1.0-Windows-X64\bin;D:\Programs\opencv\build\x64\vc16\bin;D:\Programs\opencv\build\bin"

$env:OpenCV_DIR = "D:\Programs\opencv\build"
cmake -G Ninja -S . -B build -DCMAKE_CXX_COMPILER=D:/LLVM-19.1.0-Windows-X64/bin/clang++.exe -DLIBCXX_BUILD=D:/LLVM-19.1.0-Windows-X64 -DOpenCV_RUNTIME=vc16 -DOpenCV_ARCH=x64
ninja -C build

D:\Programs\vcpkg\vcpkg integrate install

# opencvdemo

vs 2022:
$env:LIBCLANG_PATH = "D:\LLVM-19.1.0-Windows-X64\bin\libclang.dll"
$env:PATH += ";D:\LLVM-19.1.0-Windows-X64\bin;D:\Programs\opencv\build\x64\vc16\bin;D:\Programs\opencv\build\bin"

$env:OpenCV_DIR = "D:\Programs\opencv\build"
cmake -G Ninja -S . -B build -DCMAKE_CXX_COMPILER=D:/LLVM-19.1.0-Windows-X64/bin/clang++.exe -DLIBCXX_BUILD=D:/LLVM-19.1.0-Windows-X64 -DOpenCV_RUNTIME=vc16 -DOpenCV_ARCH=x64
 -DOpenCV_STATIC=ON
ninja -C build
.\build\example_15-02.exe 50 tree.avi
.\build\example_13-01.exe ../opencvvideo/tests/HandIndoorColor.jpg


D:\Programs\vcpkg\vcpkg integrate install

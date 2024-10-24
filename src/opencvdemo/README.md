# opencvdemo

vs 2022:
$env:LIBCLANG_PATH = "D:\LLVM-19.1.0-Windows-X64\bin\libclang.dll"
$env:PATH += ";D:\LLVM-19.1.0-Windows-X64\bin;D:\Programs\opencv\Debug\bin;D:\Programs\oneAPI\tbb\2021.13\bin"

$env:TBB_DLL_PATH = "D:\Programs\oneAPI\tbb\2021.13\bin"
$env:OpenCV_DIR = "D:\Programs\opencv\Debug"
cmake -G Ninja -S . -B build -DCMAKE_CXX_COMPILER=D:/LLVM-19.1.0-Windows-X64/bin/clang++.exe -DLIBCXX_BUILD=D:/LLVM-19.1.0-Windows-X64 -DOpenCV_RUNTIME=vc16 -DOpenCV_ARCH=x64 -DOpenCV_LIB_PATH=D:/Programs/opencv/Debug/lib -DCMAKE_TOOLCHAIN_FILE=D:/Programs/vcpkg/scripts/buildsystems/vcpkg.cmake
-DVCPKG_TARGET_TRIPLET=x64-windows
 -DOpenCV_STATIC=ON
ninja -C build
.\build\example_15-02.exe 50 tree.avi
.\build\example_13-01.exe ../opencvvideo/tests/HandIndoorColor.jpg


D:\Programs\vcpkg\vcpkg integrate install

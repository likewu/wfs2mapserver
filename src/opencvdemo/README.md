# opencvdemo

vs 2022:
$env:VCPKG_ROOT = "D:/Programs/vcpkg"
$env:LIBCLANG_PATH = "F:\clang+llvm-20.1.3-x86_64-pc-windows-msvc\bin\libclang.dll"
$env:PATH += ";F:\clang+llvm-20.1.3-x86_64-pc-windows-msvc\bin;D:\Programs\opencv\Debug\bin;D:\Programs\oneAPI\tbb\2021.13\bin"

$env:TBB_DLL_PATH = "D:\Programs\oneAPI\tbb\2021.13\bin"
$env:OpenCV_DIR = "D:\Programs\opencv\Debug"
cmake -G Ninja -S . -B build -DCMAKE_CXX_COMPILER=F:/clang+llvm-20.1.3-x86_64-pc-windows-msvc/bin/clang++.exe -DLIBCXX_BUILD=F:/clang+llvm-20.1.3-x86_64-pc-windows-msvc -DOpenCV_RUNTIME=vc16 -DOpenCV_ARCH=x64 -DOpenCV_LIB_PATH=D:/Programs/opencv/Debug/lib -DCMAKE_TOOLCHAIN_FILE=D:/Programs/vcpkg/scripts/buildsystems/vcpkg.cmake
-DVCPKG_TARGET_TRIPLET=x64-windows
 -DOpenCV_STATIC=ON
ninja -C build
.\build\example_15-02.exe 50 tree.avi
.\build\example_13-01.exe ../opencvvideo/tests/HandIndoorColor.jpg


D:\Programs\vcpkg\vcpkg integrate install


https://kkgithub.com/thommyho/Cpp-OpenCV-Windows-PreBuilts/releases/tag/v4.9.0

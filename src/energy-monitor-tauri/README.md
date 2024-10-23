# Energy Monitor Demo

$env.JAVA_HOME = D:/jdk17
$env.ANDROID_HOME = E:\Embarcadero\Studio\22.0\PlatformSDKs\android-sdk-windows
#$env.NDK_HOME = E:\Embarcadero\Studio\22.0\PlatformSDKs\android-ndk-r21
$env.NDK_HOME = G:\android-ndk-r27
cargo tauri android init 
cargo tauri android dev
cargo tauri android build --apk --target aarch64

cargo tauri build --no-bundle --target x86_64-pc-windows-msvc

[project]/src-tauri/gen/android/keystore.properties
password=<password defined when keytool was executed>
keyAlias=leafcolor
storeFile=E:\\www\\app-pvtool-android\\clips\\leafcolor.keystore


$env.RUSTUP_DIST_SERVER = https://static.rust-lang.org
$env.RUSTUP_UPDATE_ROOT = https://static.rust-lang.org/rustup
RUSTUP_DIST_SERVER= rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

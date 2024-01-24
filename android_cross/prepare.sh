rustup target add aarch64-linux-android armv7-linux-androideabi
echo "export NDK=/opt/android-ndk-r26b"

$NDK/build/tools/make_standalone_toolchain.py --api 21 --arch arm64 --install-dir /tmp/ndk-aarch64
$NDK/build/tools/make_standalone_toolchain.py --api 21 --arch arm --install-dir /tmp/ndk-armv7


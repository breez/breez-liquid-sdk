.PHONY: init
init:
	make -C ../../lib/bindings init
	
.PHONY: clean
clean:
	make -C ../../lib/bindings clean

all: android ios react-native

react-native:
	make -C ../../lib/bindings react-native
	cp -r ../../lib/bindings/langs/react-native/android/* android/
	cp -r ../../lib/bindings/langs/react-native/ios/* ios/
	cp -r ../../lib/bindings/langs/react-native/ts/src/* src/

android-make:
	make -C ../../lib/bindings android

android-copy:
	mkdir -p android/src/main/jniLibs/x86
	mkdir -p android/src/main/jniLibs/x86_64
	mkdir -p android/src/main/jniLibs/arm64-v8a
	mkdir -p android/src/main/jniLibs/armeabi-v7a
	cp ../../lib/bindings/ffi/kotlin/breez_sdk_liquid/breez_sdk_liquid.kt android/src/main/java/com/breezsdkliquid/breez_sdk_liquid.kt
	cp ../../lib/bindings/ffi/kotlin/jniLibs/x86/libbreez_sdk_liquid_bindings.so android/src/main/jniLibs/x86/libbreez_sdk_liquid_bindings.so
	cp ../../lib/bindings/ffi/kotlin/jniLibs/x86_64/libbreez_sdk_liquid_bindings.so android/src/main/jniLibs/x86_64/libbreez_sdk_liquid_bindings.so
	cp ../../lib/bindings/ffi/kotlin/jniLibs/arm64-v8a/libbreez_sdk_liquid_bindings.so android/src/main/jniLibs/arm64-v8a/libbreez_sdk_liquid_bindings.so
	cp ../../lib/bindings/ffi/kotlin/jniLibs/armeabi-v7a/libbreez_sdk_liquid_bindings.so android/src/main/jniLibs/armeabi-v7a/libbreez_sdk_liquid_bindings.so

android: android-make android-copy

ios-make:
	make -C ../../lib/bindings bindings-swift

ios-copy:
	rm -rf ios/bindings-swift
	cp -r ../../lib/bindings/langs/swift ios/bindings-swift
	rm -f ios/bindings-swift/Package.swift

ios: ios-make ios-copy

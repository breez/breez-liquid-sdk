.PHONY: init
init:
	make -C ../../ init

darwin:
	make -C ../../ python-darwin
	cp ../../ffi/python/breez_sdk_liquid.py src/breez_sdk_liquid
	cp ../../ffi/python/libbreez_sdk_liquid_bindings.dylib src/breez_sdk_liquid

linux:
	make -C ../../ python-linux
	cp ../../ffi/python/breez_sdk_liquid.py src/breez_sdk_liquid
	cp ../../ffi/python/libbreez_sdk_liquid_bindings.so src/breez_sdk_liquid

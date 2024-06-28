package com.breezsdkliquid

import breez_sdk_liquid.LogEntry
import breez_sdk_liquid.Logger
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter

class BreezSDKLiquidLogger(
    private val emitter: RCTDeviceEventEmitter,
) : Logger {
    companion object {
        var emitterName = "breezSdkLiquidLog"
    }

    override fun log(l: LogEntry) {
        emitter.emit(emitterName, readableMapOf(l))
    }
}

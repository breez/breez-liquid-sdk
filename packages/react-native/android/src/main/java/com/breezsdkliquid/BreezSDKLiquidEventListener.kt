package com.breezsdkliquid

import breez_sdk_liquid.EventListener
import breez_sdk_liquid.SdkEvent
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter

class BreezSDKEventListener(
    private val emitter: RCTDeviceEventEmitter,
) : EventListener {
    private var id: String? = null

    fun setId(id: String) {
        this.id = id
    }

    override fun onEvent(e: SdkEvent) {
        this.id?.let {
            emitter.emit("event-$it", readableMapOf(e))
        }
    }
}

package breez_sdk_liquid_notification

import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.ConnectRequest
import breez_sdk_liquid.EventListener
import breez_sdk_liquid.connect

class BreezSdkLiquidConnector {
    companion object {
        private const val TAG = "BreezSdkLiquidConnector"

        private var liquidSDK: BindingLiquidSdk? = null

        internal fun connectSDK(
            connectRequest: ConnectRequest,
            sdkListener: EventListener,
            logger: ServiceLogger
        ): BindingLiquidSdk {
            synchronized(this) {
                if (liquidSDK == null) {
                    logger.log(
                        TAG, "Connecting to Breez Liquid SDK", "DEBUG"
                    )
                    liquidSDK = connect(connectRequest)
                    logger.log(TAG, "Connected to Breez Liquid SDK", "DEBUG")
                    liquidSDK!!.addEventListener(sdkListener)
                } else logger.log(TAG, "Already connected to Breez Liquid SDK", "DEBUG")

                return liquidSDK!!
            }
        }
    }
}

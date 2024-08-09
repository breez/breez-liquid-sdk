package breez_sdk_liquid_notification.job

import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.EventListener

interface Job : EventListener {
    /** When the notification service is connected to the Breez Liquid SDK
     *  it calls `start` to initiate the job.
     */
    fun start(liquidSDK: BindingLiquidSdk)

    /** When the short service timeout is reached it calls `onShutdown` 
     *  to cleanup the job.
     */
    fun onShutdown()
}

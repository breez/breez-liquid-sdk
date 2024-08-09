package breez_sdk_liquid_notification.job

import android.content.Context
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.SdkEvent
import breez_sdk_liquid_notification.Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.NOTIFICATION_CHANNEL_SWAP_UPDATED
import breez_sdk_liquid_notification.Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT
import breez_sdk_liquid_notification.Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.SWAP_CONFIRMED_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_liquid_notification.ResourceHelper.Companion.getString
import breez_sdk_liquid_notification.SdkForegroundService
import breez_sdk_liquid_notification.ServiceLogger
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import java.security.MessageDigest

@Serializable
data class SwapUpdatedRequest(
    val id: String,
    val status: String,
)

class SwapUpdatedJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
    private val payload: String,
    private val logger: ServiceLogger,
    private var swapIdHash: String? = null,
) : Job {
    companion object {
        private const val TAG = "SwapUpdatedJob"
    }

    override fun start(liquidSDK: BindingLiquidSdk) {
        try {
            val request = Json.decodeFromString(SwapUpdatedRequest.serializer(), payload)
            this.swapIdHash = request.id
        } catch (e: Exception) {
            logger.log(TAG, "Failed to decode payload: ${e.message}", "WARN")
        }
    }

    override fun onEvent(e: SdkEvent) {
        when (e) {
            is SdkEvent.PaymentSucceeded -> {
                val payment = e.details

                payment.swapId?.let { swapId ->
                    if (this.swapIdHash == hashId(swapId)) {
                        logger.log(
                            TAG,
                            "Received payment succeeded event: ${this.swapIdHash}",
                            "TRACE"
                        )
                        notifySuccess()
                    }
                }
            }

            else -> {
                logger.log(TAG, "Received event: ${e}", "TRACE")
            }
        }
    }

    override fun onShutdown() {
        notifyFailure()
    }

    private fun hashId(id: String): String =
        MessageDigest.getInstance("SHA-256")
            .digest(id.toByteArray())
            .fold(StringBuilder()) { sb, it -> sb.append("%02x".format(it)) }
            .toString()

    private fun notifySuccess() {
        this.swapIdHash?.let { swapIdHash ->
            logger.log(TAG, "Swap $swapIdHash processed successfully", "INFO")
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_SWAP_UPDATED,
                getString(
                    context,
                    SWAP_CONFIRMED_NOTIFICATION_TITLE,
                    DEFAULT_SWAP_CONFIRMED_NOTIFICATION_TITLE
                ),
            )
            fgService.onFinished(this)
        }
    }

    private fun notifyFailure() {
        this.swapIdHash?.let { swapIdHash ->
            logger.log(TAG, "Swap $swapIdHash processing failed", "INFO")
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_SWAP_UPDATED,
                getString(
                    context,
                    SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE,
                    DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE
                ),
                getString(
                    context,
                    SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT,
                    DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT
                ),
            )
        }
    }
}

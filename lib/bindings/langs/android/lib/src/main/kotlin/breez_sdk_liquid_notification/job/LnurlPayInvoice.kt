package breez_sdk_liquid_notification.job

import android.content.Context
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.PrepareReceivePaymentRequest
import breez_sdk_liquid.ReceivePaymentRequest
import breez_sdk_liquid_notification.Constants.DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT
import breez_sdk_liquid_notification.Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.LNURL_PAY_INVOICE_NOTIFICATION_TITLE
import breez_sdk_liquid_notification.Constants.LNURL_PAY_METADATA_PLAIN_TEXT
import breez_sdk_liquid_notification.Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE
import breez_sdk_liquid_notification.Constants.NOTIFICATION_CHANNEL_LNURL_PAY
import breez_sdk_liquid_notification.NotificationHelper.Companion.notifyChannel
import breez_sdk_liquid_notification.ResourceHelper.Companion.getString
import breez_sdk_liquid_notification.SdkForegroundService
import breez_sdk_liquid_notification.ServiceLogger
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

@Serializable
data class LnurlInvoiceRequest(
    @SerialName("amount") val amount: ULong,
    @SerialName("reply_url") val replyURL: String,
)

// Serialize the response according to to LUD-06 payRequest base specification:
// https://github.com/lnurl/luds/blob/luds/06.md
@Serializable
data class LnurlPayInvoiceResponse(
    val pr: String,
    val routes: List<String>,
)

class LnurlPayInvoiceJob(
    private val context: Context,
    private val fgService: SdkForegroundService,
    private val payload: String,
    private val logger: ServiceLogger,
) : LnurlPayJob {
    companion object {
        private const val TAG = "LnurlPayInvoiceJob"
    }

    override fun start(liquidSDK: BindingLiquidSdk) {
        var request: LnurlInvoiceRequest? = null
        try {
            request = Json.decodeFromString(LnurlInvoiceRequest.serializer(), payload)
            // Get the lightning limits
            val limits = liquidSDK.fetchLightningLimits()
            // Check amount is within limits
            val amountSat = request.amount / 1000UL
            if (amountSat < limits.receive.minSat || amountSat > limits.receive.maxSat) {
                fail("Invalid amount requested ${request.amount}", request.replyURL)
                notifyChannel(
                    context,
                    NOTIFICATION_CHANNEL_LNURL_PAY,
                    getString(
                        context,
                        LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                        DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
                    ),
                )
                return
            }
            val plainTextMetadata = getString(
                context,
                LNURL_PAY_METADATA_PLAIN_TEXT,
                DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT
            )
            val prepareReceivePaymentRes = liquidSDK.prepareReceivePayment(
                PrepareReceivePaymentRequest(amountSat)
            )
            val receivePaymentResponse = liquidSDK.receivePayment(
                ReceivePaymentRequest(
                    prepareReceivePaymentRes,
                    description = "[[\"text/plain\",\"$plainTextMetadata\"]]",
                    useDescriptionHash = true
                )
            )
            val response =
                LnurlPayInvoiceResponse(
                    receivePaymentResponse.invoice,
                    listOf(),
                )
            val success = replyServer(Json.encodeToString(response), request.replyURL)
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_LNURL_PAY,
                getString(
                    context,
                    if (success) LNURL_PAY_INVOICE_NOTIFICATION_TITLE else LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                    if (success) DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE else DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
                ),
            )
        } catch (e: Exception) {
            logger.log(TAG, "Failed to process lnurl: ${e.message}", "WARN")
            if (request != null) {
                fail(e.message, request.replyURL)
            }
            notifyChannel(
                context,
                NOTIFICATION_CHANNEL_LNURL_PAY,
                getString(
                    context,
                    LNURL_PAY_NOTIFICATION_FAILURE_TITLE,
                    DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE
                ),
            )
        }

        fgService.onFinished(this)
    }

    override fun onShutdown() {}
}

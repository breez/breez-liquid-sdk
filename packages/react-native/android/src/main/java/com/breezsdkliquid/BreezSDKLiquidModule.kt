package com.breezsdkliquid

import breez_sdk_liquid.*
import com.facebook.react.bridge.*
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter
import java.io.File
import java.util.*
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors

class BreezSDKLiquidModule(
    reactContext: ReactApplicationContext,
) : ReactContextBaseJavaModule(reactContext) {
    private lateinit var executor: ExecutorService
    private var bindingLiquidSdk: BindingLiquidSdk? = null

    companion object {
        const val TAG = "RNBreezSDKLiquid"
    }

    override fun initialize() {
        super.initialize()

        executor = Executors.newFixedThreadPool(3)
    }

    override fun getName(): String = TAG

    @Throws(SdkException::class)
    fun getBindingLiquidSdk(): BindingLiquidSdk {
        if (bindingLiquidSdk != null) {
            return bindingLiquidSdk!!
        }

        throw SdkException.Generic("Not initialized")
    }

    @Throws(SdkException::class)
    private fun ensureWorkingDir(workingDir: String) {
        try {
            val workingDirFile = File(workingDir)

            if (!workingDirFile.exists() && !workingDirFile.mkdirs()) {
                throw SdkException.Generic("Mandatory field workingDir must contain a writable directory")
            }
        } catch (e: SecurityException) {
            throw SdkException.Generic("Mandatory field workingDir must contain a writable directory")
        }
    }

    @ReactMethod
    fun addListener(eventName: String) {}

    @ReactMethod
    fun removeListeners(count: Int) {}

    @ReactMethod
    fun defaultConfig(
        network: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val networkTmp = asLiquidNetwork(network)
                val res = defaultConfig(networkTmp)
                val workingDir = File(reactApplicationContext.filesDir.toString() + "/breezSdkLiquid")

                res.workingDir = workingDir.absolutePath
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun parse(
        input: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = parse(input)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun parseInvoice(
        input: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val res = parseInvoice(input)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun setLogger(promise: Promise) {
        executor.execute {
            try {
                val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)

                setLogger(BreezSDKLiquidLogger(emitter))
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                e.printStackTrace()
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun connect(
        req: ReadableMap,
        promise: Promise,
    ) {
        if (bindingLiquidSdk != null) {
            promise.reject("Generic", "Already initialized")
            return
        }

        executor.execute {
            try {
                var connectRequest =
                    asConnectRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "ConnectRequest")) }

                ensureWorkingDir(connectRequest.config.workingDir)

                bindingLiquidSdk = connect(connectRequest)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun addEventListener(promise: Promise) {
        executor.execute {
            try {
                val emitter = reactApplicationContext.getJSModule(RCTDeviceEventEmitter::class.java)
                var eventListener = BreezSDKEventListener(emitter)
                val res = getBindingLiquidSdk().addEventListener(eventListener)

                eventListener.setId(res)
                promise.resolve(res)
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun removeEventListener(
        id: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                getBindingLiquidSdk().removeEventListener(id)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun getInfo(promise: Promise) {
        executor.execute {
            try {
                val res = getBindingLiquidSdk().getInfo()
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun prepareSendPayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareSendRequest =
                    asPrepareSendRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PrepareSendRequest")) }
                val res = getBindingLiquidSdk().prepareSendPayment(prepareSendRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun sendPayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareSendResponse =
                    asPrepareSendResponse(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PrepareSendResponse")) }
                val res = getBindingLiquidSdk().sendPayment(prepareSendResponse)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun prepareReceivePayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareReceivePaymentRequest =
                    asPrepareReceivePaymentRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PrepareReceivePaymentRequest")) }
                val res = getBindingLiquidSdk().prepareReceivePayment(prepareReceivePaymentRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun receivePayment(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val receivePaymentRequest =
                    asReceivePaymentRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "ReceivePaymentRequest")) }
                val res = getBindingLiquidSdk().receivePayment(receivePaymentRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun fetchLightningLimits(promise: Promise) {
        executor.execute {
            try {
                val res = getBindingLiquidSdk().fetchLightningLimits()
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun fetchOnchainLimits(promise: Promise) {
        executor.execute {
            try {
                val res = getBindingLiquidSdk().fetchOnchainLimits()
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun preparePayOnchain(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val preparePayOnchainRequest =
                    asPreparePayOnchainRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PreparePayOnchainRequest")) }
                val res = getBindingLiquidSdk().preparePayOnchain(preparePayOnchainRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun payOnchain(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val payOnchainRequest =
                    asPayOnchainRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PayOnchainRequest")) }
                val res = getBindingLiquidSdk().payOnchain(payOnchainRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun prepareReceiveOnchain(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareReceiveOnchainRequest =
                    asPrepareReceiveOnchainRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PrepareReceiveOnchainRequest")) }
                val res = getBindingLiquidSdk().prepareReceiveOnchain(prepareReceiveOnchainRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun receiveOnchain(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareReceiveOnchainResponse =
                    asPrepareReceiveOnchainResponse(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PrepareReceiveOnchainResponse")) }
                val res = getBindingLiquidSdk().receiveOnchain(prepareReceiveOnchainResponse)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun prepareBuyBitcoin(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareBuyBitcoinRequest =
                    asPrepareBuyBitcoinRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PrepareBuyBitcoinRequest")) }
                val res = getBindingLiquidSdk().prepareBuyBitcoin(prepareBuyBitcoinRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun buyBitcoin(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val buyBitcoinRequest =
                    asBuyBitcoinRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "BuyBitcoinRequest")) }
                val res = getBindingLiquidSdk().buyBitcoin(buyBitcoinRequest)
                promise.resolve(res)
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun listPayments(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val listPaymentsRequest =
                    asListPaymentsRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "ListPaymentsRequest")) }
                val res = getBindingLiquidSdk().listPayments(listPaymentsRequest)
                promise.resolve(readableArrayOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun listRefundables(promise: Promise) {
        executor.execute {
            try {
                val res = getBindingLiquidSdk().listRefundables()
                promise.resolve(readableArrayOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun prepareRefund(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val prepareRefundRequest =
                    asPrepareRefundRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "PrepareRefundRequest")) }
                val res = getBindingLiquidSdk().prepareRefund(prepareRefundRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun refund(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val refundRequest =
                    asRefundRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "RefundRequest")) }
                val res = getBindingLiquidSdk().refund(refundRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun rescanOnchainSwaps(promise: Promise) {
        executor.execute {
            try {
                getBindingLiquidSdk().rescanOnchainSwaps()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun sync(promise: Promise) {
        executor.execute {
            try {
                getBindingLiquidSdk().sync()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun recommendedFees(promise: Promise) {
        executor.execute {
            try {
                val res = getBindingLiquidSdk().recommendedFees()
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun backup(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val backupRequest =
                    asBackupRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "BackupRequest")) }
                getBindingLiquidSdk().backup(backupRequest)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun restore(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val restoreRequest =
                    asRestoreRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "RestoreRequest")) }
                getBindingLiquidSdk().restore(restoreRequest)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun disconnect(promise: Promise) {
        executor.execute {
            try {
                getBindingLiquidSdk().disconnect()
                bindingLiquidSdk = null
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun lnurlPay(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val lnUrlPayRequest =
                    asLnUrlPayRequest(req) ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "LnUrlPayRequest")) }
                val res = getBindingLiquidSdk().lnurlPay(lnUrlPayRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun lnurlWithdraw(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val lnUrlWithdrawRequest =
                    asLnUrlWithdrawRequest(req)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("req", "LnUrlWithdrawRequest")) }
                val res = getBindingLiquidSdk().lnurlWithdraw(lnUrlWithdrawRequest)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun lnurlAuth(
        reqData: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val lnUrlAuthRequestData =
                    asLnUrlAuthRequestData(reqData)
                        ?: run { throw SdkException.Generic(errMissingMandatoryField("reqData", "LnUrlAuthRequestData")) }
                val res = getBindingLiquidSdk().lnurlAuth(lnUrlAuthRequestData)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun registerWebhook(
        webhookUrl: String,
        promise: Promise,
    ) {
        executor.execute {
            try {
                getBindingLiquidSdk().registerWebhook(webhookUrl)
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun unregisterWebhook(promise: Promise) {
        executor.execute {
            try {
                getBindingLiquidSdk().unregisterWebhook()
                promise.resolve(readableMapOf("status" to "ok"))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun fetchFiatRates(promise: Promise) {
        executor.execute {
            try {
                val res = getBindingLiquidSdk().fetchFiatRates()
                promise.resolve(readableArrayOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun listFiatCurrencies(promise: Promise) {
        executor.execute {
            try {
                val res = getBindingLiquidSdk().listFiatCurrencies()
                promise.resolve(readableArrayOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }
}

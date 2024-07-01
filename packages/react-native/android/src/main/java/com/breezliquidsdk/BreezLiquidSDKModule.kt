package com.breezliquidsdk

import breez_liquid_sdk.*
import com.facebook.react.bridge.*
import com.facebook.react.modules.core.DeviceEventManagerModule.RCTDeviceEventEmitter
import java.io.File
import java.util.*
import java.util.concurrent.ExecutorService
import java.util.concurrent.Executors

class BreezLiquidSDKModule(
    reactContext: ReactApplicationContext,
) : ReactContextBaseJavaModule(reactContext) {
    private lateinit var executor: ExecutorService
    private var bindingLiquidSdk: BindingLiquidSdk? = null

    companion object {
        const val TAG = "RNBreezLiquidSDK"
    }

    override fun initialize() {
        super.initialize()

        executor = Executors.newFixedThreadPool(3)
    }

    override fun getName(): String = TAG

    @Throws(LiquidSdkException::class)
    fun getBindingLiquidSdk(): BindingLiquidSdk {
        if (bindingLiquidSdk != null) {
            return bindingLiquidSdk!!
        }

        throw LiquidSdkException.Generic("Not initialized")
    }

    @Throws(LiquidSdkException::class)
    private fun ensureWorkingDir(workingDir: String) {
        try {
            val workingDirFile = File(workingDir)

            if (!workingDirFile.exists() && !workingDirFile.mkdirs()) {
                throw LiquidSdkException.Generic("Mandatory field workingDir must contain a writable directory")
            }
        } catch (e: SecurityException) {
            throw LiquidSdkException.Generic("Mandatory field workingDir must contain a writable directory")
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
                val workingDir = File(reactApplicationContext.filesDir.toString() + "/breezLiquidSdk")

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

                setLogger(BreezLiquidSDKLogger(emitter))
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
                    asConnectRequest(req) ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "ConnectRequest")) }

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
                var eventListener = BreezLiquidSDKEventListener(emitter)
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
                    asPrepareSendRequest(req)
                        ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PrepareSendRequest")) }
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
                    asPrepareSendResponse(req)
                        ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PrepareSendResponse")) }
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
                val prepareReceiveRequest =
                    asPrepareReceiveRequest(req)
                        ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PrepareReceiveRequest")) }
                val res = getBindingLiquidSdk().prepareReceivePayment(prepareReceiveRequest)
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
                val prepareReceiveResponse =
                    asPrepareReceiveResponse(req)
                        ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PrepareReceiveResponse")) }
                val res = getBindingLiquidSdk().receivePayment(prepareReceiveResponse)
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
                        ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PreparePayOnchainRequest")) }
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
                    asPayOnchainRequest(req)
                        ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PayOnchainRequest")) }
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
                        ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PrepareReceiveOnchainRequest")) }
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
                        ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PrepareReceiveOnchainResponse")) }
                val res = getBindingLiquidSdk().receiveOnchain(prepareReceiveOnchainResponse)
                promise.resolve(readableMapOf(res))
            } catch (e: Exception) {
                promise.reject(e.javaClass.simpleName.replace("Exception", "Error"), e.message, e)
            }
        }
    }

    @ReactMethod
    fun listPayments(promise: Promise) {
        executor.execute {
            try {
                val res = getBindingLiquidSdk().listPayments()
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
                        ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "PrepareRefundRequest")) }
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
                    asRefundRequest(req) ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "RefundRequest")) }
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
    fun backup(
        req: ReadableMap,
        promise: Promise,
    ) {
        executor.execute {
            try {
                val backupRequest =
                    asBackupRequest(req) ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "BackupRequest")) }
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
                    asRestoreRequest(req) ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "RestoreRequest")) }
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
                    asLnUrlPayRequest(req) ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "LnUrlPayRequest")) }
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
                        ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("req", "LnUrlWithdrawRequest")) }
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
                        ?: run { throw LiquidSdkException.Generic(errMissingMandatoryField("reqData", "LnUrlAuthRequestData")) }
                val res = getBindingLiquidSdk().lnurlAuth(lnUrlAuthRequestData)
                promise.resolve(readableMapOf(res))
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

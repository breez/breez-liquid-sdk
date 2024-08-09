package breez_sdk_liquid_notification

import android.app.Service
import android.content.Intent
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import breez_sdk_liquid.BindingLiquidSdk
import breez_sdk_liquid.ConnectRequest
import breez_sdk_liquid.EventListener
import breez_sdk_liquid.Logger
import breez_sdk_liquid.SdkEvent
import breez_sdk_liquid_notification.BreezSdkLiquidConnector.Companion.connectSDK
import breez_sdk_liquid_notification.Constants.MESSAGE_TYPE_SWAP_UPDATED
import breez_sdk_liquid_notification.Constants.MESSAGE_TYPE_LNURL_PAY_INFO
import breez_sdk_liquid_notification.Constants.MESSAGE_TYPE_LNURL_PAY_INVOICE
import breez_sdk_liquid_notification.Constants.NOTIFICATION_ID_FOREGROUND_SERVICE
import breez_sdk_liquid_notification.Constants.SERVICE_TIMEOUT_MS
import breez_sdk_liquid_notification.Constants.SHUTDOWN_DELAY_MS
import breez_sdk_liquid_notification.NotificationHelper.Companion.notifyForegroundService
import breez_sdk_liquid_notification.job.Job
import breez_sdk_liquid_notification.job.LnurlPayInfoJob
import breez_sdk_liquid_notification.job.LnurlPayInvoiceJob
import breez_sdk_liquid_notification.job.SwapUpdatedJob
import kotlinx.coroutines.CoroutineExceptionHandler
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch

interface SdkForegroundService {
    fun onFinished(job: Job)
}

abstract class ForegroundService : SdkForegroundService, EventListener, Service() {
    private var liquidSDK: BindingLiquidSdk? = null
    @Suppress("MemberVisibilityCanBePrivate")
    val serviceScope = CoroutineScope(Dispatchers.Main.immediate + SupervisorJob())
    protected var logger: ServiceLogger = ServiceLogger()
    private var jobs: MutableList<Job> = arrayListOf()

    companion object {
        private const val TAG = "ForegroundService"
    }

    // =========================================================== //
    //                      SERVICE LIFECYCLE                      //
    // =========================================================== //

    override fun onBind(intent: Intent): IBinder? {
        return null
    }

    /** Called by a Job to signal that it is complete. */
    override fun onFinished(job: Job) {
        synchronized(this) {
            logger.log(TAG, "Job has finished: $job", "DEBUG")
            jobs.remove(job)
            delayedShutdown()
        }
    }

    /** Stop the service */
    private val serviceTimeoutHandler = Handler(Looper.getMainLooper())
    private val serviceTimeoutRunnable: Runnable = Runnable {
        logger.log(TAG, "Reached service timeout...", "DEBUG")
        synchronized(this) {
            jobs.forEach { job -> job.onShutdown() }
        }

        shutdown()
    }

    private val shutdownHandler = Handler(Looper.getMainLooper())
    private val shutdownRunnable: Runnable = Runnable {
        logger.log(TAG, "Reached scheduled shutdown...", "DEBUG")
        shutdown()
    }

    private fun resetDelayedCallbacks() {
        serviceTimeoutHandler.removeCallbacksAndMessages(null)
        shutdownHandler.removeCallbacksAndMessages(null)

        shutdownHandler.postDelayed(serviceTimeoutRunnable, SERVICE_TIMEOUT_MS)
    }

    private fun delayedShutdown() {
        if (jobs.isEmpty()) {
            shutdownHandler.postDelayed(shutdownRunnable, SHUTDOWN_DELAY_MS)
        }
    }

    open fun shutdown() {
        logger.log(TAG, "Shutting down foreground service", "DEBUG")
        stopForeground(STOP_FOREGROUND_REMOVE)
        stopSelf()
    }

    /** Called when an intent is called for this service. */
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        super.onStartCommand(intent, flags, startId)
        resetDelayedCallbacks()

        val intentDetails = "[ intent=$intent, flag=$flags, startId=$startId ]"
        logger.log(TAG, "Start foreground service from intent $intentDetails", "DEBUG")

        // Display foreground service notification
        val notification = notifyForegroundService(applicationContext)
        startForeground(NOTIFICATION_ID_FOREGROUND_SERVICE, notification)

        // Connect to SDK if source intent has data message with valid payload
        getConnectRequest()?.let { connectRequest ->
            getJobFromIntent(intent)?.also { job ->
                launchSdkConnection(connectRequest, job)
            } ?: run {
                logger.log(TAG, "Received invalid data message", "WARN")
                delayedShutdown()
            }
        } ?: run {
            logger.log(TAG, "Missing ConnectRequest", "WARN")
            delayedShutdown()
        }

        return START_NOT_STICKY
    }

    /** To be implemented by the application foreground service.
     *  It should retrieve the Breez API key and node mnemonic then construct
     *  a ConnectRequest to be used to call the Breez Liquid SDK connect function. */
    abstract fun getConnectRequest(): ConnectRequest?

    /** Get the job to be executed from the Message data in the Intent.
     *  This can be overridden to handle custom jobs. */
    open fun getJobFromIntent(intent: Intent?): Job? {
        return Message.createFromIntent(intent)?.let { message ->
            message.payload?.let { payload ->
                when (message.type) {
                    MESSAGE_TYPE_SWAP_UPDATED -> SwapUpdatedJob(
                        applicationContext,
                        this,
                        payload,
                        logger
                    )

                    MESSAGE_TYPE_LNURL_PAY_INFO -> LnurlPayInfoJob(
                        applicationContext,
                        this,
                        payload,
                        logger
                    )

                    MESSAGE_TYPE_LNURL_PAY_INVOICE -> LnurlPayInvoiceJob(
                        applicationContext,
                        this,
                        payload,
                        logger
                    )

                    else -> null
                }
            }
        }
    }

    private fun launchSdkConnection(connectRequest: ConnectRequest, job: Job) {
        val sdkListener = this
        serviceScope.launch(Dispatchers.IO + CoroutineExceptionHandler { _, e ->
            logger.log(TAG, "Breez Liquid SDK connection failed $e", "ERROR")
            delayedShutdown()
        }) {
            liquidSDK ?: run {
                liquidSDK = connectSDK(connectRequest, sdkListener, logger)
            }

            liquidSDK?.let {
                jobs.add(job)
                job.start(liquidSDK!!)
            }
        }
    }

    /** Handles incoming events from the Breez Liquid SDK EventListener */
    override fun onEvent(e: SdkEvent) {
        synchronized(this) {
            jobs.forEach { job -> job.onEvent(e) }
        }
    }

    @Suppress("unused")
    fun setLogger(logger: Logger) {
        this.logger = ServiceLogger(logger)
    }
}

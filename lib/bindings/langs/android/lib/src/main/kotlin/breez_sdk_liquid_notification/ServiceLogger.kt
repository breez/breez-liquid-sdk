package breez_sdk_liquid_notification

import android.util.Log
import breez_sdk_liquid.LogEntry
import breez_sdk_liquid.Logger

class ServiceLogger(private val logger: Logger?) {
    constructor() : this(null)

    fun log(tag: String, message: String, level: String) {
        logger?.log(LogEntry(message, level)) ?: when (level) {
            "ERROR" -> Log.e(tag, message)
            "WARN" -> Log.w(tag, message)
            "INFO" -> Log.i(tag, message)
            "DEBUG" -> Log.d(tag, message)
            "TRACE" -> Log.v(tag, message)
            else -> {}
        }
    }
}
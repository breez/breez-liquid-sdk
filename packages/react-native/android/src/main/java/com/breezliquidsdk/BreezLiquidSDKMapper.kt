package com.breezliquidsdk
import breez_liquid_sdk.*
import com.facebook.react.bridge.*
import java.util.*

fun asConnectRequest(connectRequest: ReadableMap): ConnectRequest? {
    if (!validateMandatoryFields(
            connectRequest,
            arrayOf(
                "mnemonic",
                "network",
            ),
        )
    ) {
        return null
    }
    val mnemonic = connectRequest.getString("mnemonic")!!
    val network = connectRequest.getString("network")?.let { asNetwork(it) }!!
    val dataDir = if (hasNonNullKey(connectRequest, "dataDir")) connectRequest.getString("dataDir") else null
    return ConnectRequest(
        mnemonic,
        network,
        dataDir,
    )
}

fun readableMapOf(connectRequest: ConnectRequest): ReadableMap {
    return readableMapOf(
        "mnemonic" to connectRequest.mnemonic,
        "network" to connectRequest.network.name.lowercase(),
        "dataDir" to connectRequest.dataDir,
    )
}

fun asConnectRequestList(arr: ReadableArray): List<ConnectRequest> {
    val list = ArrayList<ConnectRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asConnectRequest(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asGetInfoRequest(getInfoRequest: ReadableMap): GetInfoRequest? {
    if (!validateMandatoryFields(
            getInfoRequest,
            arrayOf(
                "withScan",
            ),
        )
    ) {
        return null
    }
    val withScan = getInfoRequest.getBoolean("withScan")
    return GetInfoRequest(
        withScan,
    )
}

fun readableMapOf(getInfoRequest: GetInfoRequest): ReadableMap {
    return readableMapOf(
        "withScan" to getInfoRequest.withScan,
    )
}

fun asGetInfoRequestList(arr: ReadableArray): List<GetInfoRequest> {
    val list = ArrayList<GetInfoRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asGetInfoRequest(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asGetInfoResponse(getInfoResponse: ReadableMap): GetInfoResponse? {
    if (!validateMandatoryFields(
            getInfoResponse,
            arrayOf(
                "balanceSat",
                "pubkey",
            ),
        )
    ) {
        return null
    }
    val balanceSat = getInfoResponse.getDouble("balanceSat").toULong()
    val pubkey = getInfoResponse.getString("pubkey")!!
    return GetInfoResponse(
        balanceSat,
        pubkey,
    )
}

fun readableMapOf(getInfoResponse: GetInfoResponse): ReadableMap {
    return readableMapOf(
        "balanceSat" to getInfoResponse.balanceSat,
        "pubkey" to getInfoResponse.pubkey,
    )
}

fun asGetInfoResponseList(arr: ReadableArray): List<GetInfoResponse> {
    val list = ArrayList<GetInfoResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asGetInfoResponse(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareReceiveRequest(prepareReceiveRequest: ReadableMap): PrepareReceiveRequest? {
    if (!validateMandatoryFields(
            prepareReceiveRequest,
            arrayOf(
                "payerAmountSat",
            ),
        )
    ) {
        return null
    }
    val payerAmountSat = prepareReceiveRequest.getDouble("payerAmountSat").toULong()
    return PrepareReceiveRequest(
        payerAmountSat,
    )
}

fun readableMapOf(prepareReceiveRequest: PrepareReceiveRequest): ReadableMap {
    return readableMapOf(
        "payerAmountSat" to prepareReceiveRequest.payerAmountSat,
    )
}

fun asPrepareReceiveRequestList(arr: ReadableArray): List<PrepareReceiveRequest> {
    val list = ArrayList<PrepareReceiveRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareReceiveRequest(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareReceiveResponse(prepareReceiveResponse: ReadableMap): PrepareReceiveResponse? {
    if (!validateMandatoryFields(
            prepareReceiveResponse,
            arrayOf(
                "pairHash",
                "payerAmountSat",
                "feesSat",
            ),
        )
    ) {
        return null
    }
    val pairHash = prepareReceiveResponse.getString("pairHash")!!
    val payerAmountSat = prepareReceiveResponse.getDouble("payerAmountSat").toULong()
    val feesSat = prepareReceiveResponse.getDouble("feesSat").toULong()
    return PrepareReceiveResponse(
        pairHash,
        payerAmountSat,
        feesSat,
    )
}

fun readableMapOf(prepareReceiveResponse: PrepareReceiveResponse): ReadableMap {
    return readableMapOf(
        "pairHash" to prepareReceiveResponse.pairHash,
        "payerAmountSat" to prepareReceiveResponse.payerAmountSat,
        "feesSat" to prepareReceiveResponse.feesSat,
    )
}

fun asPrepareReceiveResponseList(arr: ReadableArray): List<PrepareReceiveResponse> {
    val list = ArrayList<PrepareReceiveResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareReceiveResponse(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareSendRequest(prepareSendRequest: ReadableMap): PrepareSendRequest? {
    if (!validateMandatoryFields(
            prepareSendRequest,
            arrayOf(
                "invoice",
            ),
        )
    ) {
        return null
    }
    val invoice = prepareSendRequest.getString("invoice")!!
    return PrepareSendRequest(
        invoice,
    )
}

fun readableMapOf(prepareSendRequest: PrepareSendRequest): ReadableMap {
    return readableMapOf(
        "invoice" to prepareSendRequest.invoice,
    )
}

fun asPrepareSendRequestList(arr: ReadableArray): List<PrepareSendRequest> {
    val list = ArrayList<PrepareSendRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareSendRequest(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareSendResponse(prepareSendResponse: ReadableMap): PrepareSendResponse? {
    if (!validateMandatoryFields(
            prepareSendResponse,
            arrayOf(
                "id",
                "payerAmountSat",
                "receiverAmountSat",
                "totalFees",
                "fundingAddress",
                "invoice",
            ),
        )
    ) {
        return null
    }
    val id = prepareSendResponse.getString("id")!!
    val payerAmountSat = prepareSendResponse.getDouble("payerAmountSat").toULong()
    val receiverAmountSat = prepareSendResponse.getDouble("receiverAmountSat").toULong()
    val totalFees = prepareSendResponse.getDouble("totalFees").toULong()
    val fundingAddress = prepareSendResponse.getString("fundingAddress")!!
    val invoice = prepareSendResponse.getString("invoice")!!
    return PrepareSendResponse(
        id,
        payerAmountSat,
        receiverAmountSat,
        totalFees,
        fundingAddress,
        invoice,
    )
}

fun readableMapOf(prepareSendResponse: PrepareSendResponse): ReadableMap {
    return readableMapOf(
        "id" to prepareSendResponse.id,
        "payerAmountSat" to prepareSendResponse.payerAmountSat,
        "receiverAmountSat" to prepareSendResponse.receiverAmountSat,
        "totalFees" to prepareSendResponse.totalFees,
        "fundingAddress" to prepareSendResponse.fundingAddress,
        "invoice" to prepareSendResponse.invoice,
    )
}

fun asPrepareSendResponseList(arr: ReadableArray): List<PrepareSendResponse> {
    val list = ArrayList<PrepareSendResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareSendResponse(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asReceivePaymentResponse(receivePaymentResponse: ReadableMap): ReceivePaymentResponse? {
    if (!validateMandatoryFields(
            receivePaymentResponse,
            arrayOf(
                "id",
                "invoice",
            ),
        )
    ) {
        return null
    }
    val id = receivePaymentResponse.getString("id")!!
    val invoice = receivePaymentResponse.getString("invoice")!!
    return ReceivePaymentResponse(
        id,
        invoice,
    )
}

fun readableMapOf(receivePaymentResponse: ReceivePaymentResponse): ReadableMap {
    return readableMapOf(
        "id" to receivePaymentResponse.id,
        "invoice" to receivePaymentResponse.invoice,
    )
}

fun asReceivePaymentResponseList(arr: ReadableArray): List<ReceivePaymentResponse> {
    val list = ArrayList<ReceivePaymentResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asReceivePaymentResponse(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asRestoreRequest(restoreRequest: ReadableMap): RestoreRequest? {
    if (!validateMandatoryFields(
            restoreRequest,
            arrayOf(),
        )
    ) {
        return null
    }
    val backupPath = if (hasNonNullKey(restoreRequest, "backupPath")) restoreRequest.getString("backupPath") else null
    return RestoreRequest(
        backupPath,
    )
}

fun readableMapOf(restoreRequest: RestoreRequest): ReadableMap {
    return readableMapOf(
        "backupPath" to restoreRequest.backupPath,
    )
}

fun asRestoreRequestList(arr: ReadableArray): List<RestoreRequest> {
    val list = ArrayList<RestoreRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRestoreRequest(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asSendPaymentResponse(sendPaymentResponse: ReadableMap): SendPaymentResponse? {
    if (!validateMandatoryFields(
            sendPaymentResponse,
            arrayOf(
                "txid",
            ),
        )
    ) {
        return null
    }
    val txid = sendPaymentResponse.getString("txid")!!
    return SendPaymentResponse(
        txid,
    )
}

fun readableMapOf(sendPaymentResponse: SendPaymentResponse): ReadableMap {
    return readableMapOf(
        "txid" to sendPaymentResponse.txid,
    )
}

fun asSendPaymentResponseList(arr: ReadableArray): List<SendPaymentResponse> {
    val list = ArrayList<SendPaymentResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSendPaymentResponse(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asNetwork(type: String): Network {
    return Network.valueOf(type.uppercase())
}

fun asNetworkList(arr: ReadableArray): List<Network> {
    val list = ArrayList<Network>()
    for (value in arr.toArrayList()) {
        when (value) {
            is String -> list.add(asNetwork(value)!!)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun readableMapOf(vararg values: Pair<String, *>): ReadableMap {
    val map = Arguments.createMap()
    for ((key, value) in values) {
        pushToMap(map, key, value)
    }
    return map
}

fun hasNonNullKey(
    map: ReadableMap,
    key: String,
): Boolean {
    return map.hasKey(key) && !map.isNull(key)
}

fun validateMandatoryFields(
    map: ReadableMap,
    keys: Array<String>,
): Boolean {
    for (k in keys) {
        if (!hasNonNullKey(map, k)) return false
    }

    return true
}

fun pushToArray(
    array: WritableArray,
    value: Any?,
) {
    when (value) {
        null -> array.pushNull()
        is Array<*> -> array.pushArray(readableArrayOf(value.asIterable()))
        is List<*> -> array.pushArray(readableArrayOf(value))
        else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
    }
}

fun pushToMap(
    map: WritableMap,
    key: String,
    value: Any?,
) {
    when (value) {
        null -> map.putNull(key)
        is Boolean -> map.putBoolean(key, value)
        is Byte -> map.putInt(key, value.toInt())
        is Double -> map.putDouble(key, value)
        is Int -> map.putInt(key, value)
        is Long -> map.putDouble(key, value.toDouble())
        is ReadableArray -> map.putArray(key, value)
        is ReadableMap -> map.putMap(key, value)
        is String -> map.putString(key, value)
        is UByte -> map.putInt(key, value.toInt())
        is UInt -> map.putInt(key, value.toInt())
        is UShort -> map.putInt(key, value.toInt())
        is ULong -> map.putDouble(key, value.toDouble())
        is Array<*> -> map.putArray(key, readableArrayOf(value.asIterable()))
        is List<*> -> map.putArray(key, readableArrayOf(value))
        else -> throw LiquidSdkException.Generic("Unexpected type ${value::class.java.name} for key [$key]")
    }
}

fun readableArrayOf(values: Iterable<*>?): ReadableArray {
    val array = Arguments.createArray()
    if (values != null) {
        for (value in values) {
            pushToArray(array, value)
        }
    }

    return array
}

fun asUByteList(arr: ReadableArray): List<UByte> {
    val list = ArrayList<UByte>()
    for (value in arr.toArrayList()) {
        when (value) {
            is Double -> list.add(value.toInt().toUByte())
            is Int -> list.add(value.toUByte())
            is UByte -> list.add(value)
            else -> throw LiquidSdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asStringList(arr: ReadableArray): List<String> {
    val list = ArrayList<String>()
    for (value in arr.toArrayList()) {
        list.add(value.toString())
    }
    return list
}

fun errMissingMandatoryField(
    fieldName: String,
    typeName: String,
): String {
    return "Missing mandatory field $fieldName for type $typeName"
}

fun errUnexpectedType(typeName: String): String {
    return "Unexpected type $typeName"
}

fun errUnexpectedValue(fieldName: String): String {
    return "Unexpected value for optional field $fieldName"
}

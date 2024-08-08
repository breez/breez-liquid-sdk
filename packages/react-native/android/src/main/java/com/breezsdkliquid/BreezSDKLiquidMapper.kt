package com.breezsdkliquid
import breez_sdk_liquid.*
import com.facebook.react.bridge.*
import java.util.*

fun asAesSuccessActionDataDecrypted(aesSuccessActionDataDecrypted: ReadableMap): AesSuccessActionDataDecrypted? {
    if (!validateMandatoryFields(
            aesSuccessActionDataDecrypted,
            arrayOf(
                "description",
                "plaintext",
            ),
        )
    ) {
        return null
    }
    val description = aesSuccessActionDataDecrypted.getString("description")!!
    val plaintext = aesSuccessActionDataDecrypted.getString("plaintext")!!
    return AesSuccessActionDataDecrypted(
        description,
        plaintext,
    )
}

fun readableMapOf(aesSuccessActionDataDecrypted: AesSuccessActionDataDecrypted): ReadableMap =
    readableMapOf(
        "description" to aesSuccessActionDataDecrypted.description,
        "plaintext" to aesSuccessActionDataDecrypted.plaintext,
    )

fun asAesSuccessActionDataDecryptedList(arr: ReadableArray): List<AesSuccessActionDataDecrypted> {
    val list = ArrayList<AesSuccessActionDataDecrypted>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asAesSuccessActionDataDecrypted(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asBackupRequest(backupRequest: ReadableMap): BackupRequest? {
    if (!validateMandatoryFields(
            backupRequest,
            arrayOf(),
        )
    ) {
        return null
    }
    val backupPath = if (hasNonNullKey(backupRequest, "backupPath")) backupRequest.getString("backupPath") else null
    return BackupRequest(
        backupPath,
    )
}

fun readableMapOf(backupRequest: BackupRequest): ReadableMap =
    readableMapOf(
        "backupPath" to backupRequest.backupPath,
    )

fun asBackupRequestList(arr: ReadableArray): List<BackupRequest> {
    val list = ArrayList<BackupRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asBackupRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asBitcoinAddressData(bitcoinAddressData: ReadableMap): BitcoinAddressData? {
    if (!validateMandatoryFields(
            bitcoinAddressData,
            arrayOf(
                "address",
                "network",
            ),
        )
    ) {
        return null
    }
    val address = bitcoinAddressData.getString("address")!!
    val network = bitcoinAddressData.getString("network")?.let { asNetwork(it) }!!
    val amountSat = if (hasNonNullKey(bitcoinAddressData, "amountSat")) bitcoinAddressData.getDouble("amountSat").toULong() else null
    val label = if (hasNonNullKey(bitcoinAddressData, "label")) bitcoinAddressData.getString("label") else null
    val message = if (hasNonNullKey(bitcoinAddressData, "message")) bitcoinAddressData.getString("message") else null
    return BitcoinAddressData(
        address,
        network,
        amountSat,
        label,
        message,
    )
}

fun readableMapOf(bitcoinAddressData: BitcoinAddressData): ReadableMap =
    readableMapOf(
        "address" to bitcoinAddressData.address,
        "network" to bitcoinAddressData.network.name.lowercase(),
        "amountSat" to bitcoinAddressData.amountSat,
        "label" to bitcoinAddressData.label,
        "message" to bitcoinAddressData.message,
    )

fun asBitcoinAddressDataList(arr: ReadableArray): List<BitcoinAddressData> {
    val list = ArrayList<BitcoinAddressData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asBitcoinAddressData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asBuyBitcoinRequest(buyBitcoinRequest: ReadableMap): BuyBitcoinRequest? {
    if (!validateMandatoryFields(
            buyBitcoinRequest,
            arrayOf(
                "prepareResponse",
            ),
        )
    ) {
        return null
    }
    val prepareResponse = buyBitcoinRequest.getMap("prepareResponse")?.let { asPrepareBuyBitcoinResponse(it) }!!
    val redirectUrl = if (hasNonNullKey(buyBitcoinRequest, "redirectUrl")) buyBitcoinRequest.getString("redirectUrl") else null
    return BuyBitcoinRequest(
        prepareResponse,
        redirectUrl,
    )
}

fun readableMapOf(buyBitcoinRequest: BuyBitcoinRequest): ReadableMap =
    readableMapOf(
        "prepareResponse" to readableMapOf(buyBitcoinRequest.prepareResponse),
        "redirectUrl" to buyBitcoinRequest.redirectUrl,
    )

fun asBuyBitcoinRequestList(arr: ReadableArray): List<BuyBitcoinRequest> {
    val list = ArrayList<BuyBitcoinRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asBuyBitcoinRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asConfig(config: ReadableMap): Config? {
    if (!validateMandatoryFields(
            config,
            arrayOf(
                "liquidElectrumUrl",
                "bitcoinElectrumUrl",
                "mempoolspaceUrl",
                "workingDir",
                "network",
                "paymentTimeoutSec",
                "zeroConfMinFeeRateMsat",
            ),
        )
    ) {
        return null
    }
    val liquidElectrumUrl = config.getString("liquidElectrumUrl")!!
    val bitcoinElectrumUrl = config.getString("bitcoinElectrumUrl")!!
    val mempoolspaceUrl = config.getString("mempoolspaceUrl")!!
    val workingDir = config.getString("workingDir")!!
    val network = config.getString("network")?.let { asLiquidNetwork(it) }!!
    val paymentTimeoutSec = config.getDouble("paymentTimeoutSec").toULong()
    val zeroConfMinFeeRateMsat = config.getInt("zeroConfMinFeeRateMsat").toUInt()
    val zeroConfMaxAmountSat =
        if (hasNonNullKey(
                config,
                "zeroConfMaxAmountSat",
            )
        ) {
            config.getDouble("zeroConfMaxAmountSat").toULong()
        } else {
            null
        }
    return Config(
        liquidElectrumUrl,
        bitcoinElectrumUrl,
        mempoolspaceUrl,
        workingDir,
        network,
        paymentTimeoutSec,
        zeroConfMinFeeRateMsat,
        zeroConfMaxAmountSat,
    )
}

fun readableMapOf(config: Config): ReadableMap =
    readableMapOf(
        "liquidElectrumUrl" to config.liquidElectrumUrl,
        "bitcoinElectrumUrl" to config.bitcoinElectrumUrl,
        "mempoolspaceUrl" to config.mempoolspaceUrl,
        "workingDir" to config.workingDir,
        "network" to config.network.name.lowercase(),
        "paymentTimeoutSec" to config.paymentTimeoutSec,
        "zeroConfMinFeeRateMsat" to config.zeroConfMinFeeRateMsat,
        "zeroConfMaxAmountSat" to config.zeroConfMaxAmountSat,
    )

fun asConfigList(arr: ReadableArray): List<Config> {
    val list = ArrayList<Config>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asConfig(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asConnectRequest(connectRequest: ReadableMap): ConnectRequest? {
    if (!validateMandatoryFields(
            connectRequest,
            arrayOf(
                "config",
                "mnemonic",
            ),
        )
    ) {
        return null
    }
    val config = connectRequest.getMap("config")?.let { asConfig(it) }!!
    val mnemonic = connectRequest.getString("mnemonic")!!
    return ConnectRequest(
        config,
        mnemonic,
    )
}

fun readableMapOf(connectRequest: ConnectRequest): ReadableMap =
    readableMapOf(
        "config" to readableMapOf(connectRequest.config),
        "mnemonic" to connectRequest.mnemonic,
    )

fun asConnectRequestList(arr: ReadableArray): List<ConnectRequest> {
    val list = ArrayList<ConnectRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asConnectRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asCurrencyInfo(currencyInfo: ReadableMap): CurrencyInfo? {
    if (!validateMandatoryFields(
            currencyInfo,
            arrayOf(
                "name",
                "fractionSize",
                "localizedName",
                "localeOverrides",
            ),
        )
    ) {
        return null
    }
    val name = currencyInfo.getString("name")!!
    val fractionSize = currencyInfo.getInt("fractionSize").toUInt()
    val spacing = if (hasNonNullKey(currencyInfo, "spacing")) currencyInfo.getInt("spacing").toUInt() else null
    val symbol = if (hasNonNullKey(currencyInfo, "symbol")) currencyInfo.getMap("symbol")?.let { asSymbol(it) } else null
    val uniqSymbol = if (hasNonNullKey(currencyInfo, "uniqSymbol")) currencyInfo.getMap("uniqSymbol")?.let { asSymbol(it) } else null
    val localizedName = currencyInfo.getArray("localizedName")?.let { asLocalizedNameList(it) }!!
    val localeOverrides = currencyInfo.getArray("localeOverrides")?.let { asLocaleOverridesList(it) }!!
    return CurrencyInfo(
        name,
        fractionSize,
        spacing,
        symbol,
        uniqSymbol,
        localizedName,
        localeOverrides,
    )
}

fun readableMapOf(currencyInfo: CurrencyInfo): ReadableMap =
    readableMapOf(
        "name" to currencyInfo.name,
        "fractionSize" to currencyInfo.fractionSize,
        "spacing" to currencyInfo.spacing,
        "symbol" to currencyInfo.symbol?.let { readableMapOf(it) },
        "uniqSymbol" to currencyInfo.uniqSymbol?.let { readableMapOf(it) },
        "localizedName" to readableArrayOf(currencyInfo.localizedName),
        "localeOverrides" to readableArrayOf(currencyInfo.localeOverrides),
    )

fun asCurrencyInfoList(arr: ReadableArray): List<CurrencyInfo> {
    val list = ArrayList<CurrencyInfo>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asCurrencyInfo(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asFiatCurrency(fiatCurrency: ReadableMap): FiatCurrency? {
    if (!validateMandatoryFields(
            fiatCurrency,
            arrayOf(
                "id",
                "info",
            ),
        )
    ) {
        return null
    }
    val id = fiatCurrency.getString("id")!!
    val info = fiatCurrency.getMap("info")?.let { asCurrencyInfo(it) }!!
    return FiatCurrency(
        id,
        info,
    )
}

fun readableMapOf(fiatCurrency: FiatCurrency): ReadableMap =
    readableMapOf(
        "id" to fiatCurrency.id,
        "info" to readableMapOf(fiatCurrency.info),
    )

fun asFiatCurrencyList(arr: ReadableArray): List<FiatCurrency> {
    val list = ArrayList<FiatCurrency>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asFiatCurrency(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asGetInfoResponse(getInfoResponse: ReadableMap): GetInfoResponse? {
    if (!validateMandatoryFields(
            getInfoResponse,
            arrayOf(
                "balanceSat",
                "pendingSendSat",
                "pendingReceiveSat",
                "pubkey",
            ),
        )
    ) {
        return null
    }
    val balanceSat = getInfoResponse.getDouble("balanceSat").toULong()
    val pendingSendSat = getInfoResponse.getDouble("pendingSendSat").toULong()
    val pendingReceiveSat = getInfoResponse.getDouble("pendingReceiveSat").toULong()
    val pubkey = getInfoResponse.getString("pubkey")!!
    return GetInfoResponse(
        balanceSat,
        pendingSendSat,
        pendingReceiveSat,
        pubkey,
    )
}

fun readableMapOf(getInfoResponse: GetInfoResponse): ReadableMap =
    readableMapOf(
        "balanceSat" to getInfoResponse.balanceSat,
        "pendingSendSat" to getInfoResponse.pendingSendSat,
        "pendingReceiveSat" to getInfoResponse.pendingReceiveSat,
        "pubkey" to getInfoResponse.pubkey,
    )

fun asGetInfoResponseList(arr: ReadableArray): List<GetInfoResponse> {
    val list = ArrayList<GetInfoResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asGetInfoResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnInvoice(lnInvoice: ReadableMap): LnInvoice? {
    if (!validateMandatoryFields(
            lnInvoice,
            arrayOf(
                "bolt11",
                "network",
                "payeePubkey",
                "paymentHash",
                "timestamp",
                "expiry",
                "routingHints",
                "paymentSecret",
                "minFinalCltvExpiryDelta",
            ),
        )
    ) {
        return null
    }
    val bolt11 = lnInvoice.getString("bolt11")!!
    val network = lnInvoice.getString("network")?.let { asNetwork(it) }!!
    val payeePubkey = lnInvoice.getString("payeePubkey")!!
    val paymentHash = lnInvoice.getString("paymentHash")!!
    val description = if (hasNonNullKey(lnInvoice, "description")) lnInvoice.getString("description") else null
    val descriptionHash = if (hasNonNullKey(lnInvoice, "descriptionHash")) lnInvoice.getString("descriptionHash") else null
    val amountMsat = if (hasNonNullKey(lnInvoice, "amountMsat")) lnInvoice.getDouble("amountMsat").toULong() else null
    val timestamp = lnInvoice.getDouble("timestamp").toULong()
    val expiry = lnInvoice.getDouble("expiry").toULong()
    val routingHints = lnInvoice.getArray("routingHints")?.let { asRouteHintList(it) }!!
    val paymentSecret = lnInvoice.getArray("paymentSecret")?.let { asUByteList(it) }!!
    val minFinalCltvExpiryDelta = lnInvoice.getDouble("minFinalCltvExpiryDelta").toULong()
    return LnInvoice(
        bolt11,
        network,
        payeePubkey,
        paymentHash,
        description,
        descriptionHash,
        amountMsat,
        timestamp,
        expiry,
        routingHints,
        paymentSecret,
        minFinalCltvExpiryDelta,
    )
}

fun readableMapOf(lnInvoice: LnInvoice): ReadableMap =
    readableMapOf(
        "bolt11" to lnInvoice.bolt11,
        "network" to lnInvoice.network.name.lowercase(),
        "payeePubkey" to lnInvoice.payeePubkey,
        "paymentHash" to lnInvoice.paymentHash,
        "description" to lnInvoice.description,
        "descriptionHash" to lnInvoice.descriptionHash,
        "amountMsat" to lnInvoice.amountMsat,
        "timestamp" to lnInvoice.timestamp,
        "expiry" to lnInvoice.expiry,
        "routingHints" to readableArrayOf(lnInvoice.routingHints),
        "paymentSecret" to readableArrayOf(lnInvoice.paymentSecret),
        "minFinalCltvExpiryDelta" to lnInvoice.minFinalCltvExpiryDelta,
    )

fun asLnInvoiceList(arr: ReadableArray): List<LnInvoice> {
    val list = ArrayList<LnInvoice>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnInvoice(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLightningPaymentLimitsResponse(lightningPaymentLimitsResponse: ReadableMap): LightningPaymentLimitsResponse? {
    if (!validateMandatoryFields(
            lightningPaymentLimitsResponse,
            arrayOf(
                "send",
                "receive",
            ),
        )
    ) {
        return null
    }
    val send = lightningPaymentLimitsResponse.getMap("send")?.let { asLimits(it) }!!
    val receive = lightningPaymentLimitsResponse.getMap("receive")?.let { asLimits(it) }!!
    return LightningPaymentLimitsResponse(
        send,
        receive,
    )
}

fun readableMapOf(lightningPaymentLimitsResponse: LightningPaymentLimitsResponse): ReadableMap =
    readableMapOf(
        "send" to readableMapOf(lightningPaymentLimitsResponse.send),
        "receive" to readableMapOf(lightningPaymentLimitsResponse.receive),
    )

fun asLightningPaymentLimitsResponseList(arr: ReadableArray): List<LightningPaymentLimitsResponse> {
    val list = ArrayList<LightningPaymentLimitsResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLightningPaymentLimitsResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLimits(limits: ReadableMap): Limits? {
    if (!validateMandatoryFields(
            limits,
            arrayOf(
                "minSat",
                "maxSat",
                "maxZeroConfSat",
            ),
        )
    ) {
        return null
    }
    val minSat = limits.getDouble("minSat").toULong()
    val maxSat = limits.getDouble("maxSat").toULong()
    val maxZeroConfSat = limits.getDouble("maxZeroConfSat").toULong()
    return Limits(
        minSat,
        maxSat,
        maxZeroConfSat,
    )
}

fun readableMapOf(limits: Limits): ReadableMap =
    readableMapOf(
        "minSat" to limits.minSat,
        "maxSat" to limits.maxSat,
        "maxZeroConfSat" to limits.maxZeroConfSat,
    )

fun asLimitsList(arr: ReadableArray): List<Limits> {
    val list = ArrayList<Limits>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLimits(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLiquidAddressData(liquidAddressData: ReadableMap): LiquidAddressData? {
    if (!validateMandatoryFields(
            liquidAddressData,
            arrayOf(
                "address",
                "network",
            ),
        )
    ) {
        return null
    }
    val address = liquidAddressData.getString("address")!!
    val network = liquidAddressData.getString("network")?.let { asNetwork(it) }!!
    val assetId = if (hasNonNullKey(liquidAddressData, "assetId")) liquidAddressData.getString("assetId") else null
    val amountSat = if (hasNonNullKey(liquidAddressData, "amountSat")) liquidAddressData.getDouble("amountSat").toULong() else null
    val label = if (hasNonNullKey(liquidAddressData, "label")) liquidAddressData.getString("label") else null
    val message = if (hasNonNullKey(liquidAddressData, "message")) liquidAddressData.getString("message") else null
    return LiquidAddressData(
        address,
        network,
        assetId,
        amountSat,
        label,
        message,
    )
}

fun readableMapOf(liquidAddressData: LiquidAddressData): ReadableMap =
    readableMapOf(
        "address" to liquidAddressData.address,
        "network" to liquidAddressData.network.name.lowercase(),
        "assetId" to liquidAddressData.assetId,
        "amountSat" to liquidAddressData.amountSat,
        "label" to liquidAddressData.label,
        "message" to liquidAddressData.message,
    )

fun asLiquidAddressDataList(arr: ReadableArray): List<LiquidAddressData> {
    val list = ArrayList<LiquidAddressData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLiquidAddressData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asListPaymentsRequest(listPaymentsRequest: ReadableMap): ListPaymentsRequest? {
    if (!validateMandatoryFields(
            listPaymentsRequest,
            arrayOf(),
        )
    ) {
        return null
    }
    val filters =
        if (hasNonNullKey(listPaymentsRequest, "filters")) {
            listPaymentsRequest.getArray("filters")?.let {
                asPaymentTypeList(it)
            }
        } else {
            null
        }
    val fromTimestamp =
        if (hasNonNullKey(
                listPaymentsRequest,
                "fromTimestamp",
            )
        ) {
            listPaymentsRequest.getDouble("fromTimestamp").toLong()
        } else {
            null
        }
    val toTimestamp = if (hasNonNullKey(listPaymentsRequest, "toTimestamp")) listPaymentsRequest.getDouble("toTimestamp").toLong() else null
    val offset = if (hasNonNullKey(listPaymentsRequest, "offset")) listPaymentsRequest.getInt("offset").toUInt() else null
    val limit = if (hasNonNullKey(listPaymentsRequest, "limit")) listPaymentsRequest.getInt("limit").toUInt() else null
    return ListPaymentsRequest(
        filters,
        fromTimestamp,
        toTimestamp,
        offset,
        limit,
    )
}

fun readableMapOf(listPaymentsRequest: ListPaymentsRequest): ReadableMap =
    readableMapOf(
        "filters" to listPaymentsRequest.filters?.let { readableArrayOf(it) },
        "fromTimestamp" to listPaymentsRequest.fromTimestamp,
        "toTimestamp" to listPaymentsRequest.toTimestamp,
        "offset" to listPaymentsRequest.offset,
        "limit" to listPaymentsRequest.limit,
    )

fun asListPaymentsRequestList(arr: ReadableArray): List<ListPaymentsRequest> {
    val list = ArrayList<ListPaymentsRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asListPaymentsRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlAuthRequestData(lnUrlAuthRequestData: ReadableMap): LnUrlAuthRequestData? {
    if (!validateMandatoryFields(
            lnUrlAuthRequestData,
            arrayOf(
                "k1",
                "domain",
                "url",
            ),
        )
    ) {
        return null
    }
    val k1 = lnUrlAuthRequestData.getString("k1")!!
    val domain = lnUrlAuthRequestData.getString("domain")!!
    val url = lnUrlAuthRequestData.getString("url")!!
    val action = if (hasNonNullKey(lnUrlAuthRequestData, "action")) lnUrlAuthRequestData.getString("action") else null
    return LnUrlAuthRequestData(
        k1,
        domain,
        url,
        action,
    )
}

fun readableMapOf(lnUrlAuthRequestData: LnUrlAuthRequestData): ReadableMap =
    readableMapOf(
        "k1" to lnUrlAuthRequestData.k1,
        "domain" to lnUrlAuthRequestData.domain,
        "url" to lnUrlAuthRequestData.url,
        "action" to lnUrlAuthRequestData.action,
    )

fun asLnUrlAuthRequestDataList(arr: ReadableArray): List<LnUrlAuthRequestData> {
    val list = ArrayList<LnUrlAuthRequestData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlAuthRequestData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlErrorData(lnUrlErrorData: ReadableMap): LnUrlErrorData? {
    if (!validateMandatoryFields(
            lnUrlErrorData,
            arrayOf(
                "reason",
            ),
        )
    ) {
        return null
    }
    val reason = lnUrlErrorData.getString("reason")!!
    return LnUrlErrorData(
        reason,
    )
}

fun readableMapOf(lnUrlErrorData: LnUrlErrorData): ReadableMap =
    readableMapOf(
        "reason" to lnUrlErrorData.reason,
    )

fun asLnUrlErrorDataList(arr: ReadableArray): List<LnUrlErrorData> {
    val list = ArrayList<LnUrlErrorData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlErrorData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlPayErrorData(lnUrlPayErrorData: ReadableMap): LnUrlPayErrorData? {
    if (!validateMandatoryFields(
            lnUrlPayErrorData,
            arrayOf(
                "paymentHash",
                "reason",
            ),
        )
    ) {
        return null
    }
    val paymentHash = lnUrlPayErrorData.getString("paymentHash")!!
    val reason = lnUrlPayErrorData.getString("reason")!!
    return LnUrlPayErrorData(
        paymentHash,
        reason,
    )
}

fun readableMapOf(lnUrlPayErrorData: LnUrlPayErrorData): ReadableMap =
    readableMapOf(
        "paymentHash" to lnUrlPayErrorData.paymentHash,
        "reason" to lnUrlPayErrorData.reason,
    )

fun asLnUrlPayErrorDataList(arr: ReadableArray): List<LnUrlPayErrorData> {
    val list = ArrayList<LnUrlPayErrorData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlPayErrorData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlPayRequest(lnUrlPayRequest: ReadableMap): LnUrlPayRequest? {
    if (!validateMandatoryFields(
            lnUrlPayRequest,
            arrayOf(
                "data",
                "amountMsat",
            ),
        )
    ) {
        return null
    }
    val data = lnUrlPayRequest.getMap("data")?.let { asLnUrlPayRequestData(it) }!!
    val amountMsat = lnUrlPayRequest.getDouble("amountMsat").toULong()
    val comment = if (hasNonNullKey(lnUrlPayRequest, "comment")) lnUrlPayRequest.getString("comment") else null
    val paymentLabel = if (hasNonNullKey(lnUrlPayRequest, "paymentLabel")) lnUrlPayRequest.getString("paymentLabel") else null
    val validateSuccessActionUrl =
        if (hasNonNullKey(
                lnUrlPayRequest,
                "validateSuccessActionUrl",
            )
        ) {
            lnUrlPayRequest.getBoolean("validateSuccessActionUrl")
        } else {
            null
        }
    return LnUrlPayRequest(
        data,
        amountMsat,
        comment,
        paymentLabel,
        validateSuccessActionUrl,
    )
}

fun readableMapOf(lnUrlPayRequest: LnUrlPayRequest): ReadableMap =
    readableMapOf(
        "data" to readableMapOf(lnUrlPayRequest.data),
        "amountMsat" to lnUrlPayRequest.amountMsat,
        "comment" to lnUrlPayRequest.comment,
        "paymentLabel" to lnUrlPayRequest.paymentLabel,
        "validateSuccessActionUrl" to lnUrlPayRequest.validateSuccessActionUrl,
    )

fun asLnUrlPayRequestList(arr: ReadableArray): List<LnUrlPayRequest> {
    val list = ArrayList<LnUrlPayRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlPayRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlPayRequestData(lnUrlPayRequestData: ReadableMap): LnUrlPayRequestData? {
    if (!validateMandatoryFields(
            lnUrlPayRequestData,
            arrayOf(
                "callback",
                "minSendable",
                "maxSendable",
                "metadataStr",
                "commentAllowed",
                "domain",
                "allowsNostr",
            ),
        )
    ) {
        return null
    }
    val callback = lnUrlPayRequestData.getString("callback")!!
    val minSendable = lnUrlPayRequestData.getDouble("minSendable").toULong()
    val maxSendable = lnUrlPayRequestData.getDouble("maxSendable").toULong()
    val metadataStr = lnUrlPayRequestData.getString("metadataStr")!!
    val commentAllowed = lnUrlPayRequestData.getInt("commentAllowed").toUShort()
    val domain = lnUrlPayRequestData.getString("domain")!!
    val allowsNostr = lnUrlPayRequestData.getBoolean("allowsNostr")
    val nostrPubkey = if (hasNonNullKey(lnUrlPayRequestData, "nostrPubkey")) lnUrlPayRequestData.getString("nostrPubkey") else null
    val lnAddress = if (hasNonNullKey(lnUrlPayRequestData, "lnAddress")) lnUrlPayRequestData.getString("lnAddress") else null
    return LnUrlPayRequestData(
        callback,
        minSendable,
        maxSendable,
        metadataStr,
        commentAllowed,
        domain,
        allowsNostr,
        nostrPubkey,
        lnAddress,
    )
}

fun readableMapOf(lnUrlPayRequestData: LnUrlPayRequestData): ReadableMap =
    readableMapOf(
        "callback" to lnUrlPayRequestData.callback,
        "minSendable" to lnUrlPayRequestData.minSendable,
        "maxSendable" to lnUrlPayRequestData.maxSendable,
        "metadataStr" to lnUrlPayRequestData.metadataStr,
        "commentAllowed" to lnUrlPayRequestData.commentAllowed,
        "domain" to lnUrlPayRequestData.domain,
        "allowsNostr" to lnUrlPayRequestData.allowsNostr,
        "nostrPubkey" to lnUrlPayRequestData.nostrPubkey,
        "lnAddress" to lnUrlPayRequestData.lnAddress,
    )

fun asLnUrlPayRequestDataList(arr: ReadableArray): List<LnUrlPayRequestData> {
    val list = ArrayList<LnUrlPayRequestData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlPayRequestData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlPaySuccessData(lnUrlPaySuccessData: ReadableMap): LnUrlPaySuccessData? {
    if (!validateMandatoryFields(
            lnUrlPaySuccessData,
            arrayOf(
                "payment",
            ),
        )
    ) {
        return null
    }
    val successAction =
        if (hasNonNullKey(lnUrlPaySuccessData, "successAction")) {
            lnUrlPaySuccessData.getMap("successAction")?.let {
                asSuccessActionProcessed(it)
            }
        } else {
            null
        }
    val payment = lnUrlPaySuccessData.getMap("payment")?.let { asPayment(it) }!!
    return LnUrlPaySuccessData(
        successAction,
        payment,
    )
}

fun readableMapOf(lnUrlPaySuccessData: LnUrlPaySuccessData): ReadableMap =
    readableMapOf(
        "successAction" to lnUrlPaySuccessData.successAction?.let { readableMapOf(it) },
        "payment" to readableMapOf(lnUrlPaySuccessData.payment),
    )

fun asLnUrlPaySuccessDataList(arr: ReadableArray): List<LnUrlPaySuccessData> {
    val list = ArrayList<LnUrlPaySuccessData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlPaySuccessData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlWithdrawRequest(lnUrlWithdrawRequest: ReadableMap): LnUrlWithdrawRequest? {
    if (!validateMandatoryFields(
            lnUrlWithdrawRequest,
            arrayOf(
                "data",
                "amountMsat",
            ),
        )
    ) {
        return null
    }
    val data = lnUrlWithdrawRequest.getMap("data")?.let { asLnUrlWithdrawRequestData(it) }!!
    val amountMsat = lnUrlWithdrawRequest.getDouble("amountMsat").toULong()
    val description = if (hasNonNullKey(lnUrlWithdrawRequest, "description")) lnUrlWithdrawRequest.getString("description") else null
    return LnUrlWithdrawRequest(
        data,
        amountMsat,
        description,
    )
}

fun readableMapOf(lnUrlWithdrawRequest: LnUrlWithdrawRequest): ReadableMap =
    readableMapOf(
        "data" to readableMapOf(lnUrlWithdrawRequest.data),
        "amountMsat" to lnUrlWithdrawRequest.amountMsat,
        "description" to lnUrlWithdrawRequest.description,
    )

fun asLnUrlWithdrawRequestList(arr: ReadableArray): List<LnUrlWithdrawRequest> {
    val list = ArrayList<LnUrlWithdrawRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlWithdrawRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlWithdrawRequestData(lnUrlWithdrawRequestData: ReadableMap): LnUrlWithdrawRequestData? {
    if (!validateMandatoryFields(
            lnUrlWithdrawRequestData,
            arrayOf(
                "callback",
                "k1",
                "defaultDescription",
                "minWithdrawable",
                "maxWithdrawable",
            ),
        )
    ) {
        return null
    }
    val callback = lnUrlWithdrawRequestData.getString("callback")!!
    val k1 = lnUrlWithdrawRequestData.getString("k1")!!
    val defaultDescription = lnUrlWithdrawRequestData.getString("defaultDescription")!!
    val minWithdrawable = lnUrlWithdrawRequestData.getDouble("minWithdrawable").toULong()
    val maxWithdrawable = lnUrlWithdrawRequestData.getDouble("maxWithdrawable").toULong()
    return LnUrlWithdrawRequestData(
        callback,
        k1,
        defaultDescription,
        minWithdrawable,
        maxWithdrawable,
    )
}

fun readableMapOf(lnUrlWithdrawRequestData: LnUrlWithdrawRequestData): ReadableMap =
    readableMapOf(
        "callback" to lnUrlWithdrawRequestData.callback,
        "k1" to lnUrlWithdrawRequestData.k1,
        "defaultDescription" to lnUrlWithdrawRequestData.defaultDescription,
        "minWithdrawable" to lnUrlWithdrawRequestData.minWithdrawable,
        "maxWithdrawable" to lnUrlWithdrawRequestData.maxWithdrawable,
    )

fun asLnUrlWithdrawRequestDataList(arr: ReadableArray): List<LnUrlWithdrawRequestData> {
    val list = ArrayList<LnUrlWithdrawRequestData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlWithdrawRequestData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlWithdrawSuccessData(lnUrlWithdrawSuccessData: ReadableMap): LnUrlWithdrawSuccessData? {
    if (!validateMandatoryFields(
            lnUrlWithdrawSuccessData,
            arrayOf(
                "invoice",
            ),
        )
    ) {
        return null
    }
    val invoice = lnUrlWithdrawSuccessData.getMap("invoice")?.let { asLnInvoice(it) }!!
    return LnUrlWithdrawSuccessData(
        invoice,
    )
}

fun readableMapOf(lnUrlWithdrawSuccessData: LnUrlWithdrawSuccessData): ReadableMap =
    readableMapOf(
        "invoice" to readableMapOf(lnUrlWithdrawSuccessData.invoice),
    )

fun asLnUrlWithdrawSuccessDataList(arr: ReadableArray): List<LnUrlWithdrawSuccessData> {
    val list = ArrayList<LnUrlWithdrawSuccessData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlWithdrawSuccessData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLocaleOverrides(localeOverrides: ReadableMap): LocaleOverrides? {
    if (!validateMandatoryFields(
            localeOverrides,
            arrayOf(
                "locale",
                "symbol",
            ),
        )
    ) {
        return null
    }
    val locale = localeOverrides.getString("locale")!!
    val spacing = if (hasNonNullKey(localeOverrides, "spacing")) localeOverrides.getInt("spacing").toUInt() else null
    val symbol = localeOverrides.getMap("symbol")?.let { asSymbol(it) }!!
    return LocaleOverrides(
        locale,
        spacing,
        symbol,
    )
}

fun readableMapOf(localeOverrides: LocaleOverrides): ReadableMap =
    readableMapOf(
        "locale" to localeOverrides.locale,
        "spacing" to localeOverrides.spacing,
        "symbol" to readableMapOf(localeOverrides.symbol),
    )

fun asLocaleOverridesList(arr: ReadableArray): List<LocaleOverrides> {
    val list = ArrayList<LocaleOverrides>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLocaleOverrides(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLocalizedName(localizedName: ReadableMap): LocalizedName? {
    if (!validateMandatoryFields(
            localizedName,
            arrayOf(
                "locale",
                "name",
            ),
        )
    ) {
        return null
    }
    val locale = localizedName.getString("locale")!!
    val name = localizedName.getString("name")!!
    return LocalizedName(
        locale,
        name,
    )
}

fun readableMapOf(localizedName: LocalizedName): ReadableMap =
    readableMapOf(
        "locale" to localizedName.locale,
        "name" to localizedName.name,
    )

fun asLocalizedNameList(arr: ReadableArray): List<LocalizedName> {
    val list = ArrayList<LocalizedName>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLocalizedName(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLogEntry(logEntry: ReadableMap): LogEntry? {
    if (!validateMandatoryFields(
            logEntry,
            arrayOf(
                "line",
                "level",
            ),
        )
    ) {
        return null
    }
    val line = logEntry.getString("line")!!
    val level = logEntry.getString("level")!!
    return LogEntry(
        line,
        level,
    )
}

fun readableMapOf(logEntry: LogEntry): ReadableMap =
    readableMapOf(
        "line" to logEntry.line,
        "level" to logEntry.level,
    )

fun asLogEntryList(arr: ReadableArray): List<LogEntry> {
    val list = ArrayList<LogEntry>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLogEntry(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asMessageSuccessActionData(messageSuccessActionData: ReadableMap): MessageSuccessActionData? {
    if (!validateMandatoryFields(
            messageSuccessActionData,
            arrayOf(
                "message",
            ),
        )
    ) {
        return null
    }
    val message = messageSuccessActionData.getString("message")!!
    return MessageSuccessActionData(
        message,
    )
}

fun readableMapOf(messageSuccessActionData: MessageSuccessActionData): ReadableMap =
    readableMapOf(
        "message" to messageSuccessActionData.message,
    )

fun asMessageSuccessActionDataList(arr: ReadableArray): List<MessageSuccessActionData> {
    val list = ArrayList<MessageSuccessActionData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asMessageSuccessActionData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asOnchainPaymentLimitsResponse(onchainPaymentLimitsResponse: ReadableMap): OnchainPaymentLimitsResponse? {
    if (!validateMandatoryFields(
            onchainPaymentLimitsResponse,
            arrayOf(
                "send",
                "receive",
            ),
        )
    ) {
        return null
    }
    val send = onchainPaymentLimitsResponse.getMap("send")?.let { asLimits(it) }!!
    val receive = onchainPaymentLimitsResponse.getMap("receive")?.let { asLimits(it) }!!
    return OnchainPaymentLimitsResponse(
        send,
        receive,
    )
}

fun readableMapOf(onchainPaymentLimitsResponse: OnchainPaymentLimitsResponse): ReadableMap =
    readableMapOf(
        "send" to readableMapOf(onchainPaymentLimitsResponse.send),
        "receive" to readableMapOf(onchainPaymentLimitsResponse.receive),
    )

fun asOnchainPaymentLimitsResponseList(arr: ReadableArray): List<OnchainPaymentLimitsResponse> {
    val list = ArrayList<OnchainPaymentLimitsResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asOnchainPaymentLimitsResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPayOnchainRequest(payOnchainRequest: ReadableMap): PayOnchainRequest? {
    if (!validateMandatoryFields(
            payOnchainRequest,
            arrayOf(
                "address",
                "prepareResponse",
            ),
        )
    ) {
        return null
    }
    val address = payOnchainRequest.getString("address")!!
    val prepareResponse = payOnchainRequest.getMap("prepareResponse")?.let { asPreparePayOnchainResponse(it) }!!
    return PayOnchainRequest(
        address,
        prepareResponse,
    )
}

fun readableMapOf(payOnchainRequest: PayOnchainRequest): ReadableMap =
    readableMapOf(
        "address" to payOnchainRequest.address,
        "prepareResponse" to readableMapOf(payOnchainRequest.prepareResponse),
    )

fun asPayOnchainRequestList(arr: ReadableArray): List<PayOnchainRequest> {
    val list = ArrayList<PayOnchainRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPayOnchainRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPayment(payment: ReadableMap): Payment? {
    if (!validateMandatoryFields(
            payment,
            arrayOf(
                "timestamp",
                "amountSat",
                "feesSat",
                "paymentType",
                "status",
                "description",
            ),
        )
    ) {
        return null
    }
    val timestamp = payment.getInt("timestamp").toUInt()
    val amountSat = payment.getDouble("amountSat").toULong()
    val feesSat = payment.getDouble("feesSat").toULong()
    val paymentType = payment.getString("paymentType")?.let { asPaymentType(it) }!!
    val status = payment.getString("status")?.let { asPaymentState(it) }!!
    val description = payment.getString("description")!!
    val txId = if (hasNonNullKey(payment, "txId")) payment.getString("txId") else null
    val swapId = if (hasNonNullKey(payment, "swapId")) payment.getString("swapId") else null
    val preimage = if (hasNonNullKey(payment, "preimage")) payment.getString("preimage") else null
    val bolt11 = if (hasNonNullKey(payment, "bolt11")) payment.getString("bolt11") else null
    val refundTxId = if (hasNonNullKey(payment, "refundTxId")) payment.getString("refundTxId") else null
    val refundTxAmountSat = if (hasNonNullKey(payment, "refundTxAmountSat")) payment.getDouble("refundTxAmountSat").toULong() else null
    return Payment(
        timestamp,
        amountSat,
        feesSat,
        paymentType,
        status,
        description,
        txId,
        swapId,
        preimage,
        bolt11,
        refundTxId,
        refundTxAmountSat,
    )
}

fun readableMapOf(payment: Payment): ReadableMap =
    readableMapOf(
        "timestamp" to payment.timestamp,
        "amountSat" to payment.amountSat,
        "feesSat" to payment.feesSat,
        "paymentType" to payment.paymentType.name.lowercase(),
        "status" to payment.status.name.lowercase(),
        "description" to payment.description,
        "txId" to payment.txId,
        "swapId" to payment.swapId,
        "preimage" to payment.preimage,
        "bolt11" to payment.bolt11,
        "refundTxId" to payment.refundTxId,
        "refundTxAmountSat" to payment.refundTxAmountSat,
    )

fun asPaymentList(arr: ReadableArray): List<Payment> {
    val list = ArrayList<Payment>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPayment(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareBuyBitcoinRequest(prepareBuyBitcoinRequest: ReadableMap): PrepareBuyBitcoinRequest? {
    if (!validateMandatoryFields(
            prepareBuyBitcoinRequest,
            arrayOf(
                "provider",
                "amountSat",
            ),
        )
    ) {
        return null
    }
    val provider = prepareBuyBitcoinRequest.getString("provider")?.let { asBuyBitcoinProvider(it) }!!
    val amountSat = prepareBuyBitcoinRequest.getDouble("amountSat").toULong()
    return PrepareBuyBitcoinRequest(
        provider,
        amountSat,
    )
}

fun readableMapOf(prepareBuyBitcoinRequest: PrepareBuyBitcoinRequest): ReadableMap =
    readableMapOf(
        "provider" to prepareBuyBitcoinRequest.provider.name.lowercase(),
        "amountSat" to prepareBuyBitcoinRequest.amountSat,
    )

fun asPrepareBuyBitcoinRequestList(arr: ReadableArray): List<PrepareBuyBitcoinRequest> {
    val list = ArrayList<PrepareBuyBitcoinRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareBuyBitcoinRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareBuyBitcoinResponse(prepareBuyBitcoinResponse: ReadableMap): PrepareBuyBitcoinResponse? {
    if (!validateMandatoryFields(
            prepareBuyBitcoinResponse,
            arrayOf(
                "provider",
                "amountSat",
                "feesSat",
            ),
        )
    ) {
        return null
    }
    val provider = prepareBuyBitcoinResponse.getString("provider")?.let { asBuyBitcoinProvider(it) }!!
    val amountSat = prepareBuyBitcoinResponse.getDouble("amountSat").toULong()
    val feesSat = prepareBuyBitcoinResponse.getDouble("feesSat").toULong()
    return PrepareBuyBitcoinResponse(
        provider,
        amountSat,
        feesSat,
    )
}

fun readableMapOf(prepareBuyBitcoinResponse: PrepareBuyBitcoinResponse): ReadableMap =
    readableMapOf(
        "provider" to prepareBuyBitcoinResponse.provider.name.lowercase(),
        "amountSat" to prepareBuyBitcoinResponse.amountSat,
        "feesSat" to prepareBuyBitcoinResponse.feesSat,
    )

fun asPrepareBuyBitcoinResponseList(arr: ReadableArray): List<PrepareBuyBitcoinResponse> {
    val list = ArrayList<PrepareBuyBitcoinResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareBuyBitcoinResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPreparePayOnchainRequest(preparePayOnchainRequest: ReadableMap): PreparePayOnchainRequest? {
    if (!validateMandatoryFields(
            preparePayOnchainRequest,
            arrayOf(
                "receiverAmountSat",
            ),
        )
    ) {
        return null
    }
    val receiverAmountSat = preparePayOnchainRequest.getDouble("receiverAmountSat").toULong()
    val satPerVbyte =
        if (hasNonNullKey(
                preparePayOnchainRequest,
                "satPerVbyte",
            )
        ) {
            preparePayOnchainRequest.getInt("satPerVbyte").toUInt()
        } else {
            null
        }
    return PreparePayOnchainRequest(
        receiverAmountSat,
        satPerVbyte,
    )
}

fun readableMapOf(preparePayOnchainRequest: PreparePayOnchainRequest): ReadableMap =
    readableMapOf(
        "receiverAmountSat" to preparePayOnchainRequest.receiverAmountSat,
        "satPerVbyte" to preparePayOnchainRequest.satPerVbyte,
    )

fun asPreparePayOnchainRequestList(arr: ReadableArray): List<PreparePayOnchainRequest> {
    val list = ArrayList<PreparePayOnchainRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPreparePayOnchainRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPreparePayOnchainResponse(preparePayOnchainResponse: ReadableMap): PreparePayOnchainResponse? {
    if (!validateMandatoryFields(
            preparePayOnchainResponse,
            arrayOf(
                "receiverAmountSat",
                "claimFeesSat",
                "totalFeesSat",
            ),
        )
    ) {
        return null
    }
    val receiverAmountSat = preparePayOnchainResponse.getDouble("receiverAmountSat").toULong()
    val claimFeesSat = preparePayOnchainResponse.getDouble("claimFeesSat").toULong()
    val totalFeesSat = preparePayOnchainResponse.getDouble("totalFeesSat").toULong()
    return PreparePayOnchainResponse(
        receiverAmountSat,
        claimFeesSat,
        totalFeesSat,
    )
}

fun readableMapOf(preparePayOnchainResponse: PreparePayOnchainResponse): ReadableMap =
    readableMapOf(
        "receiverAmountSat" to preparePayOnchainResponse.receiverAmountSat,
        "claimFeesSat" to preparePayOnchainResponse.claimFeesSat,
        "totalFeesSat" to preparePayOnchainResponse.totalFeesSat,
    )

fun asPreparePayOnchainResponseList(arr: ReadableArray): List<PreparePayOnchainResponse> {
    val list = ArrayList<PreparePayOnchainResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPreparePayOnchainResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareReceiveRequest(prepareReceiveRequest: ReadableMap): PrepareReceiveRequest? {
    if (!validateMandatoryFields(
            prepareReceiveRequest,
            arrayOf(
                "paymentMethod",
            ),
        )
    ) {
        return null
    }
    val amountSat = if (hasNonNullKey(prepareReceiveRequest, "amountSat")) prepareReceiveRequest.getDouble("amountSat").toULong() else null
    val paymentMethod = prepareReceiveRequest.getString("paymentMethod")?.let { asPaymentMethod(it) }!!
    return PrepareReceiveRequest(
        amountSat,
        paymentMethod,
    )
}

fun readableMapOf(prepareReceiveRequest: PrepareReceiveRequest): ReadableMap =
    readableMapOf(
        "amountSat" to prepareReceiveRequest.amountSat,
        "paymentMethod" to prepareReceiveRequest.paymentMethod.name.lowercase(),
    )

fun asPrepareReceiveRequestList(arr: ReadableArray): List<PrepareReceiveRequest> {
    val list = ArrayList<PrepareReceiveRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareReceiveRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareReceiveResponse(prepareReceiveResponse: ReadableMap): PrepareReceiveResponse? {
    if (!validateMandatoryFields(
            prepareReceiveResponse,
            arrayOf(
                "paymentMethod",
                "feesSat",
            ),
        )
    ) {
        return null
    }
    val amountSat =
        if (hasNonNullKey(
                prepareReceiveResponse,
                "amountSat",
            )
        ) {
            prepareReceiveResponse.getDouble("amountSat").toULong()
        } else {
            null
        }
    val paymentMethod = prepareReceiveResponse.getString("paymentMethod")?.let { asPaymentMethod(it) }!!
    val feesSat = prepareReceiveResponse.getDouble("feesSat").toULong()
    return PrepareReceiveResponse(
        amountSat,
        paymentMethod,
        feesSat,
    )
}

fun readableMapOf(prepareReceiveResponse: PrepareReceiveResponse): ReadableMap =
    readableMapOf(
        "amountSat" to prepareReceiveResponse.amountSat,
        "paymentMethod" to prepareReceiveResponse.paymentMethod.name.lowercase(),
        "feesSat" to prepareReceiveResponse.feesSat,
    )

fun asPrepareReceiveResponseList(arr: ReadableArray): List<PrepareReceiveResponse> {
    val list = ArrayList<PrepareReceiveResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareReceiveResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareRefundRequest(prepareRefundRequest: ReadableMap): PrepareRefundRequest? {
    if (!validateMandatoryFields(
            prepareRefundRequest,
            arrayOf(
                "swapAddress",
                "refundAddress",
                "satPerVbyte",
            ),
        )
    ) {
        return null
    }
    val swapAddress = prepareRefundRequest.getString("swapAddress")!!
    val refundAddress = prepareRefundRequest.getString("refundAddress")!!
    val satPerVbyte = prepareRefundRequest.getInt("satPerVbyte").toUInt()
    return PrepareRefundRequest(
        swapAddress,
        refundAddress,
        satPerVbyte,
    )
}

fun readableMapOf(prepareRefundRequest: PrepareRefundRequest): ReadableMap =
    readableMapOf(
        "swapAddress" to prepareRefundRequest.swapAddress,
        "refundAddress" to prepareRefundRequest.refundAddress,
        "satPerVbyte" to prepareRefundRequest.satPerVbyte,
    )

fun asPrepareRefundRequestList(arr: ReadableArray): List<PrepareRefundRequest> {
    val list = ArrayList<PrepareRefundRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareRefundRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareRefundResponse(prepareRefundResponse: ReadableMap): PrepareRefundResponse? {
    if (!validateMandatoryFields(
            prepareRefundResponse,
            arrayOf(
                "txVsize",
                "txFeeSat",
            ),
        )
    ) {
        return null
    }
    val txVsize = prepareRefundResponse.getInt("txVsize").toUInt()
    val txFeeSat = prepareRefundResponse.getDouble("txFeeSat").toULong()
    val refundTxId = if (hasNonNullKey(prepareRefundResponse, "refundTxId")) prepareRefundResponse.getString("refundTxId") else null
    return PrepareRefundResponse(
        txVsize,
        txFeeSat,
        refundTxId,
    )
}

fun readableMapOf(prepareRefundResponse: PrepareRefundResponse): ReadableMap =
    readableMapOf(
        "txVsize" to prepareRefundResponse.txVsize,
        "txFeeSat" to prepareRefundResponse.txFeeSat,
        "refundTxId" to prepareRefundResponse.refundTxId,
    )

fun asPrepareRefundResponseList(arr: ReadableArray): List<PrepareRefundResponse> {
    val list = ArrayList<PrepareRefundResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareRefundResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareSendRequest(prepareSendRequest: ReadableMap): PrepareSendRequest? {
    if (!validateMandatoryFields(
            prepareSendRequest,
            arrayOf(
                "destination",
            ),
        )
    ) {
        return null
    }
    val destination = prepareSendRequest.getString("destination")!!
    val amountSat = if (hasNonNullKey(prepareSendRequest, "amountSat")) prepareSendRequest.getDouble("amountSat").toULong() else null
    return PrepareSendRequest(
        destination,
        amountSat,
    )
}

fun readableMapOf(prepareSendRequest: PrepareSendRequest): ReadableMap =
    readableMapOf(
        "destination" to prepareSendRequest.destination,
        "amountSat" to prepareSendRequest.amountSat,
    )

fun asPrepareSendRequestList(arr: ReadableArray): List<PrepareSendRequest> {
    val list = ArrayList<PrepareSendRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareSendRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPrepareSendResponse(prepareSendResponse: ReadableMap): PrepareSendResponse? {
    if (!validateMandatoryFields(
            prepareSendResponse,
            arrayOf(
                "destination",
                "feesSat",
            ),
        )
    ) {
        return null
    }
    val destination = prepareSendResponse.getMap("destination")?.let { asSendDestination(it) }!!
    val feesSat = prepareSendResponse.getDouble("feesSat").toULong()
    return PrepareSendResponse(
        destination,
        feesSat,
    )
}

fun readableMapOf(prepareSendResponse: PrepareSendResponse): ReadableMap =
    readableMapOf(
        "destination" to readableMapOf(prepareSendResponse.destination),
        "feesSat" to prepareSendResponse.feesSat,
    )

fun asPrepareSendResponseList(arr: ReadableArray): List<PrepareSendResponse> {
    val list = ArrayList<PrepareSendResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asPrepareSendResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asRate(rate: ReadableMap): Rate? {
    if (!validateMandatoryFields(
            rate,
            arrayOf(
                "coin",
                "value",
            ),
        )
    ) {
        return null
    }
    val coin = rate.getString("coin")!!
    val value = rate.getDouble("value")
    return Rate(
        coin,
        value,
    )
}

fun readableMapOf(rate: Rate): ReadableMap =
    readableMapOf(
        "coin" to rate.coin,
        "value" to rate.value,
    )

fun asRateList(arr: ReadableArray): List<Rate> {
    val list = ArrayList<Rate>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRate(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asReceivePaymentRequest(receivePaymentRequest: ReadableMap): ReceivePaymentRequest? {
    if (!validateMandatoryFields(
            receivePaymentRequest,
            arrayOf(
                "prepareResponse",
            ),
        )
    ) {
        return null
    }
    val prepareResponse = receivePaymentRequest.getMap("prepareResponse")?.let { asPrepareReceiveResponse(it) }!!
    val description = if (hasNonNullKey(receivePaymentRequest, "description")) receivePaymentRequest.getString("description") else null
    return ReceivePaymentRequest(
        prepareResponse,
        description,
    )
}

fun readableMapOf(receivePaymentRequest: ReceivePaymentRequest): ReadableMap =
    readableMapOf(
        "prepareResponse" to readableMapOf(receivePaymentRequest.prepareResponse),
        "description" to receivePaymentRequest.description,
    )

fun asReceivePaymentRequestList(arr: ReadableArray): List<ReceivePaymentRequest> {
    val list = ArrayList<ReceivePaymentRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asReceivePaymentRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asReceivePaymentResponse(receivePaymentResponse: ReadableMap): ReceivePaymentResponse? {
    if (!validateMandatoryFields(
            receivePaymentResponse,
            arrayOf(
                "destination",
            ),
        )
    ) {
        return null
    }
    val destination = receivePaymentResponse.getString("destination")!!
    return ReceivePaymentResponse(
        destination,
    )
}

fun readableMapOf(receivePaymentResponse: ReceivePaymentResponse): ReadableMap =
    readableMapOf(
        "destination" to receivePaymentResponse.destination,
    )

fun asReceivePaymentResponseList(arr: ReadableArray): List<ReceivePaymentResponse> {
    val list = ArrayList<ReceivePaymentResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asReceivePaymentResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asRecommendedFees(recommendedFees: ReadableMap): RecommendedFees? {
    if (!validateMandatoryFields(
            recommendedFees,
            arrayOf(
                "fastestFee",
                "halfHourFee",
                "hourFee",
                "economyFee",
                "minimumFee",
            ),
        )
    ) {
        return null
    }
    val fastestFee = recommendedFees.getDouble("fastestFee").toULong()
    val halfHourFee = recommendedFees.getDouble("halfHourFee").toULong()
    val hourFee = recommendedFees.getDouble("hourFee").toULong()
    val economyFee = recommendedFees.getDouble("economyFee").toULong()
    val minimumFee = recommendedFees.getDouble("minimumFee").toULong()
    return RecommendedFees(
        fastestFee,
        halfHourFee,
        hourFee,
        economyFee,
        minimumFee,
    )
}

fun readableMapOf(recommendedFees: RecommendedFees): ReadableMap =
    readableMapOf(
        "fastestFee" to recommendedFees.fastestFee,
        "halfHourFee" to recommendedFees.halfHourFee,
        "hourFee" to recommendedFees.hourFee,
        "economyFee" to recommendedFees.economyFee,
        "minimumFee" to recommendedFees.minimumFee,
    )

fun asRecommendedFeesList(arr: ReadableArray): List<RecommendedFees> {
    val list = ArrayList<RecommendedFees>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRecommendedFees(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asRefundRequest(refundRequest: ReadableMap): RefundRequest? {
    if (!validateMandatoryFields(
            refundRequest,
            arrayOf(
                "swapAddress",
                "refundAddress",
                "satPerVbyte",
            ),
        )
    ) {
        return null
    }
    val swapAddress = refundRequest.getString("swapAddress")!!
    val refundAddress = refundRequest.getString("refundAddress")!!
    val satPerVbyte = refundRequest.getInt("satPerVbyte").toUInt()
    return RefundRequest(
        swapAddress,
        refundAddress,
        satPerVbyte,
    )
}

fun readableMapOf(refundRequest: RefundRequest): ReadableMap =
    readableMapOf(
        "swapAddress" to refundRequest.swapAddress,
        "refundAddress" to refundRequest.refundAddress,
        "satPerVbyte" to refundRequest.satPerVbyte,
    )

fun asRefundRequestList(arr: ReadableArray): List<RefundRequest> {
    val list = ArrayList<RefundRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRefundRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asRefundResponse(refundResponse: ReadableMap): RefundResponse? {
    if (!validateMandatoryFields(
            refundResponse,
            arrayOf(
                "refundTxId",
            ),
        )
    ) {
        return null
    }
    val refundTxId = refundResponse.getString("refundTxId")!!
    return RefundResponse(
        refundTxId,
    )
}

fun readableMapOf(refundResponse: RefundResponse): ReadableMap =
    readableMapOf(
        "refundTxId" to refundResponse.refundTxId,
    )

fun asRefundResponseList(arr: ReadableArray): List<RefundResponse> {
    val list = ArrayList<RefundResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRefundResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asRefundableSwap(refundableSwap: ReadableMap): RefundableSwap? {
    if (!validateMandatoryFields(
            refundableSwap,
            arrayOf(
                "swapAddress",
                "timestamp",
                "amountSat",
            ),
        )
    ) {
        return null
    }
    val swapAddress = refundableSwap.getString("swapAddress")!!
    val timestamp = refundableSwap.getInt("timestamp").toUInt()
    val amountSat = refundableSwap.getDouble("amountSat").toULong()
    return RefundableSwap(
        swapAddress,
        timestamp,
        amountSat,
    )
}

fun readableMapOf(refundableSwap: RefundableSwap): ReadableMap =
    readableMapOf(
        "swapAddress" to refundableSwap.swapAddress,
        "timestamp" to refundableSwap.timestamp,
        "amountSat" to refundableSwap.amountSat,
    )

fun asRefundableSwapList(arr: ReadableArray): List<RefundableSwap> {
    val list = ArrayList<RefundableSwap>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRefundableSwap(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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

fun readableMapOf(restoreRequest: RestoreRequest): ReadableMap =
    readableMapOf(
        "backupPath" to restoreRequest.backupPath,
    )

fun asRestoreRequestList(arr: ReadableArray): List<RestoreRequest> {
    val list = ArrayList<RestoreRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRestoreRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asRouteHint(routeHint: ReadableMap): RouteHint? {
    if (!validateMandatoryFields(
            routeHint,
            arrayOf(
                "hops",
            ),
        )
    ) {
        return null
    }
    val hops = routeHint.getArray("hops")?.let { asRouteHintHopList(it) }!!
    return RouteHint(
        hops,
    )
}

fun readableMapOf(routeHint: RouteHint): ReadableMap =
    readableMapOf(
        "hops" to readableArrayOf(routeHint.hops),
    )

fun asRouteHintList(arr: ReadableArray): List<RouteHint> {
    val list = ArrayList<RouteHint>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRouteHint(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asRouteHintHop(routeHintHop: ReadableMap): RouteHintHop? {
    if (!validateMandatoryFields(
            routeHintHop,
            arrayOf(
                "srcNodeId",
                "shortChannelId",
                "feesBaseMsat",
                "feesProportionalMillionths",
                "cltvExpiryDelta",
            ),
        )
    ) {
        return null
    }
    val srcNodeId = routeHintHop.getString("srcNodeId")!!
    val shortChannelId = routeHintHop.getDouble("shortChannelId").toULong()
    val feesBaseMsat = routeHintHop.getInt("feesBaseMsat").toUInt()
    val feesProportionalMillionths = routeHintHop.getInt("feesProportionalMillionths").toUInt()
    val cltvExpiryDelta = routeHintHop.getDouble("cltvExpiryDelta").toULong()
    val htlcMinimumMsat = if (hasNonNullKey(routeHintHop, "htlcMinimumMsat")) routeHintHop.getDouble("htlcMinimumMsat").toULong() else null
    val htlcMaximumMsat = if (hasNonNullKey(routeHintHop, "htlcMaximumMsat")) routeHintHop.getDouble("htlcMaximumMsat").toULong() else null
    return RouteHintHop(
        srcNodeId,
        shortChannelId,
        feesBaseMsat,
        feesProportionalMillionths,
        cltvExpiryDelta,
        htlcMinimumMsat,
        htlcMaximumMsat,
    )
}

fun readableMapOf(routeHintHop: RouteHintHop): ReadableMap =
    readableMapOf(
        "srcNodeId" to routeHintHop.srcNodeId,
        "shortChannelId" to routeHintHop.shortChannelId,
        "feesBaseMsat" to routeHintHop.feesBaseMsat,
        "feesProportionalMillionths" to routeHintHop.feesProportionalMillionths,
        "cltvExpiryDelta" to routeHintHop.cltvExpiryDelta,
        "htlcMinimumMsat" to routeHintHop.htlcMinimumMsat,
        "htlcMaximumMsat" to routeHintHop.htlcMaximumMsat,
    )

fun asRouteHintHopList(arr: ReadableArray): List<RouteHintHop> {
    val list = ArrayList<RouteHintHop>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asRouteHintHop(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asSendPaymentRequest(sendPaymentRequest: ReadableMap): SendPaymentRequest? {
    if (!validateMandatoryFields(
            sendPaymentRequest,
            arrayOf(
                "prepareResponse",
            ),
        )
    ) {
        return null
    }
    val prepareResponse = sendPaymentRequest.getMap("prepareResponse")?.let { asPrepareSendResponse(it) }!!
    return SendPaymentRequest(
        prepareResponse,
    )
}

fun readableMapOf(sendPaymentRequest: SendPaymentRequest): ReadableMap =
    readableMapOf(
        "prepareResponse" to readableMapOf(sendPaymentRequest.prepareResponse),
    )

fun asSendPaymentRequestList(arr: ReadableArray): List<SendPaymentRequest> {
    val list = ArrayList<SendPaymentRequest>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSendPaymentRequest(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asSendPaymentResponse(sendPaymentResponse: ReadableMap): SendPaymentResponse? {
    if (!validateMandatoryFields(
            sendPaymentResponse,
            arrayOf(
                "payment",
            ),
        )
    ) {
        return null
    }
    val payment = sendPaymentResponse.getMap("payment")?.let { asPayment(it) }!!
    return SendPaymentResponse(
        payment,
    )
}

fun readableMapOf(sendPaymentResponse: SendPaymentResponse): ReadableMap =
    readableMapOf(
        "payment" to readableMapOf(sendPaymentResponse.payment),
    )

fun asSendPaymentResponseList(arr: ReadableArray): List<SendPaymentResponse> {
    val list = ArrayList<SendPaymentResponse>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSendPaymentResponse(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asSymbol(symbol: ReadableMap): Symbol? {
    if (!validateMandatoryFields(
            symbol,
            arrayOf(),
        )
    ) {
        return null
    }
    val grapheme = if (hasNonNullKey(symbol, "grapheme")) symbol.getString("grapheme") else null
    val template = if (hasNonNullKey(symbol, "template")) symbol.getString("template") else null
    val rtl = if (hasNonNullKey(symbol, "rtl")) symbol.getBoolean("rtl") else null
    val position = if (hasNonNullKey(symbol, "position")) symbol.getInt("position").toUInt() else null
    return Symbol(
        grapheme,
        template,
        rtl,
        position,
    )
}

fun readableMapOf(symbol: Symbol): ReadableMap =
    readableMapOf(
        "grapheme" to symbol.grapheme,
        "template" to symbol.template,
        "rtl" to symbol.rtl,
        "position" to symbol.position,
    )

fun asSymbolList(arr: ReadableArray): List<Symbol> {
    val list = ArrayList<Symbol>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSymbol(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asUrlSuccessActionData(urlSuccessActionData: ReadableMap): UrlSuccessActionData? {
    if (!validateMandatoryFields(
            urlSuccessActionData,
            arrayOf(
                "description",
                "url",
                "matchesCallbackDomain",
            ),
        )
    ) {
        return null
    }
    val description = urlSuccessActionData.getString("description")!!
    val url = urlSuccessActionData.getString("url")!!
    val matchesCallbackDomain = urlSuccessActionData.getBoolean("matchesCallbackDomain")
    return UrlSuccessActionData(
        description,
        url,
        matchesCallbackDomain,
    )
}

fun readableMapOf(urlSuccessActionData: UrlSuccessActionData): ReadableMap =
    readableMapOf(
        "description" to urlSuccessActionData.description,
        "url" to urlSuccessActionData.url,
        "matchesCallbackDomain" to urlSuccessActionData.matchesCallbackDomain,
    )

fun asUrlSuccessActionDataList(arr: ReadableArray): List<UrlSuccessActionData> {
    val list = ArrayList<UrlSuccessActionData>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asUrlSuccessActionData(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asAesSuccessActionDataResult(aesSuccessActionDataResult: ReadableMap): AesSuccessActionDataResult? {
    val type = aesSuccessActionDataResult.getString("type")

    if (type == "decrypted") {
        return AesSuccessActionDataResult.Decrypted(
            aesSuccessActionDataResult.getMap("data")?.let { asAesSuccessActionDataDecrypted(it) }!!,
        )
    }
    if (type == "errorStatus") {
        return AesSuccessActionDataResult.ErrorStatus(aesSuccessActionDataResult.getString("reason")!!)
    }
    return null
}

fun readableMapOf(aesSuccessActionDataResult: AesSuccessActionDataResult): ReadableMap? {
    val map = Arguments.createMap()
    when (aesSuccessActionDataResult) {
        is AesSuccessActionDataResult.Decrypted -> {
            pushToMap(map, "type", "decrypted")
            pushToMap(map, "data", readableMapOf(aesSuccessActionDataResult.data))
        }
        is AesSuccessActionDataResult.ErrorStatus -> {
            pushToMap(map, "type", "errorStatus")
            pushToMap(map, "reason", aesSuccessActionDataResult.reason)
        }
    }
    return map
}

fun asAesSuccessActionDataResultList(arr: ReadableArray): List<AesSuccessActionDataResult> {
    val list = ArrayList<AesSuccessActionDataResult>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asAesSuccessActionDataResult(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asBuyBitcoinProvider(type: String): BuyBitcoinProvider = BuyBitcoinProvider.valueOf(camelToUpperSnakeCase(type))

fun asBuyBitcoinProviderList(arr: ReadableArray): List<BuyBitcoinProvider> {
    val list = ArrayList<BuyBitcoinProvider>()
    for (value in arr.toArrayList()) {
        when (value) {
            is String -> list.add(asBuyBitcoinProvider(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asInputType(inputType: ReadableMap): InputType? {
    val type = inputType.getString("type")

    if (type == "bitcoinAddress") {
        return InputType.BitcoinAddress(inputType.getMap("address")?.let { asBitcoinAddressData(it) }!!)
    }
    if (type == "liquidAddress") {
        return InputType.LiquidAddress(inputType.getMap("address")?.let { asLiquidAddressData(it) }!!)
    }
    if (type == "bolt11") {
        return InputType.Bolt11(inputType.getMap("invoice")?.let { asLnInvoice(it) }!!)
    }
    if (type == "nodeId") {
        return InputType.NodeId(inputType.getString("nodeId")!!)
    }
    if (type == "url") {
        return InputType.Url(inputType.getString("url")!!)
    }
    if (type == "lnUrlPay") {
        return InputType.LnUrlPay(inputType.getMap("data")?.let { asLnUrlPayRequestData(it) }!!)
    }
    if (type == "lnUrlWithdraw") {
        return InputType.LnUrlWithdraw(inputType.getMap("data")?.let { asLnUrlWithdrawRequestData(it) }!!)
    }
    if (type == "lnUrlAuth") {
        return InputType.LnUrlAuth(inputType.getMap("data")?.let { asLnUrlAuthRequestData(it) }!!)
    }
    if (type == "lnUrlError") {
        return InputType.LnUrlError(inputType.getMap("data")?.let { asLnUrlErrorData(it) }!!)
    }
    return null
}

fun readableMapOf(inputType: InputType): ReadableMap? {
    val map = Arguments.createMap()
    when (inputType) {
        is InputType.BitcoinAddress -> {
            pushToMap(map, "type", "bitcoinAddress")
            pushToMap(map, "address", readableMapOf(inputType.address))
        }
        is InputType.LiquidAddress -> {
            pushToMap(map, "type", "liquidAddress")
            pushToMap(map, "address", readableMapOf(inputType.address))
        }
        is InputType.Bolt11 -> {
            pushToMap(map, "type", "bolt11")
            pushToMap(map, "invoice", readableMapOf(inputType.invoice))
        }
        is InputType.NodeId -> {
            pushToMap(map, "type", "nodeId")
            pushToMap(map, "nodeId", inputType.nodeId)
        }
        is InputType.Url -> {
            pushToMap(map, "type", "url")
            pushToMap(map, "url", inputType.url)
        }
        is InputType.LnUrlPay -> {
            pushToMap(map, "type", "lnUrlPay")
            pushToMap(map, "data", readableMapOf(inputType.data))
        }
        is InputType.LnUrlWithdraw -> {
            pushToMap(map, "type", "lnUrlWithdraw")
            pushToMap(map, "data", readableMapOf(inputType.data))
        }
        is InputType.LnUrlAuth -> {
            pushToMap(map, "type", "lnUrlAuth")
            pushToMap(map, "data", readableMapOf(inputType.data))
        }
        is InputType.LnUrlError -> {
            pushToMap(map, "type", "lnUrlError")
            pushToMap(map, "data", readableMapOf(inputType.data))
        }
    }
    return map
}

fun asInputTypeList(arr: ReadableArray): List<InputType> {
    val list = ArrayList<InputType>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asInputType(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLiquidNetwork(type: String): LiquidNetwork = LiquidNetwork.valueOf(camelToUpperSnakeCase(type))

fun asLiquidNetworkList(arr: ReadableArray): List<LiquidNetwork> {
    val list = ArrayList<LiquidNetwork>()
    for (value in arr.toArrayList()) {
        when (value) {
            is String -> list.add(asLiquidNetwork(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlCallbackStatus(lnUrlCallbackStatus: ReadableMap): LnUrlCallbackStatus? {
    val type = lnUrlCallbackStatus.getString("type")

    if (type == "ok") {
        return LnUrlCallbackStatus.Ok
    }
    if (type == "errorStatus") {
        return LnUrlCallbackStatus.ErrorStatus(lnUrlCallbackStatus.getMap("data")?.let { asLnUrlErrorData(it) }!!)
    }
    return null
}

fun readableMapOf(lnUrlCallbackStatus: LnUrlCallbackStatus): ReadableMap? {
    val map = Arguments.createMap()
    when (lnUrlCallbackStatus) {
        is LnUrlCallbackStatus.Ok -> {
            pushToMap(map, "type", "ok")
        }
        is LnUrlCallbackStatus.ErrorStatus -> {
            pushToMap(map, "type", "errorStatus")
            pushToMap(map, "data", readableMapOf(lnUrlCallbackStatus.data))
        }
    }
    return map
}

fun asLnUrlCallbackStatusList(arr: ReadableArray): List<LnUrlCallbackStatus> {
    val list = ArrayList<LnUrlCallbackStatus>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlCallbackStatus(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlPayResult(lnUrlPayResult: ReadableMap): LnUrlPayResult? {
    val type = lnUrlPayResult.getString("type")

    if (type == "endpointSuccess") {
        return LnUrlPayResult.EndpointSuccess(lnUrlPayResult.getMap("data")?.let { asLnUrlPaySuccessData(it) }!!)
    }
    if (type == "endpointError") {
        return LnUrlPayResult.EndpointError(lnUrlPayResult.getMap("data")?.let { asLnUrlErrorData(it) }!!)
    }
    if (type == "payError") {
        return LnUrlPayResult.PayError(lnUrlPayResult.getMap("data")?.let { asLnUrlPayErrorData(it) }!!)
    }
    return null
}

fun readableMapOf(lnUrlPayResult: LnUrlPayResult): ReadableMap? {
    val map = Arguments.createMap()
    when (lnUrlPayResult) {
        is LnUrlPayResult.EndpointSuccess -> {
            pushToMap(map, "type", "endpointSuccess")
            pushToMap(map, "data", readableMapOf(lnUrlPayResult.data))
        }
        is LnUrlPayResult.EndpointError -> {
            pushToMap(map, "type", "endpointError")
            pushToMap(map, "data", readableMapOf(lnUrlPayResult.data))
        }
        is LnUrlPayResult.PayError -> {
            pushToMap(map, "type", "payError")
            pushToMap(map, "data", readableMapOf(lnUrlPayResult.data))
        }
    }
    return map
}

fun asLnUrlPayResultList(arr: ReadableArray): List<LnUrlPayResult> {
    val list = ArrayList<LnUrlPayResult>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlPayResult(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asLnUrlWithdrawResult(lnUrlWithdrawResult: ReadableMap): LnUrlWithdrawResult? {
    val type = lnUrlWithdrawResult.getString("type")

    if (type == "ok") {
        return LnUrlWithdrawResult.Ok(lnUrlWithdrawResult.getMap("data")?.let { asLnUrlWithdrawSuccessData(it) }!!)
    }
    if (type == "timeout") {
        return LnUrlWithdrawResult.Timeout(lnUrlWithdrawResult.getMap("data")?.let { asLnUrlWithdrawSuccessData(it) }!!)
    }
    if (type == "errorStatus") {
        return LnUrlWithdrawResult.ErrorStatus(lnUrlWithdrawResult.getMap("data")?.let { asLnUrlErrorData(it) }!!)
    }
    return null
}

fun readableMapOf(lnUrlWithdrawResult: LnUrlWithdrawResult): ReadableMap? {
    val map = Arguments.createMap()
    when (lnUrlWithdrawResult) {
        is LnUrlWithdrawResult.Ok -> {
            pushToMap(map, "type", "ok")
            pushToMap(map, "data", readableMapOf(lnUrlWithdrawResult.data))
        }
        is LnUrlWithdrawResult.Timeout -> {
            pushToMap(map, "type", "timeout")
            pushToMap(map, "data", readableMapOf(lnUrlWithdrawResult.data))
        }
        is LnUrlWithdrawResult.ErrorStatus -> {
            pushToMap(map, "type", "errorStatus")
            pushToMap(map, "data", readableMapOf(lnUrlWithdrawResult.data))
        }
    }
    return map
}

fun asLnUrlWithdrawResultList(arr: ReadableArray): List<LnUrlWithdrawResult> {
    val list = ArrayList<LnUrlWithdrawResult>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asLnUrlWithdrawResult(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asNetwork(type: String): Network = Network.valueOf(camelToUpperSnakeCase(type))

fun asNetworkList(arr: ReadableArray): List<Network> {
    val list = ArrayList<Network>()
    for (value in arr.toArrayList()) {
        when (value) {
            is String -> list.add(asNetwork(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPaymentMethod(type: String): PaymentMethod = PaymentMethod.valueOf(camelToUpperSnakeCase(type))

fun asPaymentMethodList(arr: ReadableArray): List<PaymentMethod> {
    val list = ArrayList<PaymentMethod>()
    for (value in arr.toArrayList()) {
        when (value) {
            is String -> list.add(asPaymentMethod(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPaymentState(type: String): PaymentState = PaymentState.valueOf(camelToUpperSnakeCase(type))

fun asPaymentStateList(arr: ReadableArray): List<PaymentState> {
    val list = ArrayList<PaymentState>()
    for (value in arr.toArrayList()) {
        when (value) {
            is String -> list.add(asPaymentState(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asPaymentType(type: String): PaymentType = PaymentType.valueOf(camelToUpperSnakeCase(type))

fun asPaymentTypeList(arr: ReadableArray): List<PaymentType> {
    val list = ArrayList<PaymentType>()
    for (value in arr.toArrayList()) {
        when (value) {
            is String -> list.add(asPaymentType(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asSdkEvent(sdkEvent: ReadableMap): SdkEvent? {
    val type = sdkEvent.getString("type")

    if (type == "paymentFailed") {
        return SdkEvent.PaymentFailed(sdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "paymentPending") {
        return SdkEvent.PaymentPending(sdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "paymentRefunded") {
        return SdkEvent.PaymentRefunded(sdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "paymentRefundPending") {
        return SdkEvent.PaymentRefundPending(sdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "paymentSucceeded") {
        return SdkEvent.PaymentSucceeded(sdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "paymentWaitingConfirmation") {
        return SdkEvent.PaymentWaitingConfirmation(sdkEvent.getMap("details")?.let { asPayment(it) }!!)
    }
    if (type == "synced") {
        return SdkEvent.Synced
    }
    return null
}

fun readableMapOf(sdkEvent: SdkEvent): ReadableMap? {
    val map = Arguments.createMap()
    when (sdkEvent) {
        is SdkEvent.PaymentFailed -> {
            pushToMap(map, "type", "paymentFailed")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentPending -> {
            pushToMap(map, "type", "paymentPending")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentRefunded -> {
            pushToMap(map, "type", "paymentRefunded")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentRefundPending -> {
            pushToMap(map, "type", "paymentRefundPending")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentSucceeded -> {
            pushToMap(map, "type", "paymentSucceeded")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.PaymentWaitingConfirmation -> {
            pushToMap(map, "type", "paymentWaitingConfirmation")
            pushToMap(map, "details", readableMapOf(sdkEvent.details))
        }
        is SdkEvent.Synced -> {
            pushToMap(map, "type", "synced")
        }
    }
    return map
}

fun asSdkEventList(arr: ReadableArray): List<SdkEvent> {
    val list = ArrayList<SdkEvent>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSdkEvent(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asSendDestination(sendDestination: ReadableMap): SendDestination? {
    val type = sendDestination.getString("type")

    if (type == "liquidAddress") {
        return SendDestination.LiquidAddress(sendDestination.getMap("addressData")?.let { asLiquidAddressData(it) }!!)
    }
    if (type == "bolt11") {
        return SendDestination.Bolt11(sendDestination.getMap("invoice")?.let { asLnInvoice(it) }!!)
    }
    return null
}

fun readableMapOf(sendDestination: SendDestination): ReadableMap? {
    val map = Arguments.createMap()
    when (sendDestination) {
        is SendDestination.LiquidAddress -> {
            pushToMap(map, "type", "liquidAddress")
            pushToMap(map, "addressData", readableMapOf(sendDestination.addressData))
        }
        is SendDestination.Bolt11 -> {
            pushToMap(map, "type", "bolt11")
            pushToMap(map, "invoice", readableMapOf(sendDestination.invoice))
        }
    }
    return map
}

fun asSendDestinationList(arr: ReadableArray): List<SendDestination> {
    val list = ArrayList<SendDestination>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSendDestination(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
        }
    }
    return list
}

fun asSuccessActionProcessed(successActionProcessed: ReadableMap): SuccessActionProcessed? {
    val type = successActionProcessed.getString("type")

    if (type == "aes") {
        return SuccessActionProcessed.Aes(successActionProcessed.getMap("result")?.let { asAesSuccessActionDataResult(it) }!!)
    }
    if (type == "message") {
        return SuccessActionProcessed.Message(successActionProcessed.getMap("data")?.let { asMessageSuccessActionData(it) }!!)
    }
    if (type == "url") {
        return SuccessActionProcessed.Url(successActionProcessed.getMap("data")?.let { asUrlSuccessActionData(it) }!!)
    }
    return null
}

fun readableMapOf(successActionProcessed: SuccessActionProcessed): ReadableMap? {
    val map = Arguments.createMap()
    when (successActionProcessed) {
        is SuccessActionProcessed.Aes -> {
            pushToMap(map, "type", "aes")
            pushToMap(map, "result", readableMapOf(successActionProcessed.result))
        }
        is SuccessActionProcessed.Message -> {
            pushToMap(map, "type", "message")
            pushToMap(map, "data", readableMapOf(successActionProcessed.data))
        }
        is SuccessActionProcessed.Url -> {
            pushToMap(map, "type", "url")
            pushToMap(map, "data", readableMapOf(successActionProcessed.data))
        }
    }
    return map
}

fun asSuccessActionProcessedList(arr: ReadableArray): List<SuccessActionProcessed> {
    val list = ArrayList<SuccessActionProcessed>()
    for (value in arr.toArrayList()) {
        when (value) {
            is ReadableMap -> list.add(asSuccessActionProcessed(value)!!)
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
): Boolean = map.hasKey(key) && !map.isNull(key)

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
        is FiatCurrency -> array.pushMap(readableMapOf(value))
        is LocaleOverrides -> array.pushMap(readableMapOf(value))
        is LocalizedName -> array.pushMap(readableMapOf(value))
        is Payment -> array.pushMap(readableMapOf(value))
        is PaymentType -> array.pushString(value.name.lowercase())
        is Rate -> array.pushMap(readableMapOf(value))
        is RefundableSwap -> array.pushMap(readableMapOf(value))
        is RouteHint -> array.pushMap(readableMapOf(value))
        is RouteHintHop -> array.pushMap(readableMapOf(value))
        is UByte -> array.pushInt(value.toInt())
        is Array<*> -> array.pushArray(readableArrayOf(value.asIterable()))
        is List<*> -> array.pushArray(readableArrayOf(value))
        else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
        is Float -> map.putDouble(key, value.toDouble())
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
        else -> throw SdkException.Generic("Unexpected type ${value::class.java.name} for key [$key]")
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
            else -> throw SdkException.Generic(errUnexpectedType("${value::class.java.name}"))
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
): String = "Missing mandatory field $fieldName for type $typeName"

fun errUnexpectedType(typeName: String): String = "Unexpected type $typeName"

fun errUnexpectedValue(fieldName: String): String = "Unexpected value for optional field $fieldName"

fun camelToUpperSnakeCase(str: String): String {
    val pattern = "(?<=.)[A-Z]".toRegex()
    return str.replace(pattern, "_$0").uppercase()
}

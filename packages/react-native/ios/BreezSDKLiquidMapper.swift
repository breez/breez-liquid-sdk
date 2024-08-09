import BreezSDKLiquid
import Foundation

enum BreezSDKLiquidMapper {
    static func asAesSuccessActionDataDecrypted(aesSuccessActionDataDecrypted: [String: Any?]) throws -> AesSuccessActionDataDecrypted {
        guard let description = aesSuccessActionDataDecrypted["description"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "description", typeName: "AesSuccessActionDataDecrypted"))
        }
        guard let plaintext = aesSuccessActionDataDecrypted["plaintext"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "plaintext", typeName: "AesSuccessActionDataDecrypted"))
        }

        return AesSuccessActionDataDecrypted(
            description: description,
            plaintext: plaintext
        )
    }

    static func dictionaryOf(aesSuccessActionDataDecrypted: AesSuccessActionDataDecrypted) -> [String: Any?] {
        return [
            "description": aesSuccessActionDataDecrypted.description,
            "plaintext": aesSuccessActionDataDecrypted.plaintext,
        ]
    }

    static func asAesSuccessActionDataDecryptedList(arr: [Any]) throws -> [AesSuccessActionDataDecrypted] {
        var list = [AesSuccessActionDataDecrypted]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var aesSuccessActionDataDecrypted = try asAesSuccessActionDataDecrypted(aesSuccessActionDataDecrypted: val)
                list.append(aesSuccessActionDataDecrypted)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "AesSuccessActionDataDecrypted"))
            }
        }
        return list
    }

    static func arrayOf(aesSuccessActionDataDecryptedList: [AesSuccessActionDataDecrypted]) -> [Any] {
        return aesSuccessActionDataDecryptedList.map { v -> [String: Any?] in return dictionaryOf(aesSuccessActionDataDecrypted: v) }
    }

    static func asBackupRequest(backupRequest: [String: Any?]) throws -> BackupRequest {
        var backupPath: String?
        if hasNonNilKey(data: backupRequest, key: "backupPath") {
            guard let backupPathTmp = backupRequest["backupPath"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "backupPath"))
            }
            backupPath = backupPathTmp
        }

        return BackupRequest(
            backupPath: backupPath)
    }

    static func dictionaryOf(backupRequest: BackupRequest) -> [String: Any?] {
        return [
            "backupPath": backupRequest.backupPath == nil ? nil : backupRequest.backupPath,
        ]
    }

    static func asBackupRequestList(arr: [Any]) throws -> [BackupRequest] {
        var list = [BackupRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var backupRequest = try asBackupRequest(backupRequest: val)
                list.append(backupRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "BackupRequest"))
            }
        }
        return list
    }

    static func arrayOf(backupRequestList: [BackupRequest]) -> [Any] {
        return backupRequestList.map { v -> [String: Any?] in return dictionaryOf(backupRequest: v) }
    }

    static func asBitcoinAddressData(bitcoinAddressData: [String: Any?]) throws -> BitcoinAddressData {
        guard let address = bitcoinAddressData["address"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "address", typeName: "BitcoinAddressData"))
        }
        guard let networkTmp = bitcoinAddressData["network"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "network", typeName: "BitcoinAddressData"))
        }
        let network = try asNetwork(network: networkTmp)

        var amountSat: UInt64?
        if hasNonNilKey(data: bitcoinAddressData, key: "amountSat") {
            guard let amountSatTmp = bitcoinAddressData["amountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "amountSat"))
            }
            amountSat = amountSatTmp
        }
        var label: String?
        if hasNonNilKey(data: bitcoinAddressData, key: "label") {
            guard let labelTmp = bitcoinAddressData["label"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "label"))
            }
            label = labelTmp
        }
        var message: String?
        if hasNonNilKey(data: bitcoinAddressData, key: "message") {
            guard let messageTmp = bitcoinAddressData["message"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "message"))
            }
            message = messageTmp
        }

        return BitcoinAddressData(
            address: address,
            network: network,
            amountSat: amountSat,
            label: label,
            message: message
        )
    }

    static func dictionaryOf(bitcoinAddressData: BitcoinAddressData) -> [String: Any?] {
        return [
            "address": bitcoinAddressData.address,
            "network": valueOf(network: bitcoinAddressData.network),
            "amountSat": bitcoinAddressData.amountSat == nil ? nil : bitcoinAddressData.amountSat,
            "label": bitcoinAddressData.label == nil ? nil : bitcoinAddressData.label,
            "message": bitcoinAddressData.message == nil ? nil : bitcoinAddressData.message,
        ]
    }

    static func asBitcoinAddressDataList(arr: [Any]) throws -> [BitcoinAddressData] {
        var list = [BitcoinAddressData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var bitcoinAddressData = try asBitcoinAddressData(bitcoinAddressData: val)
                list.append(bitcoinAddressData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "BitcoinAddressData"))
            }
        }
        return list
    }

    static func arrayOf(bitcoinAddressDataList: [BitcoinAddressData]) -> [Any] {
        return bitcoinAddressDataList.map { v -> [String: Any?] in return dictionaryOf(bitcoinAddressData: v) }
    }

    static func asBuyBitcoinRequest(buyBitcoinRequest: [String: Any?]) throws -> BuyBitcoinRequest {
        guard let prepareResTmp = buyBitcoinRequest["prepareRes"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "prepareRes", typeName: "BuyBitcoinRequest"))
        }
        let prepareRes = try asPrepareBuyBitcoinResponse(prepareBuyBitcoinResponse: prepareResTmp)

        var redirectUrl: String?
        if hasNonNilKey(data: buyBitcoinRequest, key: "redirectUrl") {
            guard let redirectUrlTmp = buyBitcoinRequest["redirectUrl"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "redirectUrl"))
            }
            redirectUrl = redirectUrlTmp
        }

        return BuyBitcoinRequest(
            prepareRes: prepareRes,
            redirectUrl: redirectUrl
        )
    }

    static func dictionaryOf(buyBitcoinRequest: BuyBitcoinRequest) -> [String: Any?] {
        return [
            "prepareRes": dictionaryOf(prepareBuyBitcoinResponse: buyBitcoinRequest.prepareRes),
            "redirectUrl": buyBitcoinRequest.redirectUrl == nil ? nil : buyBitcoinRequest.redirectUrl,
        ]
    }

    static func asBuyBitcoinRequestList(arr: [Any]) throws -> [BuyBitcoinRequest] {
        var list = [BuyBitcoinRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var buyBitcoinRequest = try asBuyBitcoinRequest(buyBitcoinRequest: val)
                list.append(buyBitcoinRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "BuyBitcoinRequest"))
            }
        }
        return list
    }

    static func arrayOf(buyBitcoinRequestList: [BuyBitcoinRequest]) -> [Any] {
        return buyBitcoinRequestList.map { v -> [String: Any?] in return dictionaryOf(buyBitcoinRequest: v) }
    }

    static func asConfig(config: [String: Any?]) throws -> Config {
        guard let liquidElectrumUrl = config["liquidElectrumUrl"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "liquidElectrumUrl", typeName: "Config"))
        }
        guard let bitcoinElectrumUrl = config["bitcoinElectrumUrl"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "bitcoinElectrumUrl", typeName: "Config"))
        }
        guard let mempoolspaceUrl = config["mempoolspaceUrl"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "mempoolspaceUrl", typeName: "Config"))
        }
        guard let workingDir = config["workingDir"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "workingDir", typeName: "Config"))
        }
        guard let networkTmp = config["network"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "network", typeName: "Config"))
        }
        let network = try asLiquidNetwork(liquidNetwork: networkTmp)

        guard let paymentTimeoutSec = config["paymentTimeoutSec"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentTimeoutSec", typeName: "Config"))
        }
        guard let zeroConfMinFeeRateMsat = config["zeroConfMinFeeRateMsat"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "zeroConfMinFeeRateMsat", typeName: "Config"))
        }
        var zeroConfMaxAmountSat: UInt64?
        if hasNonNilKey(data: config, key: "zeroConfMaxAmountSat") {
            guard let zeroConfMaxAmountSatTmp = config["zeroConfMaxAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "zeroConfMaxAmountSat"))
            }
            zeroConfMaxAmountSat = zeroConfMaxAmountSatTmp
        }

        return Config(
            liquidElectrumUrl: liquidElectrumUrl,
            bitcoinElectrumUrl: bitcoinElectrumUrl,
            mempoolspaceUrl: mempoolspaceUrl,
            workingDir: workingDir,
            network: network,
            paymentTimeoutSec: paymentTimeoutSec,
            zeroConfMinFeeRateMsat: zeroConfMinFeeRateMsat,
            zeroConfMaxAmountSat: zeroConfMaxAmountSat
        )
    }

    static func dictionaryOf(config: Config) -> [String: Any?] {
        return [
            "liquidElectrumUrl": config.liquidElectrumUrl,
            "bitcoinElectrumUrl": config.bitcoinElectrumUrl,
            "mempoolspaceUrl": config.mempoolspaceUrl,
            "workingDir": config.workingDir,
            "network": valueOf(liquidNetwork: config.network),
            "paymentTimeoutSec": config.paymentTimeoutSec,
            "zeroConfMinFeeRateMsat": config.zeroConfMinFeeRateMsat,
            "zeroConfMaxAmountSat": config.zeroConfMaxAmountSat == nil ? nil : config.zeroConfMaxAmountSat,
        ]
    }

    static func asConfigList(arr: [Any]) throws -> [Config] {
        var list = [Config]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var config = try asConfig(config: val)
                list.append(config)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Config"))
            }
        }
        return list
    }

    static func arrayOf(configList: [Config]) -> [Any] {
        return configList.map { v -> [String: Any?] in return dictionaryOf(config: v) }
    }

    static func asConnectRequest(connectRequest: [String: Any?]) throws -> ConnectRequest {
        guard let configTmp = connectRequest["config"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "config", typeName: "ConnectRequest"))
        }
        let config = try asConfig(config: configTmp)

        guard let mnemonic = connectRequest["mnemonic"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "mnemonic", typeName: "ConnectRequest"))
        }

        return ConnectRequest(
            config: config,
            mnemonic: mnemonic
        )
    }

    static func dictionaryOf(connectRequest: ConnectRequest) -> [String: Any?] {
        return [
            "config": dictionaryOf(config: connectRequest.config),
            "mnemonic": connectRequest.mnemonic,
        ]
    }

    static func asConnectRequestList(arr: [Any]) throws -> [ConnectRequest] {
        var list = [ConnectRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var connectRequest = try asConnectRequest(connectRequest: val)
                list.append(connectRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ConnectRequest"))
            }
        }
        return list
    }

    static func arrayOf(connectRequestList: [ConnectRequest]) -> [Any] {
        return connectRequestList.map { v -> [String: Any?] in return dictionaryOf(connectRequest: v) }
    }

    static func asCurrencyInfo(currencyInfo: [String: Any?]) throws -> CurrencyInfo {
        guard let name = currencyInfo["name"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "name", typeName: "CurrencyInfo"))
        }
        guard let fractionSize = currencyInfo["fractionSize"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "fractionSize", typeName: "CurrencyInfo"))
        }
        var spacing: UInt32?
        if hasNonNilKey(data: currencyInfo, key: "spacing") {
            guard let spacingTmp = currencyInfo["spacing"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "spacing"))
            }
            spacing = spacingTmp
        }
        var symbol: Symbol?
        if let symbolTmp = currencyInfo["symbol"] as? [String: Any?] {
            symbol = try asSymbol(symbol: symbolTmp)
        }

        var uniqSymbol: Symbol?
        if let uniqSymbolTmp = currencyInfo["uniqSymbol"] as? [String: Any?] {
            uniqSymbol = try asSymbol(symbol: uniqSymbolTmp)
        }

        guard let localizedNameTmp = currencyInfo["localizedName"] as? [[String: Any?]] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "localizedName", typeName: "CurrencyInfo"))
        }
        let localizedName = try asLocalizedNameList(arr: localizedNameTmp)

        guard let localeOverridesTmp = currencyInfo["localeOverrides"] as? [[String: Any?]] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "localeOverrides", typeName: "CurrencyInfo"))
        }
        let localeOverrides = try asLocaleOverridesList(arr: localeOverridesTmp)

        return CurrencyInfo(
            name: name,
            fractionSize: fractionSize,
            spacing: spacing,
            symbol: symbol,
            uniqSymbol: uniqSymbol,
            localizedName: localizedName,
            localeOverrides: localeOverrides
        )
    }

    static func dictionaryOf(currencyInfo: CurrencyInfo) -> [String: Any?] {
        return [
            "name": currencyInfo.name,
            "fractionSize": currencyInfo.fractionSize,
            "spacing": currencyInfo.spacing == nil ? nil : currencyInfo.spacing,
            "symbol": currencyInfo.symbol == nil ? nil : dictionaryOf(symbol: currencyInfo.symbol!),
            "uniqSymbol": currencyInfo.uniqSymbol == nil ? nil : dictionaryOf(symbol: currencyInfo.uniqSymbol!),
            "localizedName": arrayOf(localizedNameList: currencyInfo.localizedName),
            "localeOverrides": arrayOf(localeOverridesList: currencyInfo.localeOverrides),
        ]
    }

    static func asCurrencyInfoList(arr: [Any]) throws -> [CurrencyInfo] {
        var list = [CurrencyInfo]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var currencyInfo = try asCurrencyInfo(currencyInfo: val)
                list.append(currencyInfo)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "CurrencyInfo"))
            }
        }
        return list
    }

    static func arrayOf(currencyInfoList: [CurrencyInfo]) -> [Any] {
        return currencyInfoList.map { v -> [String: Any?] in return dictionaryOf(currencyInfo: v) }
    }

    static func asFiatCurrency(fiatCurrency: [String: Any?]) throws -> FiatCurrency {
        guard let id = fiatCurrency["id"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "id", typeName: "FiatCurrency"))
        }
        guard let infoTmp = fiatCurrency["info"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "info", typeName: "FiatCurrency"))
        }
        let info = try asCurrencyInfo(currencyInfo: infoTmp)

        return FiatCurrency(
            id: id,
            info: info
        )
    }

    static func dictionaryOf(fiatCurrency: FiatCurrency) -> [String: Any?] {
        return [
            "id": fiatCurrency.id,
            "info": dictionaryOf(currencyInfo: fiatCurrency.info),
        ]
    }

    static func asFiatCurrencyList(arr: [Any]) throws -> [FiatCurrency] {
        var list = [FiatCurrency]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var fiatCurrency = try asFiatCurrency(fiatCurrency: val)
                list.append(fiatCurrency)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "FiatCurrency"))
            }
        }
        return list
    }

    static func arrayOf(fiatCurrencyList: [FiatCurrency]) -> [Any] {
        return fiatCurrencyList.map { v -> [String: Any?] in return dictionaryOf(fiatCurrency: v) }
    }

    static func asGetInfoResponse(getInfoResponse: [String: Any?]) throws -> GetInfoResponse {
        guard let balanceSat = getInfoResponse["balanceSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "balanceSat", typeName: "GetInfoResponse"))
        }
        guard let pendingSendSat = getInfoResponse["pendingSendSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "pendingSendSat", typeName: "GetInfoResponse"))
        }
        guard let pendingReceiveSat = getInfoResponse["pendingReceiveSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "pendingReceiveSat", typeName: "GetInfoResponse"))
        }
        guard let pubkey = getInfoResponse["pubkey"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "pubkey", typeName: "GetInfoResponse"))
        }

        return GetInfoResponse(
            balanceSat: balanceSat,
            pendingSendSat: pendingSendSat,
            pendingReceiveSat: pendingReceiveSat,
            pubkey: pubkey
        )
    }

    static func dictionaryOf(getInfoResponse: GetInfoResponse) -> [String: Any?] {
        return [
            "balanceSat": getInfoResponse.balanceSat,
            "pendingSendSat": getInfoResponse.pendingSendSat,
            "pendingReceiveSat": getInfoResponse.pendingReceiveSat,
            "pubkey": getInfoResponse.pubkey,
        ]
    }

    static func asGetInfoResponseList(arr: [Any]) throws -> [GetInfoResponse] {
        var list = [GetInfoResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var getInfoResponse = try asGetInfoResponse(getInfoResponse: val)
                list.append(getInfoResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "GetInfoResponse"))
            }
        }
        return list
    }

    static func arrayOf(getInfoResponseList: [GetInfoResponse]) -> [Any] {
        return getInfoResponseList.map { v -> [String: Any?] in return dictionaryOf(getInfoResponse: v) }
    }

    static func asLnInvoice(lnInvoice: [String: Any?]) throws -> LnInvoice {
        guard let bolt11 = lnInvoice["bolt11"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "bolt11", typeName: "LnInvoice"))
        }
        guard let networkTmp = lnInvoice["network"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "network", typeName: "LnInvoice"))
        }
        let network = try asNetwork(network: networkTmp)

        guard let payeePubkey = lnInvoice["payeePubkey"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payeePubkey", typeName: "LnInvoice"))
        }
        guard let paymentHash = lnInvoice["paymentHash"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentHash", typeName: "LnInvoice"))
        }
        var description: String?
        if hasNonNilKey(data: lnInvoice, key: "description") {
            guard let descriptionTmp = lnInvoice["description"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "description"))
            }
            description = descriptionTmp
        }
        var descriptionHash: String?
        if hasNonNilKey(data: lnInvoice, key: "descriptionHash") {
            guard let descriptionHashTmp = lnInvoice["descriptionHash"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "descriptionHash"))
            }
            descriptionHash = descriptionHashTmp
        }
        var amountMsat: UInt64?
        if hasNonNilKey(data: lnInvoice, key: "amountMsat") {
            guard let amountMsatTmp = lnInvoice["amountMsat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "amountMsat"))
            }
            amountMsat = amountMsatTmp
        }
        guard let timestamp = lnInvoice["timestamp"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "timestamp", typeName: "LnInvoice"))
        }
        guard let expiry = lnInvoice["expiry"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "expiry", typeName: "LnInvoice"))
        }
        guard let routingHintsTmp = lnInvoice["routingHints"] as? [[String: Any?]] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "routingHints", typeName: "LnInvoice"))
        }
        let routingHints = try asRouteHintList(arr: routingHintsTmp)

        guard let paymentSecret = lnInvoice["paymentSecret"] as? [UInt8] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentSecret", typeName: "LnInvoice"))
        }
        guard let minFinalCltvExpiryDelta = lnInvoice["minFinalCltvExpiryDelta"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "minFinalCltvExpiryDelta", typeName: "LnInvoice"))
        }

        return LnInvoice(
            bolt11: bolt11,
            network: network,
            payeePubkey: payeePubkey,
            paymentHash: paymentHash,
            description: description,
            descriptionHash: descriptionHash,
            amountMsat: amountMsat,
            timestamp: timestamp,
            expiry: expiry,
            routingHints: routingHints,
            paymentSecret: paymentSecret,
            minFinalCltvExpiryDelta: minFinalCltvExpiryDelta
        )
    }

    static func dictionaryOf(lnInvoice: LnInvoice) -> [String: Any?] {
        return [
            "bolt11": lnInvoice.bolt11,
            "network": valueOf(network: lnInvoice.network),
            "payeePubkey": lnInvoice.payeePubkey,
            "paymentHash": lnInvoice.paymentHash,
            "description": lnInvoice.description == nil ? nil : lnInvoice.description,
            "descriptionHash": lnInvoice.descriptionHash == nil ? nil : lnInvoice.descriptionHash,
            "amountMsat": lnInvoice.amountMsat == nil ? nil : lnInvoice.amountMsat,
            "timestamp": lnInvoice.timestamp,
            "expiry": lnInvoice.expiry,
            "routingHints": arrayOf(routeHintList: lnInvoice.routingHints),
            "paymentSecret": lnInvoice.paymentSecret,
            "minFinalCltvExpiryDelta": lnInvoice.minFinalCltvExpiryDelta,
        ]
    }

    static func asLnInvoiceList(arr: [Any]) throws -> [LnInvoice] {
        var list = [LnInvoice]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnInvoice = try asLnInvoice(lnInvoice: val)
                list.append(lnInvoice)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnInvoice"))
            }
        }
        return list
    }

    static func arrayOf(lnInvoiceList: [LnInvoice]) -> [Any] {
        return lnInvoiceList.map { v -> [String: Any?] in return dictionaryOf(lnInvoice: v) }
    }

    static func asLightningPaymentLimitsResponse(lightningPaymentLimitsResponse: [String: Any?]) throws -> LightningPaymentLimitsResponse {
        guard let sendTmp = lightningPaymentLimitsResponse["send"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "send", typeName: "LightningPaymentLimitsResponse"))
        }
        let send = try asLimits(limits: sendTmp)

        guard let receiveTmp = lightningPaymentLimitsResponse["receive"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receive", typeName: "LightningPaymentLimitsResponse"))
        }
        let receive = try asLimits(limits: receiveTmp)

        return LightningPaymentLimitsResponse(
            send: send,
            receive: receive
        )
    }

    static func dictionaryOf(lightningPaymentLimitsResponse: LightningPaymentLimitsResponse) -> [String: Any?] {
        return [
            "send": dictionaryOf(limits: lightningPaymentLimitsResponse.send),
            "receive": dictionaryOf(limits: lightningPaymentLimitsResponse.receive),
        ]
    }

    static func asLightningPaymentLimitsResponseList(arr: [Any]) throws -> [LightningPaymentLimitsResponse] {
        var list = [LightningPaymentLimitsResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lightningPaymentLimitsResponse = try asLightningPaymentLimitsResponse(lightningPaymentLimitsResponse: val)
                list.append(lightningPaymentLimitsResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LightningPaymentLimitsResponse"))
            }
        }
        return list
    }

    static func arrayOf(lightningPaymentLimitsResponseList: [LightningPaymentLimitsResponse]) -> [Any] {
        return lightningPaymentLimitsResponseList.map { v -> [String: Any?] in return dictionaryOf(lightningPaymentLimitsResponse: v) }
    }

    static func asLimits(limits: [String: Any?]) throws -> Limits {
        guard let minSat = limits["minSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "minSat", typeName: "Limits"))
        }
        guard let maxSat = limits["maxSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "maxSat", typeName: "Limits"))
        }
        guard let maxZeroConfSat = limits["maxZeroConfSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "maxZeroConfSat", typeName: "Limits"))
        }

        return Limits(
            minSat: minSat,
            maxSat: maxSat,
            maxZeroConfSat: maxZeroConfSat
        )
    }

    static func dictionaryOf(limits: Limits) -> [String: Any?] {
        return [
            "minSat": limits.minSat,
            "maxSat": limits.maxSat,
            "maxZeroConfSat": limits.maxZeroConfSat,
        ]
    }

    static func asLimitsList(arr: [Any]) throws -> [Limits] {
        var list = [Limits]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var limits = try asLimits(limits: val)
                list.append(limits)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Limits"))
            }
        }
        return list
    }

    static func arrayOf(limitsList: [Limits]) -> [Any] {
        return limitsList.map { v -> [String: Any?] in return dictionaryOf(limits: v) }
    }

    static func asListPaymentsRequest(listPaymentsRequest: [String: Any?]) throws -> ListPaymentsRequest {
        var filters: [PaymentType]?
        if let filtersTmp = listPaymentsRequest["filters"] as? [String] {
            filters = try asPaymentTypeList(arr: filtersTmp)
        }

        var fromTimestamp: Int64?
        if hasNonNilKey(data: listPaymentsRequest, key: "fromTimestamp") {
            guard let fromTimestampTmp = listPaymentsRequest["fromTimestamp"] as? Int64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "fromTimestamp"))
            }
            fromTimestamp = fromTimestampTmp
        }
        var toTimestamp: Int64?
        if hasNonNilKey(data: listPaymentsRequest, key: "toTimestamp") {
            guard let toTimestampTmp = listPaymentsRequest["toTimestamp"] as? Int64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "toTimestamp"))
            }
            toTimestamp = toTimestampTmp
        }
        var offset: UInt32?
        if hasNonNilKey(data: listPaymentsRequest, key: "offset") {
            guard let offsetTmp = listPaymentsRequest["offset"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "offset"))
            }
            offset = offsetTmp
        }
        var limit: UInt32?
        if hasNonNilKey(data: listPaymentsRequest, key: "limit") {
            guard let limitTmp = listPaymentsRequest["limit"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "limit"))
            }
            limit = limitTmp
        }

        return ListPaymentsRequest(
            filters: filters,
            fromTimestamp: fromTimestamp,
            toTimestamp: toTimestamp,
            offset: offset,
            limit: limit
        )
    }

    static func dictionaryOf(listPaymentsRequest: ListPaymentsRequest) -> [String: Any?] {
        return [
            "filters": listPaymentsRequest.filters == nil ? nil : arrayOf(paymentTypeList: listPaymentsRequest.filters!),
            "fromTimestamp": listPaymentsRequest.fromTimestamp == nil ? nil : listPaymentsRequest.fromTimestamp,
            "toTimestamp": listPaymentsRequest.toTimestamp == nil ? nil : listPaymentsRequest.toTimestamp,
            "offset": listPaymentsRequest.offset == nil ? nil : listPaymentsRequest.offset,
            "limit": listPaymentsRequest.limit == nil ? nil : listPaymentsRequest.limit,
        ]
    }

    static func asListPaymentsRequestList(arr: [Any]) throws -> [ListPaymentsRequest] {
        var list = [ListPaymentsRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var listPaymentsRequest = try asListPaymentsRequest(listPaymentsRequest: val)
                list.append(listPaymentsRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ListPaymentsRequest"))
            }
        }
        return list
    }

    static func arrayOf(listPaymentsRequestList: [ListPaymentsRequest]) -> [Any] {
        return listPaymentsRequestList.map { v -> [String: Any?] in return dictionaryOf(listPaymentsRequest: v) }
    }

    static func asLnUrlAuthRequestData(lnUrlAuthRequestData: [String: Any?]) throws -> LnUrlAuthRequestData {
        guard let k1 = lnUrlAuthRequestData["k1"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "k1", typeName: "LnUrlAuthRequestData"))
        }
        guard let domain = lnUrlAuthRequestData["domain"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "domain", typeName: "LnUrlAuthRequestData"))
        }
        guard let url = lnUrlAuthRequestData["url"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "url", typeName: "LnUrlAuthRequestData"))
        }
        var action: String?
        if hasNonNilKey(data: lnUrlAuthRequestData, key: "action") {
            guard let actionTmp = lnUrlAuthRequestData["action"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "action"))
            }
            action = actionTmp
        }

        return LnUrlAuthRequestData(
            k1: k1,
            domain: domain,
            url: url,
            action: action
        )
    }

    static func dictionaryOf(lnUrlAuthRequestData: LnUrlAuthRequestData) -> [String: Any?] {
        return [
            "k1": lnUrlAuthRequestData.k1,
            "domain": lnUrlAuthRequestData.domain,
            "url": lnUrlAuthRequestData.url,
            "action": lnUrlAuthRequestData.action == nil ? nil : lnUrlAuthRequestData.action,
        ]
    }

    static func asLnUrlAuthRequestDataList(arr: [Any]) throws -> [LnUrlAuthRequestData] {
        var list = [LnUrlAuthRequestData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlAuthRequestData = try asLnUrlAuthRequestData(lnUrlAuthRequestData: val)
                list.append(lnUrlAuthRequestData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlAuthRequestData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlAuthRequestDataList: [LnUrlAuthRequestData]) -> [Any] {
        return lnUrlAuthRequestDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlAuthRequestData: v) }
    }

    static func asLnUrlErrorData(lnUrlErrorData: [String: Any?]) throws -> LnUrlErrorData {
        guard let reason = lnUrlErrorData["reason"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "reason", typeName: "LnUrlErrorData"))
        }

        return LnUrlErrorData(
            reason: reason)
    }

    static func dictionaryOf(lnUrlErrorData: LnUrlErrorData) -> [String: Any?] {
        return [
            "reason": lnUrlErrorData.reason,
        ]
    }

    static func asLnUrlErrorDataList(arr: [Any]) throws -> [LnUrlErrorData] {
        var list = [LnUrlErrorData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlErrorData = try asLnUrlErrorData(lnUrlErrorData: val)
                list.append(lnUrlErrorData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlErrorData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlErrorDataList: [LnUrlErrorData]) -> [Any] {
        return lnUrlErrorDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlErrorData: v) }
    }

    static func asLnUrlPayErrorData(lnUrlPayErrorData: [String: Any?]) throws -> LnUrlPayErrorData {
        guard let paymentHash = lnUrlPayErrorData["paymentHash"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentHash", typeName: "LnUrlPayErrorData"))
        }
        guard let reason = lnUrlPayErrorData["reason"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "reason", typeName: "LnUrlPayErrorData"))
        }

        return LnUrlPayErrorData(
            paymentHash: paymentHash,
            reason: reason
        )
    }

    static func dictionaryOf(lnUrlPayErrorData: LnUrlPayErrorData) -> [String: Any?] {
        return [
            "paymentHash": lnUrlPayErrorData.paymentHash,
            "reason": lnUrlPayErrorData.reason,
        ]
    }

    static func asLnUrlPayErrorDataList(arr: [Any]) throws -> [LnUrlPayErrorData] {
        var list = [LnUrlPayErrorData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlPayErrorData = try asLnUrlPayErrorData(lnUrlPayErrorData: val)
                list.append(lnUrlPayErrorData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlPayErrorData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlPayErrorDataList: [LnUrlPayErrorData]) -> [Any] {
        return lnUrlPayErrorDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlPayErrorData: v) }
    }

    static func asLnUrlPayRequest(lnUrlPayRequest: [String: Any?]) throws -> LnUrlPayRequest {
        guard let dataTmp = lnUrlPayRequest["data"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlPayRequest"))
        }
        let data = try asLnUrlPayRequestData(lnUrlPayRequestData: dataTmp)

        guard let amountMsat = lnUrlPayRequest["amountMsat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountMsat", typeName: "LnUrlPayRequest"))
        }
        var comment: String?
        if hasNonNilKey(data: lnUrlPayRequest, key: "comment") {
            guard let commentTmp = lnUrlPayRequest["comment"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "comment"))
            }
            comment = commentTmp
        }
        var paymentLabel: String?
        if hasNonNilKey(data: lnUrlPayRequest, key: "paymentLabel") {
            guard let paymentLabelTmp = lnUrlPayRequest["paymentLabel"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "paymentLabel"))
            }
            paymentLabel = paymentLabelTmp
        }
        var validateSuccessActionUrl: Bool?
        if hasNonNilKey(data: lnUrlPayRequest, key: "validateSuccessActionUrl") {
            guard let validateSuccessActionUrlTmp = lnUrlPayRequest["validateSuccessActionUrl"] as? Bool else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "validateSuccessActionUrl"))
            }
            validateSuccessActionUrl = validateSuccessActionUrlTmp
        }

        return LnUrlPayRequest(
            data: data,
            amountMsat: amountMsat,
            comment: comment,
            paymentLabel: paymentLabel,
            validateSuccessActionUrl: validateSuccessActionUrl
        )
    }

    static func dictionaryOf(lnUrlPayRequest: LnUrlPayRequest) -> [String: Any?] {
        return [
            "data": dictionaryOf(lnUrlPayRequestData: lnUrlPayRequest.data),
            "amountMsat": lnUrlPayRequest.amountMsat,
            "comment": lnUrlPayRequest.comment == nil ? nil : lnUrlPayRequest.comment,
            "paymentLabel": lnUrlPayRequest.paymentLabel == nil ? nil : lnUrlPayRequest.paymentLabel,
            "validateSuccessActionUrl": lnUrlPayRequest.validateSuccessActionUrl == nil ? nil : lnUrlPayRequest.validateSuccessActionUrl,
        ]
    }

    static func asLnUrlPayRequestList(arr: [Any]) throws -> [LnUrlPayRequest] {
        var list = [LnUrlPayRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlPayRequest = try asLnUrlPayRequest(lnUrlPayRequest: val)
                list.append(lnUrlPayRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlPayRequest"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlPayRequestList: [LnUrlPayRequest]) -> [Any] {
        return lnUrlPayRequestList.map { v -> [String: Any?] in return dictionaryOf(lnUrlPayRequest: v) }
    }

    static func asLnUrlPayRequestData(lnUrlPayRequestData: [String: Any?]) throws -> LnUrlPayRequestData {
        guard let callback = lnUrlPayRequestData["callback"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "callback", typeName: "LnUrlPayRequestData"))
        }
        guard let minSendable = lnUrlPayRequestData["minSendable"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "minSendable", typeName: "LnUrlPayRequestData"))
        }
        guard let maxSendable = lnUrlPayRequestData["maxSendable"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "maxSendable", typeName: "LnUrlPayRequestData"))
        }
        guard let metadataStr = lnUrlPayRequestData["metadataStr"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "metadataStr", typeName: "LnUrlPayRequestData"))
        }
        guard let commentAllowed = lnUrlPayRequestData["commentAllowed"] as? UInt16 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "commentAllowed", typeName: "LnUrlPayRequestData"))
        }
        guard let domain = lnUrlPayRequestData["domain"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "domain", typeName: "LnUrlPayRequestData"))
        }
        guard let allowsNostr = lnUrlPayRequestData["allowsNostr"] as? Bool else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "allowsNostr", typeName: "LnUrlPayRequestData"))
        }
        var nostrPubkey: String?
        if hasNonNilKey(data: lnUrlPayRequestData, key: "nostrPubkey") {
            guard let nostrPubkeyTmp = lnUrlPayRequestData["nostrPubkey"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "nostrPubkey"))
            }
            nostrPubkey = nostrPubkeyTmp
        }
        var lnAddress: String?
        if hasNonNilKey(data: lnUrlPayRequestData, key: "lnAddress") {
            guard let lnAddressTmp = lnUrlPayRequestData["lnAddress"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "lnAddress"))
            }
            lnAddress = lnAddressTmp
        }

        return LnUrlPayRequestData(
            callback: callback,
            minSendable: minSendable,
            maxSendable: maxSendable,
            metadataStr: metadataStr,
            commentAllowed: commentAllowed,
            domain: domain,
            allowsNostr: allowsNostr,
            nostrPubkey: nostrPubkey,
            lnAddress: lnAddress
        )
    }

    static func dictionaryOf(lnUrlPayRequestData: LnUrlPayRequestData) -> [String: Any?] {
        return [
            "callback": lnUrlPayRequestData.callback,
            "minSendable": lnUrlPayRequestData.minSendable,
            "maxSendable": lnUrlPayRequestData.maxSendable,
            "metadataStr": lnUrlPayRequestData.metadataStr,
            "commentAllowed": lnUrlPayRequestData.commentAllowed,
            "domain": lnUrlPayRequestData.domain,
            "allowsNostr": lnUrlPayRequestData.allowsNostr,
            "nostrPubkey": lnUrlPayRequestData.nostrPubkey == nil ? nil : lnUrlPayRequestData.nostrPubkey,
            "lnAddress": lnUrlPayRequestData.lnAddress == nil ? nil : lnUrlPayRequestData.lnAddress,
        ]
    }

    static func asLnUrlPayRequestDataList(arr: [Any]) throws -> [LnUrlPayRequestData] {
        var list = [LnUrlPayRequestData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlPayRequestData = try asLnUrlPayRequestData(lnUrlPayRequestData: val)
                list.append(lnUrlPayRequestData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlPayRequestData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlPayRequestDataList: [LnUrlPayRequestData]) -> [Any] {
        return lnUrlPayRequestDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlPayRequestData: v) }
    }

    static func asLnUrlPaySuccessData(lnUrlPaySuccessData: [String: Any?]) throws -> LnUrlPaySuccessData {
        var successAction: SuccessActionProcessed?
        if let successActionTmp = lnUrlPaySuccessData["successAction"] as? [String: Any?] {
            successAction = try asSuccessActionProcessed(successActionProcessed: successActionTmp)
        }

        guard let paymentTmp = lnUrlPaySuccessData["payment"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payment", typeName: "LnUrlPaySuccessData"))
        }
        let payment = try asPayment(payment: paymentTmp)

        return LnUrlPaySuccessData(
            successAction: successAction,
            payment: payment
        )
    }

    static func dictionaryOf(lnUrlPaySuccessData: LnUrlPaySuccessData) -> [String: Any?] {
        return [
            "successAction": lnUrlPaySuccessData.successAction == nil ? nil : dictionaryOf(successActionProcessed: lnUrlPaySuccessData.successAction!),
            "payment": dictionaryOf(payment: lnUrlPaySuccessData.payment),
        ]
    }

    static func asLnUrlPaySuccessDataList(arr: [Any]) throws -> [LnUrlPaySuccessData] {
        var list = [LnUrlPaySuccessData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlPaySuccessData = try asLnUrlPaySuccessData(lnUrlPaySuccessData: val)
                list.append(lnUrlPaySuccessData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlPaySuccessData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlPaySuccessDataList: [LnUrlPaySuccessData]) -> [Any] {
        return lnUrlPaySuccessDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlPaySuccessData: v) }
    }

    static func asLnUrlWithdrawRequest(lnUrlWithdrawRequest: [String: Any?]) throws -> LnUrlWithdrawRequest {
        guard let dataTmp = lnUrlWithdrawRequest["data"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlWithdrawRequest"))
        }
        let data = try asLnUrlWithdrawRequestData(lnUrlWithdrawRequestData: dataTmp)

        guard let amountMsat = lnUrlWithdrawRequest["amountMsat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountMsat", typeName: "LnUrlWithdrawRequest"))
        }
        var description: String?
        if hasNonNilKey(data: lnUrlWithdrawRequest, key: "description") {
            guard let descriptionTmp = lnUrlWithdrawRequest["description"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "description"))
            }
            description = descriptionTmp
        }

        return LnUrlWithdrawRequest(
            data: data,
            amountMsat: amountMsat,
            description: description
        )
    }

    static func dictionaryOf(lnUrlWithdrawRequest: LnUrlWithdrawRequest) -> [String: Any?] {
        return [
            "data": dictionaryOf(lnUrlWithdrawRequestData: lnUrlWithdrawRequest.data),
            "amountMsat": lnUrlWithdrawRequest.amountMsat,
            "description": lnUrlWithdrawRequest.description == nil ? nil : lnUrlWithdrawRequest.description,
        ]
    }

    static func asLnUrlWithdrawRequestList(arr: [Any]) throws -> [LnUrlWithdrawRequest] {
        var list = [LnUrlWithdrawRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlWithdrawRequest = try asLnUrlWithdrawRequest(lnUrlWithdrawRequest: val)
                list.append(lnUrlWithdrawRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlWithdrawRequest"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlWithdrawRequestList: [LnUrlWithdrawRequest]) -> [Any] {
        return lnUrlWithdrawRequestList.map { v -> [String: Any?] in return dictionaryOf(lnUrlWithdrawRequest: v) }
    }

    static func asLnUrlWithdrawRequestData(lnUrlWithdrawRequestData: [String: Any?]) throws -> LnUrlWithdrawRequestData {
        guard let callback = lnUrlWithdrawRequestData["callback"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "callback", typeName: "LnUrlWithdrawRequestData"))
        }
        guard let k1 = lnUrlWithdrawRequestData["k1"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "k1", typeName: "LnUrlWithdrawRequestData"))
        }
        guard let defaultDescription = lnUrlWithdrawRequestData["defaultDescription"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "defaultDescription", typeName: "LnUrlWithdrawRequestData"))
        }
        guard let minWithdrawable = lnUrlWithdrawRequestData["minWithdrawable"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "minWithdrawable", typeName: "LnUrlWithdrawRequestData"))
        }
        guard let maxWithdrawable = lnUrlWithdrawRequestData["maxWithdrawable"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "maxWithdrawable", typeName: "LnUrlWithdrawRequestData"))
        }

        return LnUrlWithdrawRequestData(
            callback: callback,
            k1: k1,
            defaultDescription: defaultDescription,
            minWithdrawable: minWithdrawable,
            maxWithdrawable: maxWithdrawable
        )
    }

    static func dictionaryOf(lnUrlWithdrawRequestData: LnUrlWithdrawRequestData) -> [String: Any?] {
        return [
            "callback": lnUrlWithdrawRequestData.callback,
            "k1": lnUrlWithdrawRequestData.k1,
            "defaultDescription": lnUrlWithdrawRequestData.defaultDescription,
            "minWithdrawable": lnUrlWithdrawRequestData.minWithdrawable,
            "maxWithdrawable": lnUrlWithdrawRequestData.maxWithdrawable,
        ]
    }

    static func asLnUrlWithdrawRequestDataList(arr: [Any]) throws -> [LnUrlWithdrawRequestData] {
        var list = [LnUrlWithdrawRequestData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlWithdrawRequestData = try asLnUrlWithdrawRequestData(lnUrlWithdrawRequestData: val)
                list.append(lnUrlWithdrawRequestData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlWithdrawRequestData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlWithdrawRequestDataList: [LnUrlWithdrawRequestData]) -> [Any] {
        return lnUrlWithdrawRequestDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlWithdrawRequestData: v) }
    }

    static func asLnUrlWithdrawSuccessData(lnUrlWithdrawSuccessData: [String: Any?]) throws -> LnUrlWithdrawSuccessData {
        guard let invoiceTmp = lnUrlWithdrawSuccessData["invoice"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "LnUrlWithdrawSuccessData"))
        }
        let invoice = try asLnInvoice(lnInvoice: invoiceTmp)

        return LnUrlWithdrawSuccessData(
            invoice: invoice)
    }

    static func dictionaryOf(lnUrlWithdrawSuccessData: LnUrlWithdrawSuccessData) -> [String: Any?] {
        return [
            "invoice": dictionaryOf(lnInvoice: lnUrlWithdrawSuccessData.invoice),
        ]
    }

    static func asLnUrlWithdrawSuccessDataList(arr: [Any]) throws -> [LnUrlWithdrawSuccessData] {
        var list = [LnUrlWithdrawSuccessData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlWithdrawSuccessData = try asLnUrlWithdrawSuccessData(lnUrlWithdrawSuccessData: val)
                list.append(lnUrlWithdrawSuccessData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlWithdrawSuccessData"))
            }
        }
        return list
    }

    static func arrayOf(lnUrlWithdrawSuccessDataList: [LnUrlWithdrawSuccessData]) -> [Any] {
        return lnUrlWithdrawSuccessDataList.map { v -> [String: Any?] in return dictionaryOf(lnUrlWithdrawSuccessData: v) }
    }

    static func asLocaleOverrides(localeOverrides: [String: Any?]) throws -> LocaleOverrides {
        guard let locale = localeOverrides["locale"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "locale", typeName: "LocaleOverrides"))
        }
        var spacing: UInt32?
        if hasNonNilKey(data: localeOverrides, key: "spacing") {
            guard let spacingTmp = localeOverrides["spacing"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "spacing"))
            }
            spacing = spacingTmp
        }
        guard let symbolTmp = localeOverrides["symbol"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "symbol", typeName: "LocaleOverrides"))
        }
        let symbol = try asSymbol(symbol: symbolTmp)

        return LocaleOverrides(
            locale: locale,
            spacing: spacing,
            symbol: symbol
        )
    }

    static func dictionaryOf(localeOverrides: LocaleOverrides) -> [String: Any?] {
        return [
            "locale": localeOverrides.locale,
            "spacing": localeOverrides.spacing == nil ? nil : localeOverrides.spacing,
            "symbol": dictionaryOf(symbol: localeOverrides.symbol),
        ]
    }

    static func asLocaleOverridesList(arr: [Any]) throws -> [LocaleOverrides] {
        var list = [LocaleOverrides]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var localeOverrides = try asLocaleOverrides(localeOverrides: val)
                list.append(localeOverrides)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LocaleOverrides"))
            }
        }
        return list
    }

    static func arrayOf(localeOverridesList: [LocaleOverrides]) -> [Any] {
        return localeOverridesList.map { v -> [String: Any?] in return dictionaryOf(localeOverrides: v) }
    }

    static func asLocalizedName(localizedName: [String: Any?]) throws -> LocalizedName {
        guard let locale = localizedName["locale"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "locale", typeName: "LocalizedName"))
        }
        guard let name = localizedName["name"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "name", typeName: "LocalizedName"))
        }

        return LocalizedName(
            locale: locale,
            name: name
        )
    }

    static func dictionaryOf(localizedName: LocalizedName) -> [String: Any?] {
        return [
            "locale": localizedName.locale,
            "name": localizedName.name,
        ]
    }

    static func asLocalizedNameList(arr: [Any]) throws -> [LocalizedName] {
        var list = [LocalizedName]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var localizedName = try asLocalizedName(localizedName: val)
                list.append(localizedName)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LocalizedName"))
            }
        }
        return list
    }

    static func arrayOf(localizedNameList: [LocalizedName]) -> [Any] {
        return localizedNameList.map { v -> [String: Any?] in return dictionaryOf(localizedName: v) }
    }

    static func asLogEntry(logEntry: [String: Any?]) throws -> LogEntry {
        guard let line = logEntry["line"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "line", typeName: "LogEntry"))
        }
        guard let level = logEntry["level"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "level", typeName: "LogEntry"))
        }

        return LogEntry(
            line: line,
            level: level
        )
    }

    static func dictionaryOf(logEntry: LogEntry) -> [String: Any?] {
        return [
            "line": logEntry.line,
            "level": logEntry.level,
        ]
    }

    static func asLogEntryList(arr: [Any]) throws -> [LogEntry] {
        var list = [LogEntry]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var logEntry = try asLogEntry(logEntry: val)
                list.append(logEntry)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LogEntry"))
            }
        }
        return list
    }

    static func arrayOf(logEntryList: [LogEntry]) -> [Any] {
        return logEntryList.map { v -> [String: Any?] in return dictionaryOf(logEntry: v) }
    }

    static func asMessageSuccessActionData(messageSuccessActionData: [String: Any?]) throws -> MessageSuccessActionData {
        guard let message = messageSuccessActionData["message"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "message", typeName: "MessageSuccessActionData"))
        }

        return MessageSuccessActionData(
            message: message)
    }

    static func dictionaryOf(messageSuccessActionData: MessageSuccessActionData) -> [String: Any?] {
        return [
            "message": messageSuccessActionData.message,
        ]
    }

    static func asMessageSuccessActionDataList(arr: [Any]) throws -> [MessageSuccessActionData] {
        var list = [MessageSuccessActionData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var messageSuccessActionData = try asMessageSuccessActionData(messageSuccessActionData: val)
                list.append(messageSuccessActionData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "MessageSuccessActionData"))
            }
        }
        return list
    }

    static func arrayOf(messageSuccessActionDataList: [MessageSuccessActionData]) -> [Any] {
        return messageSuccessActionDataList.map { v -> [String: Any?] in return dictionaryOf(messageSuccessActionData: v) }
    }

    static func asOnchainPaymentLimitsResponse(onchainPaymentLimitsResponse: [String: Any?]) throws -> OnchainPaymentLimitsResponse {
        guard let sendTmp = onchainPaymentLimitsResponse["send"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "send", typeName: "OnchainPaymentLimitsResponse"))
        }
        let send = try asLimits(limits: sendTmp)

        guard let receiveTmp = onchainPaymentLimitsResponse["receive"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receive", typeName: "OnchainPaymentLimitsResponse"))
        }
        let receive = try asLimits(limits: receiveTmp)

        return OnchainPaymentLimitsResponse(
            send: send,
            receive: receive
        )
    }

    static func dictionaryOf(onchainPaymentLimitsResponse: OnchainPaymentLimitsResponse) -> [String: Any?] {
        return [
            "send": dictionaryOf(limits: onchainPaymentLimitsResponse.send),
            "receive": dictionaryOf(limits: onchainPaymentLimitsResponse.receive),
        ]
    }

    static func asOnchainPaymentLimitsResponseList(arr: [Any]) throws -> [OnchainPaymentLimitsResponse] {
        var list = [OnchainPaymentLimitsResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var onchainPaymentLimitsResponse = try asOnchainPaymentLimitsResponse(onchainPaymentLimitsResponse: val)
                list.append(onchainPaymentLimitsResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "OnchainPaymentLimitsResponse"))
            }
        }
        return list
    }

    static func arrayOf(onchainPaymentLimitsResponseList: [OnchainPaymentLimitsResponse]) -> [Any] {
        return onchainPaymentLimitsResponseList.map { v -> [String: Any?] in return dictionaryOf(onchainPaymentLimitsResponse: v) }
    }

    static func asPayOnchainRequest(payOnchainRequest: [String: Any?]) throws -> PayOnchainRequest {
        guard let address = payOnchainRequest["address"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "address", typeName: "PayOnchainRequest"))
        }
        guard let prepareResTmp = payOnchainRequest["prepareRes"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "prepareRes", typeName: "PayOnchainRequest"))
        }
        let prepareRes = try asPreparePayOnchainResponse(preparePayOnchainResponse: prepareResTmp)

        return PayOnchainRequest(
            address: address,
            prepareRes: prepareRes
        )
    }

    static func dictionaryOf(payOnchainRequest: PayOnchainRequest) -> [String: Any?] {
        return [
            "address": payOnchainRequest.address,
            "prepareRes": dictionaryOf(preparePayOnchainResponse: payOnchainRequest.prepareRes),
        ]
    }

    static func asPayOnchainRequestList(arr: [Any]) throws -> [PayOnchainRequest] {
        var list = [PayOnchainRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var payOnchainRequest = try asPayOnchainRequest(payOnchainRequest: val)
                list.append(payOnchainRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PayOnchainRequest"))
            }
        }
        return list
    }

    static func arrayOf(payOnchainRequestList: [PayOnchainRequest]) -> [Any] {
        return payOnchainRequestList.map { v -> [String: Any?] in return dictionaryOf(payOnchainRequest: v) }
    }

    static func asPayment(payment: [String: Any?]) throws -> Payment {
        guard let timestamp = payment["timestamp"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "timestamp", typeName: "Payment"))
        }
        guard let amountSat = payment["amountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountSat", typeName: "Payment"))
        }
        guard let feesSat = payment["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "Payment"))
        }
        guard let paymentTypeTmp = payment["paymentType"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "paymentType", typeName: "Payment"))
        }
        let paymentType = try asPaymentType(paymentType: paymentTypeTmp)

        guard let statusTmp = payment["status"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "status", typeName: "Payment"))
        }
        let status = try asPaymentState(paymentState: statusTmp)

        guard let description = payment["description"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "description", typeName: "Payment"))
        }
        var txId: String?
        if hasNonNilKey(data: payment, key: "txId") {
            guard let txIdTmp = payment["txId"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "txId"))
            }
            txId = txIdTmp
        }
        var swapId: String?
        if hasNonNilKey(data: payment, key: "swapId") {
            guard let swapIdTmp = payment["swapId"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "swapId"))
            }
            swapId = swapIdTmp
        }
        var preimage: String?
        if hasNonNilKey(data: payment, key: "preimage") {
            guard let preimageTmp = payment["preimage"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "preimage"))
            }
            preimage = preimageTmp
        }
        var bolt11: String?
        if hasNonNilKey(data: payment, key: "bolt11") {
            guard let bolt11Tmp = payment["bolt11"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "bolt11"))
            }
            bolt11 = bolt11Tmp
        }
        var refundTxId: String?
        if hasNonNilKey(data: payment, key: "refundTxId") {
            guard let refundTxIdTmp = payment["refundTxId"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "refundTxId"))
            }
            refundTxId = refundTxIdTmp
        }
        var refundTxAmountSat: UInt64?
        if hasNonNilKey(data: payment, key: "refundTxAmountSat") {
            guard let refundTxAmountSatTmp = payment["refundTxAmountSat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "refundTxAmountSat"))
            }
            refundTxAmountSat = refundTxAmountSatTmp
        }

        return Payment(
            timestamp: timestamp,
            amountSat: amountSat,
            feesSat: feesSat,
            paymentType: paymentType,
            status: status,
            description: description,
            txId: txId,
            swapId: swapId,
            preimage: preimage,
            bolt11: bolt11,
            refundTxId: refundTxId,
            refundTxAmountSat: refundTxAmountSat
        )
    }

    static func dictionaryOf(payment: Payment) -> [String: Any?] {
        return [
            "timestamp": payment.timestamp,
            "amountSat": payment.amountSat,
            "feesSat": payment.feesSat,
            "paymentType": valueOf(paymentType: payment.paymentType),
            "status": valueOf(paymentState: payment.status),
            "description": payment.description,
            "txId": payment.txId == nil ? nil : payment.txId,
            "swapId": payment.swapId == nil ? nil : payment.swapId,
            "preimage": payment.preimage == nil ? nil : payment.preimage,
            "bolt11": payment.bolt11 == nil ? nil : payment.bolt11,
            "refundTxId": payment.refundTxId == nil ? nil : payment.refundTxId,
            "refundTxAmountSat": payment.refundTxAmountSat == nil ? nil : payment.refundTxAmountSat,
        ]
    }

    static func asPaymentList(arr: [Any]) throws -> [Payment] {
        var list = [Payment]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var payment = try asPayment(payment: val)
                list.append(payment)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Payment"))
            }
        }
        return list
    }

    static func arrayOf(paymentList: [Payment]) -> [Any] {
        return paymentList.map { v -> [String: Any?] in return dictionaryOf(payment: v) }
    }

    static func asPrepareBuyBitcoinRequest(prepareBuyBitcoinRequest: [String: Any?]) throws -> PrepareBuyBitcoinRequest {
        guard let providerTmp = prepareBuyBitcoinRequest["provider"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "provider", typeName: "PrepareBuyBitcoinRequest"))
        }
        let provider = try asBuyBitcoinProvider(buyBitcoinProvider: providerTmp)

        guard let amountSat = prepareBuyBitcoinRequest["amountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountSat", typeName: "PrepareBuyBitcoinRequest"))
        }

        return PrepareBuyBitcoinRequest(
            provider: provider,
            amountSat: amountSat
        )
    }

    static func dictionaryOf(prepareBuyBitcoinRequest: PrepareBuyBitcoinRequest) -> [String: Any?] {
        return [
            "provider": valueOf(buyBitcoinProvider: prepareBuyBitcoinRequest.provider),
            "amountSat": prepareBuyBitcoinRequest.amountSat,
        ]
    }

    static func asPrepareBuyBitcoinRequestList(arr: [Any]) throws -> [PrepareBuyBitcoinRequest] {
        var list = [PrepareBuyBitcoinRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareBuyBitcoinRequest = try asPrepareBuyBitcoinRequest(prepareBuyBitcoinRequest: val)
                list.append(prepareBuyBitcoinRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareBuyBitcoinRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareBuyBitcoinRequestList: [PrepareBuyBitcoinRequest]) -> [Any] {
        return prepareBuyBitcoinRequestList.map { v -> [String: Any?] in return dictionaryOf(prepareBuyBitcoinRequest: v) }
    }

    static func asPrepareBuyBitcoinResponse(prepareBuyBitcoinResponse: [String: Any?]) throws -> PrepareBuyBitcoinResponse {
        guard let providerTmp = prepareBuyBitcoinResponse["provider"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "provider", typeName: "PrepareBuyBitcoinResponse"))
        }
        let provider = try asBuyBitcoinProvider(buyBitcoinProvider: providerTmp)

        guard let amountSat = prepareBuyBitcoinResponse["amountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountSat", typeName: "PrepareBuyBitcoinResponse"))
        }
        guard let feesSat = prepareBuyBitcoinResponse["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareBuyBitcoinResponse"))
        }

        return PrepareBuyBitcoinResponse(
            provider: provider,
            amountSat: amountSat,
            feesSat: feesSat
        )
    }

    static func dictionaryOf(prepareBuyBitcoinResponse: PrepareBuyBitcoinResponse) -> [String: Any?] {
        return [
            "provider": valueOf(buyBitcoinProvider: prepareBuyBitcoinResponse.provider),
            "amountSat": prepareBuyBitcoinResponse.amountSat,
            "feesSat": prepareBuyBitcoinResponse.feesSat,
        ]
    }

    static func asPrepareBuyBitcoinResponseList(arr: [Any]) throws -> [PrepareBuyBitcoinResponse] {
        var list = [PrepareBuyBitcoinResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareBuyBitcoinResponse = try asPrepareBuyBitcoinResponse(prepareBuyBitcoinResponse: val)
                list.append(prepareBuyBitcoinResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareBuyBitcoinResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareBuyBitcoinResponseList: [PrepareBuyBitcoinResponse]) -> [Any] {
        return prepareBuyBitcoinResponseList.map { v -> [String: Any?] in return dictionaryOf(prepareBuyBitcoinResponse: v) }
    }

    static func asPreparePayOnchainRequest(preparePayOnchainRequest: [String: Any?]) throws -> PreparePayOnchainRequest {
        guard let receiverAmountSat = preparePayOnchainRequest["receiverAmountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receiverAmountSat", typeName: "PreparePayOnchainRequest"))
        }
        var satPerVbyte: UInt32?
        if hasNonNilKey(data: preparePayOnchainRequest, key: "satPerVbyte") {
            guard let satPerVbyteTmp = preparePayOnchainRequest["satPerVbyte"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "satPerVbyte"))
            }
            satPerVbyte = satPerVbyteTmp
        }

        return PreparePayOnchainRequest(
            receiverAmountSat: receiverAmountSat,
            satPerVbyte: satPerVbyte
        )
    }

    static func dictionaryOf(preparePayOnchainRequest: PreparePayOnchainRequest) -> [String: Any?] {
        return [
            "receiverAmountSat": preparePayOnchainRequest.receiverAmountSat,
            "satPerVbyte": preparePayOnchainRequest.satPerVbyte == nil ? nil : preparePayOnchainRequest.satPerVbyte,
        ]
    }

    static func asPreparePayOnchainRequestList(arr: [Any]) throws -> [PreparePayOnchainRequest] {
        var list = [PreparePayOnchainRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var preparePayOnchainRequest = try asPreparePayOnchainRequest(preparePayOnchainRequest: val)
                list.append(preparePayOnchainRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PreparePayOnchainRequest"))
            }
        }
        return list
    }

    static func arrayOf(preparePayOnchainRequestList: [PreparePayOnchainRequest]) -> [Any] {
        return preparePayOnchainRequestList.map { v -> [String: Any?] in return dictionaryOf(preparePayOnchainRequest: v) }
    }

    static func asPreparePayOnchainResponse(preparePayOnchainResponse: [String: Any?]) throws -> PreparePayOnchainResponse {
        guard let receiverAmountSat = preparePayOnchainResponse["receiverAmountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "receiverAmountSat", typeName: "PreparePayOnchainResponse"))
        }
        guard let claimFeesSat = preparePayOnchainResponse["claimFeesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "claimFeesSat", typeName: "PreparePayOnchainResponse"))
        }
        guard let totalFeesSat = preparePayOnchainResponse["totalFeesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "totalFeesSat", typeName: "PreparePayOnchainResponse"))
        }

        return PreparePayOnchainResponse(
            receiverAmountSat: receiverAmountSat,
            claimFeesSat: claimFeesSat,
            totalFeesSat: totalFeesSat
        )
    }

    static func dictionaryOf(preparePayOnchainResponse: PreparePayOnchainResponse) -> [String: Any?] {
        return [
            "receiverAmountSat": preparePayOnchainResponse.receiverAmountSat,
            "claimFeesSat": preparePayOnchainResponse.claimFeesSat,
            "totalFeesSat": preparePayOnchainResponse.totalFeesSat,
        ]
    }

    static func asPreparePayOnchainResponseList(arr: [Any]) throws -> [PreparePayOnchainResponse] {
        var list = [PreparePayOnchainResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var preparePayOnchainResponse = try asPreparePayOnchainResponse(preparePayOnchainResponse: val)
                list.append(preparePayOnchainResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PreparePayOnchainResponse"))
            }
        }
        return list
    }

    static func arrayOf(preparePayOnchainResponseList: [PreparePayOnchainResponse]) -> [Any] {
        return preparePayOnchainResponseList.map { v -> [String: Any?] in return dictionaryOf(preparePayOnchainResponse: v) }
    }

    static func asPrepareReceiveOnchainRequest(prepareReceiveOnchainRequest: [String: Any?]) throws -> PrepareReceiveOnchainRequest {
        guard let payerAmountSat = prepareReceiveOnchainRequest["payerAmountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "PrepareReceiveOnchainRequest"))
        }

        return PrepareReceiveOnchainRequest(
            payerAmountSat: payerAmountSat)
    }

    static func dictionaryOf(prepareReceiveOnchainRequest: PrepareReceiveOnchainRequest) -> [String: Any?] {
        return [
            "payerAmountSat": prepareReceiveOnchainRequest.payerAmountSat,
        ]
    }

    static func asPrepareReceiveOnchainRequestList(arr: [Any]) throws -> [PrepareReceiveOnchainRequest] {
        var list = [PrepareReceiveOnchainRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareReceiveOnchainRequest = try asPrepareReceiveOnchainRequest(prepareReceiveOnchainRequest: val)
                list.append(prepareReceiveOnchainRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareReceiveOnchainRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareReceiveOnchainRequestList: [PrepareReceiveOnchainRequest]) -> [Any] {
        return prepareReceiveOnchainRequestList.map { v -> [String: Any?] in return dictionaryOf(prepareReceiveOnchainRequest: v) }
    }

    static func asPrepareReceiveOnchainResponse(prepareReceiveOnchainResponse: [String: Any?]) throws -> PrepareReceiveOnchainResponse {
        guard let payerAmountSat = prepareReceiveOnchainResponse["payerAmountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "PrepareReceiveOnchainResponse"))
        }
        guard let feesSat = prepareReceiveOnchainResponse["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareReceiveOnchainResponse"))
        }

        return PrepareReceiveOnchainResponse(
            payerAmountSat: payerAmountSat,
            feesSat: feesSat
        )
    }

    static func dictionaryOf(prepareReceiveOnchainResponse: PrepareReceiveOnchainResponse) -> [String: Any?] {
        return [
            "payerAmountSat": prepareReceiveOnchainResponse.payerAmountSat,
            "feesSat": prepareReceiveOnchainResponse.feesSat,
        ]
    }

    static func asPrepareReceiveOnchainResponseList(arr: [Any]) throws -> [PrepareReceiveOnchainResponse] {
        var list = [PrepareReceiveOnchainResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareReceiveOnchainResponse = try asPrepareReceiveOnchainResponse(prepareReceiveOnchainResponse: val)
                list.append(prepareReceiveOnchainResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareReceiveOnchainResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareReceiveOnchainResponseList: [PrepareReceiveOnchainResponse]) -> [Any] {
        return prepareReceiveOnchainResponseList.map { v -> [String: Any?] in return dictionaryOf(prepareReceiveOnchainResponse: v) }
    }

    static func asPrepareReceivePaymentRequest(prepareReceivePaymentRequest: [String: Any?]) throws -> PrepareReceivePaymentRequest {
        guard let payerAmountSat = prepareReceivePaymentRequest["payerAmountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "PrepareReceivePaymentRequest"))
        }

        return PrepareReceivePaymentRequest(
            payerAmountSat: payerAmountSat)
    }

    static func dictionaryOf(prepareReceivePaymentRequest: PrepareReceivePaymentRequest) -> [String: Any?] {
        return [
            "payerAmountSat": prepareReceivePaymentRequest.payerAmountSat,
        ]
    }

    static func asPrepareReceivePaymentRequestList(arr: [Any]) throws -> [PrepareReceivePaymentRequest] {
        var list = [PrepareReceivePaymentRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareReceivePaymentRequest = try asPrepareReceivePaymentRequest(prepareReceivePaymentRequest: val)
                list.append(prepareReceivePaymentRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareReceivePaymentRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareReceivePaymentRequestList: [PrepareReceivePaymentRequest]) -> [Any] {
        return prepareReceivePaymentRequestList.map { v -> [String: Any?] in return dictionaryOf(prepareReceivePaymentRequest: v) }
    }

    static func asPrepareReceivePaymentResponse(prepareReceivePaymentResponse: [String: Any?]) throws -> PrepareReceivePaymentResponse {
        guard let payerAmountSat = prepareReceivePaymentResponse["payerAmountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payerAmountSat", typeName: "PrepareReceivePaymentResponse"))
        }
        guard let feesSat = prepareReceivePaymentResponse["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareReceivePaymentResponse"))
        }

        return PrepareReceivePaymentResponse(
            payerAmountSat: payerAmountSat,
            feesSat: feesSat
        )
    }

    static func dictionaryOf(prepareReceivePaymentResponse: PrepareReceivePaymentResponse) -> [String: Any?] {
        return [
            "payerAmountSat": prepareReceivePaymentResponse.payerAmountSat,
            "feesSat": prepareReceivePaymentResponse.feesSat,
        ]
    }

    static func asPrepareReceivePaymentResponseList(arr: [Any]) throws -> [PrepareReceivePaymentResponse] {
        var list = [PrepareReceivePaymentResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareReceivePaymentResponse = try asPrepareReceivePaymentResponse(prepareReceivePaymentResponse: val)
                list.append(prepareReceivePaymentResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareReceivePaymentResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareReceivePaymentResponseList: [PrepareReceivePaymentResponse]) -> [Any] {
        return prepareReceivePaymentResponseList.map { v -> [String: Any?] in return dictionaryOf(prepareReceivePaymentResponse: v) }
    }

    static func asPrepareRefundRequest(prepareRefundRequest: [String: Any?]) throws -> PrepareRefundRequest {
        guard let swapAddress = prepareRefundRequest["swapAddress"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapAddress", typeName: "PrepareRefundRequest"))
        }
        guard let refundAddress = prepareRefundRequest["refundAddress"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "refundAddress", typeName: "PrepareRefundRequest"))
        }
        guard let satPerVbyte = prepareRefundRequest["satPerVbyte"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "satPerVbyte", typeName: "PrepareRefundRequest"))
        }

        return PrepareRefundRequest(
            swapAddress: swapAddress,
            refundAddress: refundAddress,
            satPerVbyte: satPerVbyte
        )
    }

    static func dictionaryOf(prepareRefundRequest: PrepareRefundRequest) -> [String: Any?] {
        return [
            "swapAddress": prepareRefundRequest.swapAddress,
            "refundAddress": prepareRefundRequest.refundAddress,
            "satPerVbyte": prepareRefundRequest.satPerVbyte,
        ]
    }

    static func asPrepareRefundRequestList(arr: [Any]) throws -> [PrepareRefundRequest] {
        var list = [PrepareRefundRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareRefundRequest = try asPrepareRefundRequest(prepareRefundRequest: val)
                list.append(prepareRefundRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareRefundRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareRefundRequestList: [PrepareRefundRequest]) -> [Any] {
        return prepareRefundRequestList.map { v -> [String: Any?] in return dictionaryOf(prepareRefundRequest: v) }
    }

    static func asPrepareRefundResponse(prepareRefundResponse: [String: Any?]) throws -> PrepareRefundResponse {
        guard let txVsize = prepareRefundResponse["txVsize"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "txVsize", typeName: "PrepareRefundResponse"))
        }
        guard let txFeeSat = prepareRefundResponse["txFeeSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "txFeeSat", typeName: "PrepareRefundResponse"))
        }
        var refundTxId: String?
        if hasNonNilKey(data: prepareRefundResponse, key: "refundTxId") {
            guard let refundTxIdTmp = prepareRefundResponse["refundTxId"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "refundTxId"))
            }
            refundTxId = refundTxIdTmp
        }

        return PrepareRefundResponse(
            txVsize: txVsize,
            txFeeSat: txFeeSat,
            refundTxId: refundTxId
        )
    }

    static func dictionaryOf(prepareRefundResponse: PrepareRefundResponse) -> [String: Any?] {
        return [
            "txVsize": prepareRefundResponse.txVsize,
            "txFeeSat": prepareRefundResponse.txFeeSat,
            "refundTxId": prepareRefundResponse.refundTxId == nil ? nil : prepareRefundResponse.refundTxId,
        ]
    }

    static func asPrepareRefundResponseList(arr: [Any]) throws -> [PrepareRefundResponse] {
        var list = [PrepareRefundResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareRefundResponse = try asPrepareRefundResponse(prepareRefundResponse: val)
                list.append(prepareRefundResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareRefundResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareRefundResponseList: [PrepareRefundResponse]) -> [Any] {
        return prepareRefundResponseList.map { v -> [String: Any?] in return dictionaryOf(prepareRefundResponse: v) }
    }

    static func asPrepareSendRequest(prepareSendRequest: [String: Any?]) throws -> PrepareSendRequest {
        guard let invoice = prepareSendRequest["invoice"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "PrepareSendRequest"))
        }

        return PrepareSendRequest(
            invoice: invoice)
    }

    static func dictionaryOf(prepareSendRequest: PrepareSendRequest) -> [String: Any?] {
        return [
            "invoice": prepareSendRequest.invoice,
        ]
    }

    static func asPrepareSendRequestList(arr: [Any]) throws -> [PrepareSendRequest] {
        var list = [PrepareSendRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareSendRequest = try asPrepareSendRequest(prepareSendRequest: val)
                list.append(prepareSendRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareSendRequest"))
            }
        }
        return list
    }

    static func arrayOf(prepareSendRequestList: [PrepareSendRequest]) -> [Any] {
        return prepareSendRequestList.map { v -> [String: Any?] in return dictionaryOf(prepareSendRequest: v) }
    }

    static func asPrepareSendResponse(prepareSendResponse: [String: Any?]) throws -> PrepareSendResponse {
        guard let invoice = prepareSendResponse["invoice"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "PrepareSendResponse"))
        }
        guard let feesSat = prepareSendResponse["feesSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesSat", typeName: "PrepareSendResponse"))
        }

        return PrepareSendResponse(
            invoice: invoice,
            feesSat: feesSat
        )
    }

    static func dictionaryOf(prepareSendResponse: PrepareSendResponse) -> [String: Any?] {
        return [
            "invoice": prepareSendResponse.invoice,
            "feesSat": prepareSendResponse.feesSat,
        ]
    }

    static func asPrepareSendResponseList(arr: [Any]) throws -> [PrepareSendResponse] {
        var list = [PrepareSendResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var prepareSendResponse = try asPrepareSendResponse(prepareSendResponse: val)
                list.append(prepareSendResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PrepareSendResponse"))
            }
        }
        return list
    }

    static func arrayOf(prepareSendResponseList: [PrepareSendResponse]) -> [Any] {
        return prepareSendResponseList.map { v -> [String: Any?] in return dictionaryOf(prepareSendResponse: v) }
    }

    static func asRate(rate: [String: Any?]) throws -> Rate {
        guard let coin = rate["coin"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "coin", typeName: "Rate"))
        }
        guard let value = rate["value"] as? Double else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "value", typeName: "Rate"))
        }

        return Rate(
            coin: coin,
            value: value
        )
    }

    static func dictionaryOf(rate: Rate) -> [String: Any?] {
        return [
            "coin": rate.coin,
            "value": rate.value,
        ]
    }

    static func asRateList(arr: [Any]) throws -> [Rate] {
        var list = [Rate]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var rate = try asRate(rate: val)
                list.append(rate)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Rate"))
            }
        }
        return list
    }

    static func arrayOf(rateList: [Rate]) -> [Any] {
        return rateList.map { v -> [String: Any?] in return dictionaryOf(rate: v) }
    }

    static func asReceiveOnchainResponse(receiveOnchainResponse: [String: Any?]) throws -> ReceiveOnchainResponse {
        guard let address = receiveOnchainResponse["address"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "address", typeName: "ReceiveOnchainResponse"))
        }
        guard let bip21 = receiveOnchainResponse["bip21"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "bip21", typeName: "ReceiveOnchainResponse"))
        }

        return ReceiveOnchainResponse(
            address: address,
            bip21: bip21
        )
    }

    static func dictionaryOf(receiveOnchainResponse: ReceiveOnchainResponse) -> [String: Any?] {
        return [
            "address": receiveOnchainResponse.address,
            "bip21": receiveOnchainResponse.bip21,
        ]
    }

    static func asReceiveOnchainResponseList(arr: [Any]) throws -> [ReceiveOnchainResponse] {
        var list = [ReceiveOnchainResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var receiveOnchainResponse = try asReceiveOnchainResponse(receiveOnchainResponse: val)
                list.append(receiveOnchainResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ReceiveOnchainResponse"))
            }
        }
        return list
    }

    static func arrayOf(receiveOnchainResponseList: [ReceiveOnchainResponse]) -> [Any] {
        return receiveOnchainResponseList.map { v -> [String: Any?] in return dictionaryOf(receiveOnchainResponse: v) }
    }

    static func asReceivePaymentRequest(receivePaymentRequest: [String: Any?]) throws -> ReceivePaymentRequest {
        guard let prepareResTmp = receivePaymentRequest["prepareRes"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "prepareRes", typeName: "ReceivePaymentRequest"))
        }
        let prepareRes = try asPrepareReceivePaymentResponse(prepareReceivePaymentResponse: prepareResTmp)

        var description: String?
        if hasNonNilKey(data: receivePaymentRequest, key: "description") {
            guard let descriptionTmp = receivePaymentRequest["description"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "description"))
            }
            description = descriptionTmp
        }
        var useDescriptionHash: Bool?
        if hasNonNilKey(data: receivePaymentRequest, key: "useDescriptionHash") {
            guard let useDescriptionHashTmp = receivePaymentRequest["useDescriptionHash"] as? Bool else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "useDescriptionHash"))
            }
            useDescriptionHash = useDescriptionHashTmp
        }

        return ReceivePaymentRequest(
            prepareRes: prepareRes,
            description: description,
            useDescriptionHash: useDescriptionHash
        )
    }

    static func dictionaryOf(receivePaymentRequest: ReceivePaymentRequest) -> [String: Any?] {
        return [
            "prepareRes": dictionaryOf(prepareReceivePaymentResponse: receivePaymentRequest.prepareRes),
            "description": receivePaymentRequest.description == nil ? nil : receivePaymentRequest.description,
            "useDescriptionHash": receivePaymentRequest.useDescriptionHash == nil ? nil : receivePaymentRequest.useDescriptionHash,
        ]
    }

    static func asReceivePaymentRequestList(arr: [Any]) throws -> [ReceivePaymentRequest] {
        var list = [ReceivePaymentRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var receivePaymentRequest = try asReceivePaymentRequest(receivePaymentRequest: val)
                list.append(receivePaymentRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ReceivePaymentRequest"))
            }
        }
        return list
    }

    static func arrayOf(receivePaymentRequestList: [ReceivePaymentRequest]) -> [Any] {
        return receivePaymentRequestList.map { v -> [String: Any?] in return dictionaryOf(receivePaymentRequest: v) }
    }

    static func asReceivePaymentResponse(receivePaymentResponse: [String: Any?]) throws -> ReceivePaymentResponse {
        guard let id = receivePaymentResponse["id"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "id", typeName: "ReceivePaymentResponse"))
        }
        guard let invoice = receivePaymentResponse["invoice"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "ReceivePaymentResponse"))
        }

        return ReceivePaymentResponse(
            id: id,
            invoice: invoice
        )
    }

    static func dictionaryOf(receivePaymentResponse: ReceivePaymentResponse) -> [String: Any?] {
        return [
            "id": receivePaymentResponse.id,
            "invoice": receivePaymentResponse.invoice,
        ]
    }

    static func asReceivePaymentResponseList(arr: [Any]) throws -> [ReceivePaymentResponse] {
        var list = [ReceivePaymentResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var receivePaymentResponse = try asReceivePaymentResponse(receivePaymentResponse: val)
                list.append(receivePaymentResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "ReceivePaymentResponse"))
            }
        }
        return list
    }

    static func arrayOf(receivePaymentResponseList: [ReceivePaymentResponse]) -> [Any] {
        return receivePaymentResponseList.map { v -> [String: Any?] in return dictionaryOf(receivePaymentResponse: v) }
    }

    static func asRecommendedFees(recommendedFees: [String: Any?]) throws -> RecommendedFees {
        guard let fastestFee = recommendedFees["fastestFee"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "fastestFee", typeName: "RecommendedFees"))
        }
        guard let halfHourFee = recommendedFees["halfHourFee"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "halfHourFee", typeName: "RecommendedFees"))
        }
        guard let hourFee = recommendedFees["hourFee"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "hourFee", typeName: "RecommendedFees"))
        }
        guard let economyFee = recommendedFees["economyFee"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "economyFee", typeName: "RecommendedFees"))
        }
        guard let minimumFee = recommendedFees["minimumFee"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "minimumFee", typeName: "RecommendedFees"))
        }

        return RecommendedFees(
            fastestFee: fastestFee,
            halfHourFee: halfHourFee,
            hourFee: hourFee,
            economyFee: economyFee,
            minimumFee: minimumFee
        )
    }

    static func dictionaryOf(recommendedFees: RecommendedFees) -> [String: Any?] {
        return [
            "fastestFee": recommendedFees.fastestFee,
            "halfHourFee": recommendedFees.halfHourFee,
            "hourFee": recommendedFees.hourFee,
            "economyFee": recommendedFees.economyFee,
            "minimumFee": recommendedFees.minimumFee,
        ]
    }

    static func asRecommendedFeesList(arr: [Any]) throws -> [RecommendedFees] {
        var list = [RecommendedFees]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var recommendedFees = try asRecommendedFees(recommendedFees: val)
                list.append(recommendedFees)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RecommendedFees"))
            }
        }
        return list
    }

    static func arrayOf(recommendedFeesList: [RecommendedFees]) -> [Any] {
        return recommendedFeesList.map { v -> [String: Any?] in return dictionaryOf(recommendedFees: v) }
    }

    static func asRefundRequest(refundRequest: [String: Any?]) throws -> RefundRequest {
        guard let swapAddress = refundRequest["swapAddress"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapAddress", typeName: "RefundRequest"))
        }
        guard let refundAddress = refundRequest["refundAddress"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "refundAddress", typeName: "RefundRequest"))
        }
        guard let satPerVbyte = refundRequest["satPerVbyte"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "satPerVbyte", typeName: "RefundRequest"))
        }

        return RefundRequest(
            swapAddress: swapAddress,
            refundAddress: refundAddress,
            satPerVbyte: satPerVbyte
        )
    }

    static func dictionaryOf(refundRequest: RefundRequest) -> [String: Any?] {
        return [
            "swapAddress": refundRequest.swapAddress,
            "refundAddress": refundRequest.refundAddress,
            "satPerVbyte": refundRequest.satPerVbyte,
        ]
    }

    static func asRefundRequestList(arr: [Any]) throws -> [RefundRequest] {
        var list = [RefundRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var refundRequest = try asRefundRequest(refundRequest: val)
                list.append(refundRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RefundRequest"))
            }
        }
        return list
    }

    static func arrayOf(refundRequestList: [RefundRequest]) -> [Any] {
        return refundRequestList.map { v -> [String: Any?] in return dictionaryOf(refundRequest: v) }
    }

    static func asRefundResponse(refundResponse: [String: Any?]) throws -> RefundResponse {
        guard let refundTxId = refundResponse["refundTxId"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "refundTxId", typeName: "RefundResponse"))
        }

        return RefundResponse(
            refundTxId: refundTxId)
    }

    static func dictionaryOf(refundResponse: RefundResponse) -> [String: Any?] {
        return [
            "refundTxId": refundResponse.refundTxId,
        ]
    }

    static func asRefundResponseList(arr: [Any]) throws -> [RefundResponse] {
        var list = [RefundResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var refundResponse = try asRefundResponse(refundResponse: val)
                list.append(refundResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RefundResponse"))
            }
        }
        return list
    }

    static func arrayOf(refundResponseList: [RefundResponse]) -> [Any] {
        return refundResponseList.map { v -> [String: Any?] in return dictionaryOf(refundResponse: v) }
    }

    static func asRefundableSwap(refundableSwap: [String: Any?]) throws -> RefundableSwap {
        guard let swapAddress = refundableSwap["swapAddress"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "swapAddress", typeName: "RefundableSwap"))
        }
        guard let timestamp = refundableSwap["timestamp"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "timestamp", typeName: "RefundableSwap"))
        }
        guard let amountSat = refundableSwap["amountSat"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "amountSat", typeName: "RefundableSwap"))
        }

        return RefundableSwap(
            swapAddress: swapAddress,
            timestamp: timestamp,
            amountSat: amountSat
        )
    }

    static func dictionaryOf(refundableSwap: RefundableSwap) -> [String: Any?] {
        return [
            "swapAddress": refundableSwap.swapAddress,
            "timestamp": refundableSwap.timestamp,
            "amountSat": refundableSwap.amountSat,
        ]
    }

    static func asRefundableSwapList(arr: [Any]) throws -> [RefundableSwap] {
        var list = [RefundableSwap]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var refundableSwap = try asRefundableSwap(refundableSwap: val)
                list.append(refundableSwap)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RefundableSwap"))
            }
        }
        return list
    }

    static func arrayOf(refundableSwapList: [RefundableSwap]) -> [Any] {
        return refundableSwapList.map { v -> [String: Any?] in return dictionaryOf(refundableSwap: v) }
    }

    static func asRestoreRequest(restoreRequest: [String: Any?]) throws -> RestoreRequest {
        var backupPath: String?
        if hasNonNilKey(data: restoreRequest, key: "backupPath") {
            guard let backupPathTmp = restoreRequest["backupPath"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "backupPath"))
            }
            backupPath = backupPathTmp
        }

        return RestoreRequest(
            backupPath: backupPath)
    }

    static func dictionaryOf(restoreRequest: RestoreRequest) -> [String: Any?] {
        return [
            "backupPath": restoreRequest.backupPath == nil ? nil : restoreRequest.backupPath,
        ]
    }

    static func asRestoreRequestList(arr: [Any]) throws -> [RestoreRequest] {
        var list = [RestoreRequest]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var restoreRequest = try asRestoreRequest(restoreRequest: val)
                list.append(restoreRequest)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RestoreRequest"))
            }
        }
        return list
    }

    static func arrayOf(restoreRequestList: [RestoreRequest]) -> [Any] {
        return restoreRequestList.map { v -> [String: Any?] in return dictionaryOf(restoreRequest: v) }
    }

    static func asRouteHint(routeHint: [String: Any?]) throws -> RouteHint {
        guard let hopsTmp = routeHint["hops"] as? [[String: Any?]] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "hops", typeName: "RouteHint"))
        }
        let hops = try asRouteHintHopList(arr: hopsTmp)

        return RouteHint(
            hops: hops)
    }

    static func dictionaryOf(routeHint: RouteHint) -> [String: Any?] {
        return [
            "hops": arrayOf(routeHintHopList: routeHint.hops),
        ]
    }

    static func asRouteHintList(arr: [Any]) throws -> [RouteHint] {
        var list = [RouteHint]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var routeHint = try asRouteHint(routeHint: val)
                list.append(routeHint)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RouteHint"))
            }
        }
        return list
    }

    static func arrayOf(routeHintList: [RouteHint]) -> [Any] {
        return routeHintList.map { v -> [String: Any?] in return dictionaryOf(routeHint: v) }
    }

    static func asRouteHintHop(routeHintHop: [String: Any?]) throws -> RouteHintHop {
        guard let srcNodeId = routeHintHop["srcNodeId"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "srcNodeId", typeName: "RouteHintHop"))
        }
        guard let shortChannelId = routeHintHop["shortChannelId"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "shortChannelId", typeName: "RouteHintHop"))
        }
        guard let feesBaseMsat = routeHintHop["feesBaseMsat"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesBaseMsat", typeName: "RouteHintHop"))
        }
        guard let feesProportionalMillionths = routeHintHop["feesProportionalMillionths"] as? UInt32 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "feesProportionalMillionths", typeName: "RouteHintHop"))
        }
        guard let cltvExpiryDelta = routeHintHop["cltvExpiryDelta"] as? UInt64 else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "cltvExpiryDelta", typeName: "RouteHintHop"))
        }
        var htlcMinimumMsat: UInt64?
        if hasNonNilKey(data: routeHintHop, key: "htlcMinimumMsat") {
            guard let htlcMinimumMsatTmp = routeHintHop["htlcMinimumMsat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "htlcMinimumMsat"))
            }
            htlcMinimumMsat = htlcMinimumMsatTmp
        }
        var htlcMaximumMsat: UInt64?
        if hasNonNilKey(data: routeHintHop, key: "htlcMaximumMsat") {
            guard let htlcMaximumMsatTmp = routeHintHop["htlcMaximumMsat"] as? UInt64 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "htlcMaximumMsat"))
            }
            htlcMaximumMsat = htlcMaximumMsatTmp
        }

        return RouteHintHop(
            srcNodeId: srcNodeId,
            shortChannelId: shortChannelId,
            feesBaseMsat: feesBaseMsat,
            feesProportionalMillionths: feesProportionalMillionths,
            cltvExpiryDelta: cltvExpiryDelta,
            htlcMinimumMsat: htlcMinimumMsat,
            htlcMaximumMsat: htlcMaximumMsat
        )
    }

    static func dictionaryOf(routeHintHop: RouteHintHop) -> [String: Any?] {
        return [
            "srcNodeId": routeHintHop.srcNodeId,
            "shortChannelId": routeHintHop.shortChannelId,
            "feesBaseMsat": routeHintHop.feesBaseMsat,
            "feesProportionalMillionths": routeHintHop.feesProportionalMillionths,
            "cltvExpiryDelta": routeHintHop.cltvExpiryDelta,
            "htlcMinimumMsat": routeHintHop.htlcMinimumMsat == nil ? nil : routeHintHop.htlcMinimumMsat,
            "htlcMaximumMsat": routeHintHop.htlcMaximumMsat == nil ? nil : routeHintHop.htlcMaximumMsat,
        ]
    }

    static func asRouteHintHopList(arr: [Any]) throws -> [RouteHintHop] {
        var list = [RouteHintHop]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var routeHintHop = try asRouteHintHop(routeHintHop: val)
                list.append(routeHintHop)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "RouteHintHop"))
            }
        }
        return list
    }

    static func arrayOf(routeHintHopList: [RouteHintHop]) -> [Any] {
        return routeHintHopList.map { v -> [String: Any?] in return dictionaryOf(routeHintHop: v) }
    }

    static func asSendPaymentResponse(sendPaymentResponse: [String: Any?]) throws -> SendPaymentResponse {
        guard let paymentTmp = sendPaymentResponse["payment"] as? [String: Any?] else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "payment", typeName: "SendPaymentResponse"))
        }
        let payment = try asPayment(payment: paymentTmp)

        return SendPaymentResponse(
            payment: payment)
    }

    static func dictionaryOf(sendPaymentResponse: SendPaymentResponse) -> [String: Any?] {
        return [
            "payment": dictionaryOf(payment: sendPaymentResponse.payment),
        ]
    }

    static func asSendPaymentResponseList(arr: [Any]) throws -> [SendPaymentResponse] {
        var list = [SendPaymentResponse]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var sendPaymentResponse = try asSendPaymentResponse(sendPaymentResponse: val)
                list.append(sendPaymentResponse)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "SendPaymentResponse"))
            }
        }
        return list
    }

    static func arrayOf(sendPaymentResponseList: [SendPaymentResponse]) -> [Any] {
        return sendPaymentResponseList.map { v -> [String: Any?] in return dictionaryOf(sendPaymentResponse: v) }
    }

    static func asSymbol(symbol: [String: Any?]) throws -> Symbol {
        var grapheme: String?
        if hasNonNilKey(data: symbol, key: "grapheme") {
            guard let graphemeTmp = symbol["grapheme"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "grapheme"))
            }
            grapheme = graphemeTmp
        }
        var template: String?
        if hasNonNilKey(data: symbol, key: "template") {
            guard let templateTmp = symbol["template"] as? String else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "template"))
            }
            template = templateTmp
        }
        var rtl: Bool?
        if hasNonNilKey(data: symbol, key: "rtl") {
            guard let rtlTmp = symbol["rtl"] as? Bool else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "rtl"))
            }
            rtl = rtlTmp
        }
        var position: UInt32?
        if hasNonNilKey(data: symbol, key: "position") {
            guard let positionTmp = symbol["position"] as? UInt32 else {
                throw SdkError.Generic(message: errUnexpectedValue(fieldName: "position"))
            }
            position = positionTmp
        }

        return Symbol(
            grapheme: grapheme,
            template: template,
            rtl: rtl,
            position: position
        )
    }

    static func dictionaryOf(symbol: Symbol) -> [String: Any?] {
        return [
            "grapheme": symbol.grapheme == nil ? nil : symbol.grapheme,
            "template": symbol.template == nil ? nil : symbol.template,
            "rtl": symbol.rtl == nil ? nil : symbol.rtl,
            "position": symbol.position == nil ? nil : symbol.position,
        ]
    }

    static func asSymbolList(arr: [Any]) throws -> [Symbol] {
        var list = [Symbol]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var symbol = try asSymbol(symbol: val)
                list.append(symbol)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Symbol"))
            }
        }
        return list
    }

    static func arrayOf(symbolList: [Symbol]) -> [Any] {
        return symbolList.map { v -> [String: Any?] in return dictionaryOf(symbol: v) }
    }

    static func asUrlSuccessActionData(urlSuccessActionData: [String: Any?]) throws -> UrlSuccessActionData {
        guard let description = urlSuccessActionData["description"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "description", typeName: "UrlSuccessActionData"))
        }
        guard let url = urlSuccessActionData["url"] as? String else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "url", typeName: "UrlSuccessActionData"))
        }
        guard let matchesCallbackDomain = urlSuccessActionData["matchesCallbackDomain"] as? Bool else {
            throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "matchesCallbackDomain", typeName: "UrlSuccessActionData"))
        }

        return UrlSuccessActionData(
            description: description,
            url: url,
            matchesCallbackDomain: matchesCallbackDomain
        )
    }

    static func dictionaryOf(urlSuccessActionData: UrlSuccessActionData) -> [String: Any?] {
        return [
            "description": urlSuccessActionData.description,
            "url": urlSuccessActionData.url,
            "matchesCallbackDomain": urlSuccessActionData.matchesCallbackDomain,
        ]
    }

    static func asUrlSuccessActionDataList(arr: [Any]) throws -> [UrlSuccessActionData] {
        var list = [UrlSuccessActionData]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var urlSuccessActionData = try asUrlSuccessActionData(urlSuccessActionData: val)
                list.append(urlSuccessActionData)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "UrlSuccessActionData"))
            }
        }
        return list
    }

    static func arrayOf(urlSuccessActionDataList: [UrlSuccessActionData]) -> [Any] {
        return urlSuccessActionDataList.map { v -> [String: Any?] in return dictionaryOf(urlSuccessActionData: v) }
    }

    static func asAesSuccessActionDataResult(aesSuccessActionDataResult: [String: Any?]) throws -> AesSuccessActionDataResult {
        let type = aesSuccessActionDataResult["type"] as! String
        if type == "decrypted" {
            guard let dataTmp = aesSuccessActionDataResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "AesSuccessActionDataResult"))
            }
            let _data = try asAesSuccessActionDataDecrypted(aesSuccessActionDataDecrypted: dataTmp)

            return AesSuccessActionDataResult.decrypted(data: _data)
        }
        if type == "errorStatus" {
            guard let _reason = aesSuccessActionDataResult["reason"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "reason", typeName: "AesSuccessActionDataResult"))
            }
            return AesSuccessActionDataResult.errorStatus(reason: _reason)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum AesSuccessActionDataResult")
    }

    static func dictionaryOf(aesSuccessActionDataResult: AesSuccessActionDataResult) -> [String: Any?] {
        switch aesSuccessActionDataResult {
        case let .decrypted(
            data
        ):
            return [
                "type": "decrypted",
                "data": dictionaryOf(aesSuccessActionDataDecrypted: data),
            ]

        case let .errorStatus(
            reason
        ):
            return [
                "type": "errorStatus",
                "reason": reason,
            ]
        }
    }

    static func arrayOf(aesSuccessActionDataResultList: [AesSuccessActionDataResult]) -> [Any] {
        return aesSuccessActionDataResultList.map { v -> [String: Any?] in return dictionaryOf(aesSuccessActionDataResult: v) }
    }

    static func asAesSuccessActionDataResultList(arr: [Any]) throws -> [AesSuccessActionDataResult] {
        var list = [AesSuccessActionDataResult]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var aesSuccessActionDataResult = try asAesSuccessActionDataResult(aesSuccessActionDataResult: val)
                list.append(aesSuccessActionDataResult)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "AesSuccessActionDataResult"))
            }
        }
        return list
    }

    static func asBuyBitcoinProvider(buyBitcoinProvider: String) throws -> BuyBitcoinProvider {
        switch buyBitcoinProvider {
        case "moonpay":
            return BuyBitcoinProvider.moonpay

        default: throw SdkError.Generic(message: "Invalid variant \(buyBitcoinProvider) for enum BuyBitcoinProvider")
        }
    }

    static func valueOf(buyBitcoinProvider: BuyBitcoinProvider) -> String {
        switch buyBitcoinProvider {
        case .moonpay:
            return "moonpay"
        }
    }

    static func arrayOf(buyBitcoinProviderList: [BuyBitcoinProvider]) -> [String] {
        return buyBitcoinProviderList.map { v -> String in return valueOf(buyBitcoinProvider: v) }
    }

    static func asBuyBitcoinProviderList(arr: [Any]) throws -> [BuyBitcoinProvider] {
        var list = [BuyBitcoinProvider]()
        for value in arr {
            if let val = value as? String {
                var buyBitcoinProvider = try asBuyBitcoinProvider(buyBitcoinProvider: val)
                list.append(buyBitcoinProvider)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "BuyBitcoinProvider"))
            }
        }
        return list
    }

    static func asInputType(inputType: [String: Any?]) throws -> InputType {
        let type = inputType["type"] as! String
        if type == "bitcoinAddress" {
            guard let addressTmp = inputType["address"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "address", typeName: "InputType"))
            }
            let _address = try asBitcoinAddressData(bitcoinAddressData: addressTmp)

            return InputType.bitcoinAddress(address: _address)
        }
        if type == "bolt11" {
            guard let invoiceTmp = inputType["invoice"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "invoice", typeName: "InputType"))
            }
            let _invoice = try asLnInvoice(lnInvoice: invoiceTmp)

            return InputType.bolt11(invoice: _invoice)
        }
        if type == "nodeId" {
            guard let _nodeId = inputType["nodeId"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "nodeId", typeName: "InputType"))
            }
            return InputType.nodeId(nodeId: _nodeId)
        }
        if type == "url" {
            guard let _url = inputType["url"] as? String else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "url", typeName: "InputType"))
            }
            return InputType.url(url: _url)
        }
        if type == "lnUrlPay" {
            guard let dataTmp = inputType["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "InputType"))
            }
            let _data = try asLnUrlPayRequestData(lnUrlPayRequestData: dataTmp)

            return InputType.lnUrlPay(data: _data)
        }
        if type == "lnUrlWithdraw" {
            guard let dataTmp = inputType["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "InputType"))
            }
            let _data = try asLnUrlWithdrawRequestData(lnUrlWithdrawRequestData: dataTmp)

            return InputType.lnUrlWithdraw(data: _data)
        }
        if type == "lnUrlAuth" {
            guard let dataTmp = inputType["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "InputType"))
            }
            let _data = try asLnUrlAuthRequestData(lnUrlAuthRequestData: dataTmp)

            return InputType.lnUrlAuth(data: _data)
        }
        if type == "lnUrlError" {
            guard let dataTmp = inputType["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "InputType"))
            }
            let _data = try asLnUrlErrorData(lnUrlErrorData: dataTmp)

            return InputType.lnUrlError(data: _data)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum InputType")
    }

    static func dictionaryOf(inputType: InputType) -> [String: Any?] {
        switch inputType {
        case let .bitcoinAddress(
            address
        ):
            return [
                "type": "bitcoinAddress",
                "address": dictionaryOf(bitcoinAddressData: address),
            ]

        case let .bolt11(
            invoice
        ):
            return [
                "type": "bolt11",
                "invoice": dictionaryOf(lnInvoice: invoice),
            ]

        case let .nodeId(
            nodeId
        ):
            return [
                "type": "nodeId",
                "nodeId": nodeId,
            ]

        case let .url(
            url
        ):
            return [
                "type": "url",
                "url": url,
            ]

        case let .lnUrlPay(
            data
        ):
            return [
                "type": "lnUrlPay",
                "data": dictionaryOf(lnUrlPayRequestData: data),
            ]

        case let .lnUrlWithdraw(
            data
        ):
            return [
                "type": "lnUrlWithdraw",
                "data": dictionaryOf(lnUrlWithdrawRequestData: data),
            ]

        case let .lnUrlAuth(
            data
        ):
            return [
                "type": "lnUrlAuth",
                "data": dictionaryOf(lnUrlAuthRequestData: data),
            ]

        case let .lnUrlError(
            data
        ):
            return [
                "type": "lnUrlError",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]
        }
    }

    static func arrayOf(inputTypeList: [InputType]) -> [Any] {
        return inputTypeList.map { v -> [String: Any?] in return dictionaryOf(inputType: v) }
    }

    static func asInputTypeList(arr: [Any]) throws -> [InputType] {
        var list = [InputType]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var inputType = try asInputType(inputType: val)
                list.append(inputType)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "InputType"))
            }
        }
        return list
    }

    static func asLiquidNetwork(liquidNetwork: String) throws -> LiquidNetwork {
        switch liquidNetwork {
        case "mainnet":
            return LiquidNetwork.mainnet

        case "testnet":
            return LiquidNetwork.testnet

        default: throw SdkError.Generic(message: "Invalid variant \(liquidNetwork) for enum LiquidNetwork")
        }
    }

    static func valueOf(liquidNetwork: LiquidNetwork) -> String {
        switch liquidNetwork {
        case .mainnet:
            return "mainnet"

        case .testnet:
            return "testnet"
        }
    }

    static func arrayOf(liquidNetworkList: [LiquidNetwork]) -> [String] {
        return liquidNetworkList.map { v -> String in return valueOf(liquidNetwork: v) }
    }

    static func asLiquidNetworkList(arr: [Any]) throws -> [LiquidNetwork] {
        var list = [LiquidNetwork]()
        for value in arr {
            if let val = value as? String {
                var liquidNetwork = try asLiquidNetwork(liquidNetwork: val)
                list.append(liquidNetwork)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LiquidNetwork"))
            }
        }
        return list
    }

    static func asLnUrlCallbackStatus(lnUrlCallbackStatus: [String: Any?]) throws -> LnUrlCallbackStatus {
        let type = lnUrlCallbackStatus["type"] as! String
        if type == "ok" {
            return LnUrlCallbackStatus.ok
        }
        if type == "errorStatus" {
            guard let dataTmp = lnUrlCallbackStatus["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlCallbackStatus"))
            }
            let _data = try asLnUrlErrorData(lnUrlErrorData: dataTmp)

            return LnUrlCallbackStatus.errorStatus(data: _data)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum LnUrlCallbackStatus")
    }

    static func dictionaryOf(lnUrlCallbackStatus: LnUrlCallbackStatus) -> [String: Any?] {
        switch lnUrlCallbackStatus {
        case .ok:
            return [
                "type": "ok",
            ]

        case let .errorStatus(
            data
        ):
            return [
                "type": "errorStatus",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]
        }
    }

    static func arrayOf(lnUrlCallbackStatusList: [LnUrlCallbackStatus]) -> [Any] {
        return lnUrlCallbackStatusList.map { v -> [String: Any?] in return dictionaryOf(lnUrlCallbackStatus: v) }
    }

    static func asLnUrlCallbackStatusList(arr: [Any]) throws -> [LnUrlCallbackStatus] {
        var list = [LnUrlCallbackStatus]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlCallbackStatus = try asLnUrlCallbackStatus(lnUrlCallbackStatus: val)
                list.append(lnUrlCallbackStatus)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlCallbackStatus"))
            }
        }
        return list
    }

    static func asLnUrlPayResult(lnUrlPayResult: [String: Any?]) throws -> LnUrlPayResult {
        let type = lnUrlPayResult["type"] as! String
        if type == "endpointSuccess" {
            guard let dataTmp = lnUrlPayResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlPayResult"))
            }
            let _data = try asLnUrlPaySuccessData(lnUrlPaySuccessData: dataTmp)

            return LnUrlPayResult.endpointSuccess(data: _data)
        }
        if type == "endpointError" {
            guard let dataTmp = lnUrlPayResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlPayResult"))
            }
            let _data = try asLnUrlErrorData(lnUrlErrorData: dataTmp)

            return LnUrlPayResult.endpointError(data: _data)
        }
        if type == "payError" {
            guard let dataTmp = lnUrlPayResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlPayResult"))
            }
            let _data = try asLnUrlPayErrorData(lnUrlPayErrorData: dataTmp)

            return LnUrlPayResult.payError(data: _data)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum LnUrlPayResult")
    }

    static func dictionaryOf(lnUrlPayResult: LnUrlPayResult) -> [String: Any?] {
        switch lnUrlPayResult {
        case let .endpointSuccess(
            data
        ):
            return [
                "type": "endpointSuccess",
                "data": dictionaryOf(lnUrlPaySuccessData: data),
            ]

        case let .endpointError(
            data
        ):
            return [
                "type": "endpointError",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]

        case let .payError(
            data
        ):
            return [
                "type": "payError",
                "data": dictionaryOf(lnUrlPayErrorData: data),
            ]
        }
    }

    static func arrayOf(lnUrlPayResultList: [LnUrlPayResult]) -> [Any] {
        return lnUrlPayResultList.map { v -> [String: Any?] in return dictionaryOf(lnUrlPayResult: v) }
    }

    static func asLnUrlPayResultList(arr: [Any]) throws -> [LnUrlPayResult] {
        var list = [LnUrlPayResult]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlPayResult = try asLnUrlPayResult(lnUrlPayResult: val)
                list.append(lnUrlPayResult)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlPayResult"))
            }
        }
        return list
    }

    static func asLnUrlWithdrawResult(lnUrlWithdrawResult: [String: Any?]) throws -> LnUrlWithdrawResult {
        let type = lnUrlWithdrawResult["type"] as! String
        if type == "ok" {
            guard let dataTmp = lnUrlWithdrawResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlWithdrawResult"))
            }
            let _data = try asLnUrlWithdrawSuccessData(lnUrlWithdrawSuccessData: dataTmp)

            return LnUrlWithdrawResult.ok(data: _data)
        }
        if type == "timeout" {
            guard let dataTmp = lnUrlWithdrawResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlWithdrawResult"))
            }
            let _data = try asLnUrlWithdrawSuccessData(lnUrlWithdrawSuccessData: dataTmp)

            return LnUrlWithdrawResult.timeout(data: _data)
        }
        if type == "errorStatus" {
            guard let dataTmp = lnUrlWithdrawResult["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "LnUrlWithdrawResult"))
            }
            let _data = try asLnUrlErrorData(lnUrlErrorData: dataTmp)

            return LnUrlWithdrawResult.errorStatus(data: _data)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum LnUrlWithdrawResult")
    }

    static func dictionaryOf(lnUrlWithdrawResult: LnUrlWithdrawResult) -> [String: Any?] {
        switch lnUrlWithdrawResult {
        case let .ok(
            data
        ):
            return [
                "type": "ok",
                "data": dictionaryOf(lnUrlWithdrawSuccessData: data),
            ]

        case let .timeout(
            data
        ):
            return [
                "type": "timeout",
                "data": dictionaryOf(lnUrlWithdrawSuccessData: data),
            ]

        case let .errorStatus(
            data
        ):
            return [
                "type": "errorStatus",
                "data": dictionaryOf(lnUrlErrorData: data),
            ]
        }
    }

    static func arrayOf(lnUrlWithdrawResultList: [LnUrlWithdrawResult]) -> [Any] {
        return lnUrlWithdrawResultList.map { v -> [String: Any?] in return dictionaryOf(lnUrlWithdrawResult: v) }
    }

    static func asLnUrlWithdrawResultList(arr: [Any]) throws -> [LnUrlWithdrawResult] {
        var list = [LnUrlWithdrawResult]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var lnUrlWithdrawResult = try asLnUrlWithdrawResult(lnUrlWithdrawResult: val)
                list.append(lnUrlWithdrawResult)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "LnUrlWithdrawResult"))
            }
        }
        return list
    }

    static func asNetwork(network: String) throws -> Network {
        switch network {
        case "bitcoin":
            return Network.bitcoin

        case "testnet":
            return Network.testnet

        case "signet":
            return Network.signet

        case "regtest":
            return Network.regtest

        default: throw SdkError.Generic(message: "Invalid variant \(network) for enum Network")
        }
    }

    static func valueOf(network: Network) -> String {
        switch network {
        case .bitcoin:
            return "bitcoin"

        case .testnet:
            return "testnet"

        case .signet:
            return "signet"

        case .regtest:
            return "regtest"
        }
    }

    static func arrayOf(networkList: [Network]) -> [String] {
        return networkList.map { v -> String in return valueOf(network: v) }
    }

    static func asNetworkList(arr: [Any]) throws -> [Network] {
        var list = [Network]()
        for value in arr {
            if let val = value as? String {
                var network = try asNetwork(network: val)
                list.append(network)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "Network"))
            }
        }
        return list
    }

    static func asPaymentState(paymentState: String) throws -> PaymentState {
        switch paymentState {
        case "created":
            return PaymentState.created

        case "pending":
            return PaymentState.pending

        case "complete":
            return PaymentState.complete

        case "failed":
            return PaymentState.failed

        case "timedOut":
            return PaymentState.timedOut

        case "refundable":
            return PaymentState.refundable

        case "refundPending":
            return PaymentState.refundPending

        default: throw SdkError.Generic(message: "Invalid variant \(paymentState) for enum PaymentState")
        }
    }

    static func valueOf(paymentState: PaymentState) -> String {
        switch paymentState {
        case .created:
            return "created"

        case .pending:
            return "pending"

        case .complete:
            return "complete"

        case .failed:
            return "failed"

        case .timedOut:
            return "timedOut"

        case .refundable:
            return "refundable"

        case .refundPending:
            return "refundPending"
        }
    }

    static func arrayOf(paymentStateList: [PaymentState]) -> [String] {
        return paymentStateList.map { v -> String in return valueOf(paymentState: v) }
    }

    static func asPaymentStateList(arr: [Any]) throws -> [PaymentState] {
        var list = [PaymentState]()
        for value in arr {
            if let val = value as? String {
                var paymentState = try asPaymentState(paymentState: val)
                list.append(paymentState)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PaymentState"))
            }
        }
        return list
    }

    static func asPaymentType(paymentType: String) throws -> PaymentType {
        switch paymentType {
        case "receive":
            return PaymentType.receive

        case "send":
            return PaymentType.send

        default: throw SdkError.Generic(message: "Invalid variant \(paymentType) for enum PaymentType")
        }
    }

    static func valueOf(paymentType: PaymentType) -> String {
        switch paymentType {
        case .receive:
            return "receive"

        case .send:
            return "send"
        }
    }

    static func arrayOf(paymentTypeList: [PaymentType]) -> [String] {
        return paymentTypeList.map { v -> String in return valueOf(paymentType: v) }
    }

    static func asPaymentTypeList(arr: [Any]) throws -> [PaymentType] {
        var list = [PaymentType]()
        for value in arr {
            if let val = value as? String {
                var paymentType = try asPaymentType(paymentType: val)
                list.append(paymentType)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "PaymentType"))
            }
        }
        return list
    }

    static func asSdkEvent(sdkEvent: [String: Any?]) throws -> SdkEvent {
        let type = sdkEvent["type"] as! String
        if type == "paymentFailed" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentFailed(details: _details)
        }
        if type == "paymentPending" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentPending(details: _details)
        }
        if type == "paymentRefunded" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentRefunded(details: _details)
        }
        if type == "paymentRefundPending" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentRefundPending(details: _details)
        }
        if type == "paymentSucceeded" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentSucceeded(details: _details)
        }
        if type == "paymentWaitingConfirmation" {
            guard let detailsTmp = sdkEvent["details"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "details", typeName: "SdkEvent"))
            }
            let _details = try asPayment(payment: detailsTmp)

            return SdkEvent.paymentWaitingConfirmation(details: _details)
        }
        if type == "synced" {
            return SdkEvent.synced
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum SdkEvent")
    }

    static func dictionaryOf(sdkEvent: SdkEvent) -> [String: Any?] {
        switch sdkEvent {
        case let .paymentFailed(
            details
        ):
            return [
                "type": "paymentFailed",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentPending(
            details
        ):
            return [
                "type": "paymentPending",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentRefunded(
            details
        ):
            return [
                "type": "paymentRefunded",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentRefundPending(
            details
        ):
            return [
                "type": "paymentRefundPending",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentSucceeded(
            details
        ):
            return [
                "type": "paymentSucceeded",
                "details": dictionaryOf(payment: details),
            ]

        case let .paymentWaitingConfirmation(
            details
        ):
            return [
                "type": "paymentWaitingConfirmation",
                "details": dictionaryOf(payment: details),
            ]

        case .synced:
            return [
                "type": "synced",
            ]
        }
    }

    static func arrayOf(sdkEventList: [SdkEvent]) -> [Any] {
        return sdkEventList.map { v -> [String: Any?] in return dictionaryOf(sdkEvent: v) }
    }

    static func asSdkEventList(arr: [Any]) throws -> [SdkEvent] {
        var list = [SdkEvent]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var sdkEvent = try asSdkEvent(sdkEvent: val)
                list.append(sdkEvent)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "SdkEvent"))
            }
        }
        return list
    }

    static func asSuccessActionProcessed(successActionProcessed: [String: Any?]) throws -> SuccessActionProcessed {
        let type = successActionProcessed["type"] as! String
        if type == "aes" {
            guard let resultTmp = successActionProcessed["result"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "result", typeName: "SuccessActionProcessed"))
            }
            let _result = try asAesSuccessActionDataResult(aesSuccessActionDataResult: resultTmp)

            return SuccessActionProcessed.aes(result: _result)
        }
        if type == "message" {
            guard let dataTmp = successActionProcessed["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "SuccessActionProcessed"))
            }
            let _data = try asMessageSuccessActionData(messageSuccessActionData: dataTmp)

            return SuccessActionProcessed.message(data: _data)
        }
        if type == "url" {
            guard let dataTmp = successActionProcessed["data"] as? [String: Any?] else {
                throw SdkError.Generic(message: errMissingMandatoryField(fieldName: "data", typeName: "SuccessActionProcessed"))
            }
            let _data = try asUrlSuccessActionData(urlSuccessActionData: dataTmp)

            return SuccessActionProcessed.url(data: _data)
        }

        throw SdkError.Generic(message: "Unexpected type \(type) for enum SuccessActionProcessed")
    }

    static func dictionaryOf(successActionProcessed: SuccessActionProcessed) -> [String: Any?] {
        switch successActionProcessed {
        case let .aes(
            result
        ):
            return [
                "type": "aes",
                "result": dictionaryOf(aesSuccessActionDataResult: result),
            ]

        case let .message(
            data
        ):
            return [
                "type": "message",
                "data": dictionaryOf(messageSuccessActionData: data),
            ]

        case let .url(
            data
        ):
            return [
                "type": "url",
                "data": dictionaryOf(urlSuccessActionData: data),
            ]
        }
    }

    static func arrayOf(successActionProcessedList: [SuccessActionProcessed]) -> [Any] {
        return successActionProcessedList.map { v -> [String: Any?] in return dictionaryOf(successActionProcessed: v) }
    }

    static func asSuccessActionProcessedList(arr: [Any]) throws -> [SuccessActionProcessed] {
        var list = [SuccessActionProcessed]()
        for value in arr {
            if let val = value as? [String: Any?] {
                var successActionProcessed = try asSuccessActionProcessed(successActionProcessed: val)
                list.append(successActionProcessed)
            } else {
                throw SdkError.Generic(message: errUnexpectedType(typeName: "SuccessActionProcessed"))
            }
        }
        return list
    }

    static func hasNonNilKey(data: [String: Any?], key: String) -> Bool {
        if let val = data[key] {
            return !(val == nil || val is NSNull)
        }

        return false
    }

    static func errMissingMandatoryField(fieldName: String, typeName: String) -> String {
        return "Missing mandatory field \(fieldName) for type \(typeName)"
    }

    static func errUnexpectedType(typeName: String) -> String {
        return "Unexpected type \(typeName)"
    }

    static func errUnexpectedValue(fieldName: String) -> String {
        return "Unexpected value for optional field \(fieldName)"
    }
}

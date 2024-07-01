import BreezLiquidSDK
import Foundation

@objc(RNBreezLiquidSDK)
class RNBreezLiquidSDK: RCTEventEmitter {
    static let TAG: String = "BreezLiquidSDK"

    public static var emitter: RCTEventEmitter!
    public static var hasListeners: Bool = false
    public static var supportedEvents: [String] = ["breezLiquidSdkLog"]

    private var bindingLiquidSdk: BindingLiquidSdk!

    static var breezLiquidSdkDirectory: URL {
        let applicationDirectory = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first!
        let breezLiquidSdkDirectory = applicationDirectory.appendingPathComponent("breezLiquidSdk", isDirectory: true)

        if !FileManager.default.fileExists(atPath: breezLiquidSdkDirectory.path) {
            try! FileManager.default.createDirectory(atPath: breezLiquidSdkDirectory.path, withIntermediateDirectories: true)
        }

        return breezLiquidSdkDirectory
    }

    override init() {
        super.init()
        RNBreezLiquidSDK.emitter = self
    }

    @objc
    override static func moduleName() -> String! {
        TAG
    }

    static func addSupportedEvent(name: String) {
        RNBreezLiquidSDK.supportedEvents.append(name)
    }

    override func supportedEvents() -> [String]! {
        return RNBreezLiquidSDK.supportedEvents
    }

    override func startObserving() {
        RNBreezLiquidSDK.hasListeners = true
    }

    override func stopObserving() {
        RNBreezLiquidSDK.hasListeners = false
    }

    @objc
    override static func requiresMainQueueSetup() -> Bool {
        return false
    }

    func getBindingLiquidSdk() throws -> BindingLiquidSdk {
        if bindingLiquidSdk != nil {
            return bindingLiquidSdk
        }

        throw LiquidSdkError.Generic(message: "Not initialized")
    }

    private func ensureWorkingDir(workingDir: String) throws {
        do {
            if !FileManager.default.fileExists(atPath: workingDir) {
                try FileManager.default.createDirectory(atPath: workingDir, withIntermediateDirectories: true)
            }
        } catch {
            throw LiquidSdkError.Generic(message: "Mandatory field workingDir must contain a writable directory")
        }
    }

    @objc(defaultConfig:resolve:reject:)
    func defaultConfig(_ network: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let networkTmp = try BreezLiquidSDKMapper.asLiquidNetwork(liquidNetwork: network)
            var res = BreezLiquidSDK.defaultConfig(network: networkTmp)
            res.workingDir = RNBreezLiquidSDK.breezLiquidSdkDirectory.path
            resolve(BreezLiquidSDKMapper.dictionaryOf(config: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(parse:resolve:reject:)
    func parse(_ input: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try BreezLiquidSDK.parse(input: input)
            resolve(BreezLiquidSDKMapper.dictionaryOf(inputType: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(parseInvoice:resolve:reject:)
    func parseInvoice(_ input: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try BreezLiquidSDK.parseInvoice(input: input)
            resolve(BreezLiquidSDKMapper.dictionaryOf(lnInvoice: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(setLogger:reject:)
    func setLogger(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try BreezLiquidSDK.setLogger(logger: BreezLiquidSDKLogger())
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(connect:resolve:reject:)
    func connect(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        if bindingLiquidSdk != nil {
            reject("Generic", "Already initialized", nil)
            return
        }

        do {
            var connectRequest = try BreezLiquidSDKMapper.asConnectRequest(connectRequest: req)
            try ensureWorkingDir(workingDir: connectRequest.config.workingDir)

            bindingLiquidSdk = try BreezLiquidSDK.connect(req: connectRequest)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(addEventListener:reject:)
    func addEventListener(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var eventListener = BreezLiquidSDKEventListener()
            var res = try getBindingLiquidSdk().addEventListener(listener: eventListener)

            eventListener.setId(id: res)
            resolve(res)
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(removeEventListener:resolve:reject:)
    func removeEventListener(_ id: String, resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().removeEventListener(id: id)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(getInfo:reject:)
    func getInfo(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().getInfo()
            resolve(BreezLiquidSDKMapper.dictionaryOf(getInfoResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareSendPayment:resolve:reject:)
    func prepareSendPayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareSendRequest = try BreezLiquidSDKMapper.asPrepareSendRequest(prepareSendRequest: req)
            var res = try getBindingLiquidSdk().prepareSendPayment(req: prepareSendRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(prepareSendResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sendPayment:resolve:reject:)
    func sendPayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareSendResponse = try BreezLiquidSDKMapper.asPrepareSendResponse(prepareSendResponse: req)
            var res = try getBindingLiquidSdk().sendPayment(req: prepareSendResponse)
            resolve(BreezLiquidSDKMapper.dictionaryOf(sendPaymentResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareReceivePayment:resolve:reject:)
    func prepareReceivePayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareReceiveRequest = try BreezLiquidSDKMapper.asPrepareReceiveRequest(prepareReceiveRequest: req)
            var res = try getBindingLiquidSdk().prepareReceivePayment(req: prepareReceiveRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(prepareReceiveResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(receivePayment:resolve:reject:)
    func receivePayment(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareReceiveResponse = try BreezLiquidSDKMapper.asPrepareReceiveResponse(prepareReceiveResponse: req)
            var res = try getBindingLiquidSdk().receivePayment(req: prepareReceiveResponse)
            resolve(BreezLiquidSDKMapper.dictionaryOf(receivePaymentResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(fetchOnchainLimits:reject:)
    func fetchOnchainLimits(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().fetchOnchainLimits()
            resolve(BreezLiquidSDKMapper.dictionaryOf(onchainPaymentLimitsResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(preparePayOnchain:resolve:reject:)
    func preparePayOnchain(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let preparePayOnchainRequest = try BreezLiquidSDKMapper.asPreparePayOnchainRequest(preparePayOnchainRequest: req)
            var res = try getBindingLiquidSdk().preparePayOnchain(req: preparePayOnchainRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(preparePayOnchainResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(payOnchain:resolve:reject:)
    func payOnchain(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let payOnchainRequest = try BreezLiquidSDKMapper.asPayOnchainRequest(payOnchainRequest: req)
            var res = try getBindingLiquidSdk().payOnchain(req: payOnchainRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(sendPaymentResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareReceiveOnchain:resolve:reject:)
    func prepareReceiveOnchain(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareReceiveOnchainRequest = try BreezLiquidSDKMapper.asPrepareReceiveOnchainRequest(prepareReceiveOnchainRequest: req)
            var res = try getBindingLiquidSdk().prepareReceiveOnchain(req: prepareReceiveOnchainRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(prepareReceiveOnchainResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(receiveOnchain:resolve:reject:)
    func receiveOnchain(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareReceiveOnchainResponse = try BreezLiquidSDKMapper.asPrepareReceiveOnchainResponse(prepareReceiveOnchainResponse: req)
            var res = try getBindingLiquidSdk().receiveOnchain(req: prepareReceiveOnchainResponse)
            resolve(BreezLiquidSDKMapper.dictionaryOf(receiveOnchainResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listPayments:reject:)
    func listPayments(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().listPayments()
            resolve(BreezLiquidSDKMapper.arrayOf(paymentList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listRefundables:reject:)
    func listRefundables(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().listRefundables()
            resolve(BreezLiquidSDKMapper.arrayOf(refundableSwapList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(prepareRefund:resolve:reject:)
    func prepareRefund(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let prepareRefundRequest = try BreezLiquidSDKMapper.asPrepareRefundRequest(prepareRefundRequest: req)
            var res = try getBindingLiquidSdk().prepareRefund(req: prepareRefundRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(prepareRefundResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(refund:resolve:reject:)
    func refund(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let refundRequest = try BreezLiquidSDKMapper.asRefundRequest(refundRequest: req)
            var res = try getBindingLiquidSdk().refund(req: refundRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(refundResponse: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(rescanOnchainSwaps:reject:)
    func rescanOnchainSwaps(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().rescanOnchainSwaps()
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(sync:reject:)
    func sync(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().sync()
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(backup:resolve:reject:)
    func backup(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let backupRequest = try BreezLiquidSDKMapper.asBackupRequest(backupRequest: req)
            try getBindingLiquidSdk().backup(req: backupRequest)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(restore:resolve:reject:)
    func restore(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let restoreRequest = try BreezLiquidSDKMapper.asRestoreRequest(restoreRequest: req)
            try getBindingLiquidSdk().restore(req: restoreRequest)
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(disconnect:reject:)
    func disconnect(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            try getBindingLiquidSdk().disconnect()
            bindingLiquidSdk = nil
            resolve(["status": "ok"])
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(lnurlPay:resolve:reject:)
    func lnurlPay(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let lnUrlPayRequest = try BreezLiquidSDKMapper.asLnUrlPayRequest(lnUrlPayRequest: req)
            var res = try getBindingLiquidSdk().lnurlPay(req: lnUrlPayRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(lnUrlPayResult: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(lnurlWithdraw:resolve:reject:)
    func lnurlWithdraw(_ req: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let lnUrlWithdrawRequest = try BreezLiquidSDKMapper.asLnUrlWithdrawRequest(lnUrlWithdrawRequest: req)
            var res = try getBindingLiquidSdk().lnurlWithdraw(req: lnUrlWithdrawRequest)
            resolve(BreezLiquidSDKMapper.dictionaryOf(lnUrlWithdrawResult: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(lnurlAuth:resolve:reject:)
    func lnurlAuth(_ reqData: [String: Any], resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            let lnUrlAuthRequestData = try BreezLiquidSDKMapper.asLnUrlAuthRequestData(lnUrlAuthRequestData: reqData)
            var res = try getBindingLiquidSdk().lnurlAuth(reqData: lnUrlAuthRequestData)
            resolve(BreezLiquidSDKMapper.dictionaryOf(lnUrlCallbackStatus: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(fetchFiatRates:reject:)
    func fetchFiatRates(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().fetchFiatRates()
            resolve(BreezLiquidSDKMapper.arrayOf(rateList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    @objc(listFiatCurrencies:reject:)
    func listFiatCurrencies(_ resolve: @escaping RCTPromiseResolveBlock, reject: @escaping RCTPromiseRejectBlock) {
        do {
            var res = try getBindingLiquidSdk().listFiatCurrencies()
            resolve(BreezLiquidSDKMapper.arrayOf(fiatCurrencyList: res))
        } catch let err {
            rejectErr(err: err, reject: reject)
        }
    }

    func rejectErr(err: Error, reject: @escaping RCTPromiseRejectBlock) {
        var errorName = "Generic"
        var message = "\(err)"
        if let errAssociated = Mirror(reflecting: err).children.first {
            errorName = errAssociated.label ?? errorName
            if let associatedMessage = Mirror(reflecting: errAssociated.value).children.first {
                message = associatedMessage.value as! String
            }
        }
        reject(errorName, message, err)
    }
}

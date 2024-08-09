import UserNotifications
import Foundation

struct LnurlInvoiceRequest: Codable {
    let reply_url: String
    let amount: UInt64
}

struct LnurlInvoiceResponse: Decodable, Encodable {
    let pr: String
    let routes: [String]
    
    init(pr: String, routes: [String]) {
        self.pr = pr
        self.routes = routes
    }
}

class LnurlPayInvoiceTask : LnurlPayTask {
    fileprivate let TAG = "LnurlPayInvoiceTask"
    
    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        let successNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_INVOICE_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_INVOICE_NOTIFICATION_TITLE)
        let failNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE)
        super.init(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent, successNotificationTitle: successNotificationTitle, failNotificationTitle: failNotificationTitle)
    }
    
    override func start(liquidSDK: BindingLiquidSdk) throws {
        var request: LnurlInvoiceRequest? = nil
        do {
            request = try JSONDecoder().decode(LnurlInvoiceRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to decode payload: \(e)", level: "ERROR")
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_LNURL_PAY)
            throw e
        }
        
        do {
            // Get the lightning limits
            let limits = try liquidSDK.fetchLightningLimits()
            // Check amount is within limits
            let amountSat = request!.amount / UInt64(1000)
            if amountSat < limits.receive.minSat || amountSat > limits.receive.maxSat {
                fail(withError: "Invalid amount requested \(request!.amount)", replyURL: request!.reply_url)
                return
            }
            let plainTextMetadata = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_METADATA_PLAIN_TEXT, fallback: Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT)
            let metadata = "[[\"text/plain\",\"\(plainTextMetadata)\"]]"
            let prepareReceivePaymentRes = try liquidSDK.prepareReceivePayment(req: PrepareReceivePaymentRequest(payerAmountSat: amountSat))
            let receivePaymentRes = try liquidSDK.receivePayment(req: ReceivePaymentRequest(prepareRes: prepareReceivePaymentRes, description: metadata, useDescriptionHash: true))
            self.replyServer(encodable: LnurlInvoiceResponse(pr: receivePaymentRes.invoice, routes: []), replyURL: request!.reply_url)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to process lnurl: \(e)", level: "ERROR")
            self.fail(withError: e.localizedDescription, replyURL: request!.reply_url)
        }
    }
}

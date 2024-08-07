import UserNotifications
import Foundation

struct LnurlInfoRequest: Codable {
    let callback_url: String
    let reply_url: String
}

struct LnurlInfoResponse: Decodable, Encodable {
    let callback: String
    let maxSendable: UInt64
    let minSendable: UInt64
    let metadata: String
    let tag: String
    
    init(callback: String, maxSendable: UInt64, minSendable: UInt64, metadata: String, tag: String) {
        self.callback = callback
        self.maxSendable = maxSendable
        self.minSendable = minSendable
        self.metadata = metadata
        self.tag = tag
    }
}

class LnurlPayInfoTask : LnurlPayTask {
    fileprivate let TAG = "LnurlPayInfoTask"
    
    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        let successNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_INFO_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_INFO_NOTIFICATION_TITLE)
        let failNotificationTitle = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_LNURL_PAY_NOTIFICATION_FAILURE_TITLE)
        super.init(payload: payload, logger: logger, contentHandler: contentHandler, bestAttemptContent: bestAttemptContent, successNotificationTitle: successNotificationTitle, failNotificationTitle: failNotificationTitle)
    }
    
    override func start(liquidSDK: BindingLiquidSdk) throws {
        var request: LnurlInfoRequest? = nil
        do {
            request = try JSONDecoder().decode(LnurlInfoRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to decode payload: \(e)", level: "ERROR")
            self.displayPushNotification(title: self.failNotificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_LNURL_PAY)
            throw e
        }
        
        do {
            // Get the lightning limits
            let limits = try liquidSDK.fetchLightningLimits()
            let maxSendableMsat = limits.receive.maxSat * UInt64(1000)
            let minSendableMsat = limits.receive.minSat * UInt64(1000)
            // Format the response
            let plainTextMetadata = ResourceHelper.shared.getString(key: Constants.LNURL_PAY_METADATA_PLAIN_TEXT, fallback: Constants.DEFAULT_LNURL_PAY_METADATA_PLAIN_TEXT)
            let metadata = "[[\"text/plain\",\"\(plainTextMetadata)\"]]"
            replyServer(encodable: LnurlInfoResponse(callback: request!.callback_url, maxSendable: maxSendableMsat, minSendable: minSendableMsat, metadata: metadata, tag: "payRequest"),
                        replyURL: request!.reply_url)
        } catch let e {
            self.logger.log(tag: TAG, line: "failed to process lnurl: \(e)", level: "ERROR")
            fail(withError: e.localizedDescription, replyURL: request!.reply_url)
        }
    }
}

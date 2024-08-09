import UserNotifications
import Foundation

struct SwapUpdatedRequest: Codable {
    let id: String
    let status: String
}

class SwapUpdatedTask : TaskProtocol {
    fileprivate let TAG = "SwapUpdatedTask"
    
    internal var payload: String
    internal var contentHandler: ((UNNotificationContent) -> Void)?
    internal var bestAttemptContent: UNMutableNotificationContent?
    internal var logger: ServiceLogger
    internal var request: SwapUpdatedRequest? = nil
    
    init(payload: String, logger: ServiceLogger, contentHandler: ((UNNotificationContent) -> Void)? = nil, bestAttemptContent: UNMutableNotificationContent? = nil) {
        self.payload = payload
        self.contentHandler = contentHandler
        self.bestAttemptContent = bestAttemptContent
        self.logger = logger
    }
    
    func start(liquidSDK: BindingLiquidSdk) throws {
        do {
            self.request = try JSONDecoder().decode(SwapUpdatedRequest.self, from: self.payload.data(using: .utf8)!)
        } catch let e {
            self.logger.log(tag: TAG, line: "Failed to decode payload: \(e)", level: "ERROR")
            self.onShutdown()
            throw e
        }
    }

    public func onEvent(e: SdkEvent) {
        if let swapIdHash = self.request?.id {
            switch e {
            case .paymentSucceeded(details: let payment):
                if swapIdHash == payment.swapId?.sha256() {
                    self.logger.log(tag: TAG, line: "Received payment succeeded event: \(swapIdHash)", level: "INFO")
                    self.notifySuccess()
                }
                break
            default:
                break
            }
        }
    }

    func onShutdown() {
        let notificationTitle = ResourceHelper.shared.getString(key: Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE, fallback: Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TITLE)
        let notificationBody = ResourceHelper.shared.getString(key: Constants.SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT, fallback: Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_FAILURE_TEXT)
        self.displayPushNotification(title: notificationTitle, body: notificationBody, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_SWAP_UPDATED)
    }

    func notifySuccess() {
        if let swapIdHash = self.request?.id {
            self.logger.log(tag: TAG, line: "Swap \(swapIdHash) processed successfully", level: "INFO")
            let notificationTitle = ResourceHelper.shared.getString(key: Constants.SWAP_CONFIRMED_NOTIFICATION_TITLE, fallback: Constants.DEFAULT_SWAP_CONFIRMED_NOTIFICATION_TITLE)
            self.displayPushNotification(title: notificationTitle, logger: self.logger, threadIdentifier: Constants.NOTIFICATION_THREAD_SWAP_UPDATED)
        }
    }
}

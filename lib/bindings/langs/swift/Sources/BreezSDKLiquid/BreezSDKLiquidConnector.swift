import Foundation
import os.log

#if DEBUG && true
fileprivate var logger = OSLog(
    subsystem: Bundle.main.bundleIdentifier!,
    category: "BreezSDKLiquidConnector"
)
#else
fileprivate var logger = OSLog.disabled
#endif

class BreezSDKLiquidConnector {
    private static var liquidSDK: BindingLiquidSdk? = nil
    fileprivate static var queue = DispatchQueue(label: "BreezSDKLiquidConnector")
    fileprivate static var sdkListener: EventListener? = nil
    
    static func register(connectRequest: ConnectRequest, listener: EventListener) throws -> BindingLiquidSdk? {
        try BreezSDKLiquidConnector.queue.sync { [] in
            BreezSDKLiquidConnector.sdkListener = listener
            if BreezSDKLiquidConnector.liquidSDK == nil {
                BreezSDKLiquidConnector.liquidSDK = try BreezSDKLiquidConnector.connectSDK(connectRequest: connectRequest)
            }
            return BreezSDKLiquidConnector.liquidSDK
        }
    }
    
    static func unregister() {
        BreezSDKLiquidConnector.queue.sync { [] in
            BreezSDKLiquidConnector.sdkListener = nil
        }
    }
    
    static func connectSDK(connectRequest: ConnectRequest) throws -> BindingLiquidSdk? {
        // Connect to the Breez Liquid SDK make it ready for use
        os_log("Connecting to Breez Liquid SDK", log: logger, type: .debug)
        let liquidSDK = try connect(req: connectRequest)
        os_log("Connected to Breez Liquid SDK", log: logger, type: .debug)
        let _ = try liquidSDK.addEventListener(listener: BreezSDKEventListener())
        return liquidSDK
    }
}

class BreezSDKEventListener: EventListener {
    func onEvent(e: SdkEvent) {
        BreezSDKLiquidConnector.queue.async { [] in
            BreezSDKLiquidConnector.sdkListener?.onEvent(e: e)
        }
    }
}

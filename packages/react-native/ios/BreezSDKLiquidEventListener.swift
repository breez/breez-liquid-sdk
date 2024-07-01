import Foundation
import BreezSDKLiquid

class BreezSDKEventListener: EventListener {
    private var id: String?

    func setId(id: String) {
        self.id = id
        RNBreezSDKLiquid.addSupportedEvent(name: "event-\(id)")
    }

    func onEvent(e: SdkEvent) {
        if let id = self.id {
            if RNBreezSDKLiquid.hasListeners {
                RNBreezSDKLiquid.emitter.sendEvent(withName: "event-\(id)",
                                                   body: BreezSDKLiquidMapper.dictionaryOf(sdkEvent: e))
            }
        }
    }
}

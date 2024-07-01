
export type EventListener = (e: SdkEvent) => void

export type Logger = (logEntry: LogEntry) => void

export const connect = async (req: ConnectRequest): Promise<void> => {
    const response = await BreezSDKLiquid.connect(req)
    return response
}

export const addEventListener = async (listener: EventListener): Promise<string> => {
    const response = await BreezSDKLiquid.addEventListener()
    BreezSDKLiquidEmitter.addListener(`event-${response}`, listener)
    
    return response
}

export const setLogger = async (logger: Logger): Promise<EmitterSubscription> => {
    const subscription = BreezSDKLiquidEmitter.addListener("breezSdkLiquidLog", logger)

    try {
        await BreezSDKLiquid.setLogger()
    } catch {}

    return subscription
}
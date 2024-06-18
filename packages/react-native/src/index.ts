import { NativeModules, Platform, EmitterSubscription, NativeEventEmitter } from "react-native"

const LINKING_ERROR =
    `The package 'react-native-breez-liquid-sdk' doesn't seem to be linked. Make sure: \n\n` +
    Platform.select({ ios: "- You have run 'pod install'\n", default: "" }) +
    "- You rebuilt the app after installing the package\n" +
    "- You are not using Expo managed workflow\n"

const BreezLiquidSDK = NativeModules.RNBreezLiquidSDK
    ? NativeModules.RNBreezLiquidSDK
    : new Proxy(
          {},
          {
              get() {
                  throw new Error(LINKING_ERROR)
              }
          }
      )

const BreezLiquidSDKEmitter = new NativeEventEmitter(BreezLiquidSDK)

export interface AesSuccessActionDataDecrypted {
    description: string
    plaintext: string
}

export interface BackupRequest {
    backupPath?: string
}

export interface BitcoinAddressData {
    address: string
    network: Network
    amountSat?: number
    label?: string
    message?: string
}

export interface Config {
    boltzUrl: string
    electrumUrl: string
    workingDir: string
    network: LiquidSdkNetwork
    paymentTimeoutSec: number
    zeroConfMinFeeRate: number
    zeroConfMaxAmountSat?: number
}

export interface ConnectRequest {
    config: Config
    mnemonic: string
}

export interface GetInfoResponse {
    balanceSat: number
    pendingSendSat: number
    pendingReceiveSat: number
    pubkey: string
}

export interface LnInvoice {
    bolt11: string
    network: Network
    payeePubkey: string
    paymentHash: string
    description?: string
    descriptionHash?: string
    amountMsat?: number
    timestamp: number
    expiry: number
    routingHints: RouteHint[]
    paymentSecret: number[]
    minFinalCltvExpiryDelta: number
}

export interface LnUrlAuthRequestData {
    k1: string
    domain: string
    url: string
    action?: string
}

export interface LnUrlErrorData {
    reason: string
}

export interface LnUrlPayErrorData {
    paymentHash: string
    reason: string
}

export interface LnUrlPayRequest {
    data: LnUrlPayRequestData
    amountMsat: number
    comment?: string
    paymentLabel?: string
}

export interface LnUrlPayRequestData {
    callback: string
    minSendable: number
    maxSendable: number
    metadataStr: string
    commentAllowed: number
    domain: string
    allowsNostr: boolean
    nostrPubkey?: string
    lnAddress?: string
}

export interface LnUrlPaySuccessData {
    successAction?: SuccessActionProcessed
    payment: Payment
}

export interface LnUrlWithdrawRequest {
    data: LnUrlWithdrawRequestData
    amountMsat: number
    description?: string
}

export interface LnUrlWithdrawRequestData {
    callback: string
    k1: string
    defaultDescription: string
    minWithdrawable: number
    maxWithdrawable: number
}

export interface LnUrlWithdrawSuccessData {
    invoice: LnInvoice
}

export interface LogEntry {
    line: string
    level: string
}

export interface MessageSuccessActionData {
    message: string
}

export interface Payment {
    txId?: string
    swapId?: string
    timestamp: number
    amountSat: number
    feesSat: number
    preimage?: string
    refundTxId?: string
    refundTxAmountSat?: number
    paymentType: PaymentType
    status: PaymentState
}

export interface PrepareReceiveRequest {
    payerAmountSat: number
}

export interface PrepareReceiveResponse {
    payerAmountSat: number
    feesSat: number
}

export interface PrepareSendRequest {
    invoice: string
}

export interface PrepareSendResponse {
    invoice: string
    feesSat: number
}

export interface ReceivePaymentResponse {
    id: string
    invoice: string
}

export interface RestoreRequest {
    backupPath?: string
}

export interface RouteHint {
    hops: RouteHintHop[]
}

export interface RouteHintHop {
    srcNodeId: string
    shortChannelId: number
    feesBaseMsat: number
    feesProportionalMillionths: number
    cltvExpiryDelta: number
    htlcMinimumMsat?: number
    htlcMaximumMsat?: number
}

export interface SendPaymentResponse {
    payment: Payment
}

export interface UrlSuccessActionData {
    description: string
    url: string
}

export enum AesSuccessActionDataResultVariant {
    DECRYPTED = "decrypted",
    ERROR_STATUS = "errorStatus"
}

export type AesSuccessActionDataResult = {
    type: AesSuccessActionDataResultVariant.DECRYPTED,
    data: AesSuccessActionDataDecrypted
} | {
    type: AesSuccessActionDataResultVariant.ERROR_STATUS,
    reason: string
}

export enum InputTypeVariant {
    BITCOIN_ADDRESS = "bitcoinAddress",
    BOLT11 = "bolt11",
    NODE_ID = "nodeId",
    URL = "url",
    LN_URL_PAY = "lnUrlPay",
    LN_URL_WITHDRAW = "lnUrlWithdraw",
    LN_URL_AUTH = "lnUrlAuth",
    LN_URL_ERROR = "lnUrlError"
}

export type InputType = {
    type: InputTypeVariant.BITCOIN_ADDRESS,
    address: BitcoinAddressData
} | {
    type: InputTypeVariant.BOLT11,
    invoice: LnInvoice
} | {
    type: InputTypeVariant.NODE_ID,
    nodeId: string
} | {
    type: InputTypeVariant.URL,
    url: string
} | {
    type: InputTypeVariant.LN_URL_PAY,
    data: LnUrlPayRequestData
} | {
    type: InputTypeVariant.LN_URL_WITHDRAW,
    data: LnUrlWithdrawRequestData
} | {
    type: InputTypeVariant.LN_URL_AUTH,
    data: LnUrlAuthRequestData
} | {
    type: InputTypeVariant.LN_URL_ERROR,
    data: LnUrlErrorData
}

export enum LiquidSdkEventVariant {
    PAYMENT_FAILED = "paymentFailed",
    PAYMENT_PENDING = "paymentPending",
    PAYMENT_REFUNDED = "paymentRefunded",
    PAYMENT_REFUND_PENDING = "paymentRefundPending",
    PAYMENT_SUCCEEDED = "paymentSucceeded",
    PAYMENT_WAITING_CONFIRMATION = "paymentWaitingConfirmation",
    SYNCED = "synced"
}

export type LiquidSdkEvent = {
    type: LiquidSdkEventVariant.PAYMENT_FAILED,
    details: Payment
} | {
    type: LiquidSdkEventVariant.PAYMENT_PENDING,
    details: Payment
} | {
    type: LiquidSdkEventVariant.PAYMENT_REFUNDED,
    details: Payment
} | {
    type: LiquidSdkEventVariant.PAYMENT_REFUND_PENDING,
    details: Payment
} | {
    type: LiquidSdkEventVariant.PAYMENT_SUCCEEDED,
    details: Payment
} | {
    type: LiquidSdkEventVariant.PAYMENT_WAITING_CONFIRMATION,
    details: Payment
} | {
    type: LiquidSdkEventVariant.SYNCED
}

export enum LiquidSdkNetwork {
    MAINNET = "mainnet",
    TESTNET = "testnet"
}

export enum LnUrlCallbackStatusVariant {
    OK = "ok",
    ERROR_STATUS = "errorStatus"
}

export type LnUrlCallbackStatus = {
    type: LnUrlCallbackStatusVariant.OK
} | {
    type: LnUrlCallbackStatusVariant.ERROR_STATUS,
    data: LnUrlErrorData
}

export enum LnUrlPayResultVariant {
    ENDPOINT_SUCCESS = "endpointSuccess",
    ENDPOINT_ERROR = "endpointError",
    PAY_ERROR = "payError"
}

export type LnUrlPayResult = {
    type: LnUrlPayResultVariant.ENDPOINT_SUCCESS,
    data: LnUrlPaySuccessData
} | {
    type: LnUrlPayResultVariant.ENDPOINT_ERROR,
    data: LnUrlErrorData
} | {
    type: LnUrlPayResultVariant.PAY_ERROR,
    data: LnUrlPayErrorData
}

export enum LnUrlWithdrawResultVariant {
    OK = "ok",
    ERROR_STATUS = "errorStatus"
}

export type LnUrlWithdrawResult = {
    type: LnUrlWithdrawResultVariant.OK,
    data: LnUrlWithdrawSuccessData
} | {
    type: LnUrlWithdrawResultVariant.ERROR_STATUS,
    data: LnUrlErrorData
}

export enum Network {
    BITCOIN = "bitcoin",
    TESTNET = "testnet",
    SIGNET = "signet",
    REGTEST = "regtest"
}

export enum PaymentState {
    CREATED = "created",
    PENDING = "pending",
    COMPLETE = "complete",
    FAILED = "failed",
    TIMED_OUT = "timedOut"
}

export enum PaymentType {
    RECEIVE = "receive",
    SEND = "send"
}

export enum SuccessActionProcessedVariant {
    AES = "aes",
    MESSAGE = "message",
    URL = "url"
}

export type SuccessActionProcessed = {
    type: SuccessActionProcessedVariant.AES,
    result: AesSuccessActionDataResult
} | {
    type: SuccessActionProcessedVariant.MESSAGE,
    data: MessageSuccessActionData
} | {
    type: SuccessActionProcessedVariant.URL,
    data: UrlSuccessActionData
}

export type EventListener = (e: LiquidSdkEvent) => void

export type Logger = (logEntry: LogEntry) => void

export const connect = async (req: ConnectRequest): Promise<void> => {
    const response = await BreezLiquidSDK.connect(req)
    return response
}

export const addEventListener = async (listener: EventListener): Promise<string> => {
    const response = await BreezLiquidSDK.addEventListener()
    BreezLiquidSDKEmitter.addListener(`event-${response}`, listener)

    return response
}

export const setLogger = async (logger: Logger): Promise<EmitterSubscription> => {
    const subscription = BreezLiquidSDKEmitter.addListener("breezLiquidSdkLog", logger)

    try {
        await BreezLiquidSDK.setLogger()
    } catch {}

    return subscription
}

export const defaultConfig = async (network: LiquidSdkNetwork): Promise<Config> => {
    const response = await BreezLiquidSDK.defaultConfig(network)
    return response
}

export const parse = async (input: string): Promise<InputType> => {
    const response = await BreezLiquidSDK.parse(input)
    return response
}

export const parseInvoice = async (input: string): Promise<LnInvoice> => {
    const response = await BreezLiquidSDK.parseInvoice(input)
    return response
}


export const removeEventListener = async (id: string): Promise<void> => {
    await BreezLiquidSDK.removeEventListener(id)
}

export const getInfo = async (): Promise<GetInfoResponse> => {
    const response = await BreezLiquidSDK.getInfo()
    return response
}

export const prepareSendPayment = async (req: PrepareSendRequest): Promise<PrepareSendResponse> => {
    const response = await BreezLiquidSDK.prepareSendPayment(req)
    return response
}

export const sendPayment = async (req: PrepareSendResponse): Promise<SendPaymentResponse> => {
    const response = await BreezLiquidSDK.sendPayment(req)
    return response
}

export const prepareReceivePayment = async (req: PrepareReceiveRequest): Promise<PrepareReceiveResponse> => {
    const response = await BreezLiquidSDK.prepareReceivePayment(req)
    return response
}

export const receivePayment = async (req: PrepareReceiveResponse): Promise<ReceivePaymentResponse> => {
    const response = await BreezLiquidSDK.receivePayment(req)
    return response
}

export const listPayments = async (): Promise<Payment[]> => {
    const response = await BreezLiquidSDK.listPayments()
    return response
}

export const sync = async (): Promise<void> => {
    await BreezLiquidSDK.sync()
}

export const backup = async (req: BackupRequest): Promise<void> => {
    await BreezLiquidSDK.backup(req)
}

export const restore = async (req: RestoreRequest): Promise<void> => {
    await BreezLiquidSDK.restore(req)
}

export const disconnect = async (): Promise<void> => {
    await BreezLiquidSDK.disconnect()
}

export const lnurlPay = async (req: LnUrlPayRequest): Promise<LnUrlPayResult> => {
    const response = await BreezLiquidSDK.lnurlPay(req)
    return response
}

export const lnurlWithdraw = async (req: LnUrlWithdrawRequest): Promise<LnUrlWithdrawResult> => {
    const response = await BreezLiquidSDK.lnurlWithdraw(req)
    return response
}

export const lnurlAuth = async (reqData: LnUrlAuthRequestData): Promise<LnUrlCallbackStatus> => {
    const response = await BreezLiquidSDK.lnurlAuth(reqData)
    return response
}

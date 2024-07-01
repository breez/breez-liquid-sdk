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
    liquidElectrumUrl: string
    bitcoinElectrumUrl: string
    workingDir: string
    network: LiquidNetwork
    paymentTimeoutSec: number
    zeroConfMinFeeRate: number
    zeroConfMaxAmountSat?: number
}

export interface ConnectRequest {
    config: Config
    mnemonic: string
}

export interface CurrencyInfo {
    name: string
    fractionSize: number
    spacing?: number
    symbol?: SymbolType
    uniqSymbol?: SymbolType
    localizedName: LocalizedName[]
    localeOverrides: LocaleOverrides[]
}

export interface FiatCurrency {
    id: string
    info: CurrencyInfo
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

export interface LocaleOverrides {
    locale: string
    spacing?: number
    symbol: SymbolType
}

export interface LocalizedName {
    locale: string
    name: string
}

export interface LogEntry {
    line: string
    level: string
}

export interface MessageSuccessActionData {
    message: string
}

export interface OnchainPaymentLimitsResponse {
    sendMinPayerAmountSat: number
    sendMaxPayerAmountSat: number
    sendMaxPayerAmountSatZeroConf: number
    receiveMinPayerAmountSat: number
    receiveMaxPayerAmountSat: number
    receiveMaxPayerAmountSatZeroConf: number
}

export interface PayOnchainRequest {
    address: string
    prepareRes: PreparePayOnchainResponse
}

export interface Payment {
    txId?: string
    swapId?: string
    timestamp: number
    amountSat: number
    feesSat: number
    preimage?: string
    bolt11?: string
    refundTxId?: string
    refundTxAmountSat?: number
    paymentType: PaymentType
    status: PaymentState
}

export interface PreparePayOnchainRequest {
    receiverAmountSat: number
}

export interface PreparePayOnchainResponse {
    receiverAmountSat: number
    feesSat: number
}

export interface PrepareReceiveOnchainRequest {
    payerAmountSat: number
}

export interface PrepareReceiveOnchainResponse {
    payerAmountSat: number
    feesSat: number
}

export interface PrepareReceiveRequest {
    payerAmountSat: number
}

export interface PrepareReceiveResponse {
    payerAmountSat: number
    feesSat: number
}

export interface PrepareRefundRequest {
    swapAddress: string
    refundAddress: string
    satPerVbyte: number
}

export interface PrepareRefundResponse {
    txVsize: number
    txFeeSat: number
    refundTxId?: string
}

export interface PrepareSendRequest {
    invoice: string
}

export interface PrepareSendResponse {
    invoice: string
    feesSat: number
}

export interface Rate {
    coin: string
    value: number
}

export interface ReceiveOnchainResponse {
    address: string
    bip21: string
}

export interface ReceivePaymentResponse {
    id: string
    invoice: string
}

export interface RefundRequest {
    swapAddress: string
    refundAddress: string
    satPerVbyte: number
}

export interface RefundResponse {
    refundTxId: string
}

export interface RefundableSwap {
    swapAddress: string
    timestamp: number
    amountSat: number
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

export interface SymbolType {
    grapheme?: string
    template?: string
    rtl?: boolean
    position?: number
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

export enum LiquidNetwork {
    MAINNET = "mainnet",
    TESTNET = "testnet"
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
    TIMED_OUT = "timedOut",
    REFUNDABLE = "refundable",
    REFUND_PENDING = "refundPending"
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

export const defaultConfig = async (network: LiquidNetwork): Promise<Config> => {
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

export const fetchOnchainLimits = async (): Promise<OnchainPaymentLimitsResponse> => {
    const response = await BreezLiquidSDK.fetchOnchainLimits()
    return response
}

export const preparePayOnchain = async (req: PreparePayOnchainRequest): Promise<PreparePayOnchainResponse> => {
    const response = await BreezLiquidSDK.preparePayOnchain(req)
    return response
}

export const payOnchain = async (req: PayOnchainRequest): Promise<SendPaymentResponse> => {
    const response = await BreezLiquidSDK.payOnchain(req)
    return response
}

export const prepareReceiveOnchain = async (req: PrepareReceiveOnchainRequest): Promise<PrepareReceiveOnchainResponse> => {
    const response = await BreezLiquidSDK.prepareReceiveOnchain(req)
    return response
}

export const receiveOnchain = async (req: PrepareReceiveOnchainResponse): Promise<ReceiveOnchainResponse> => {
    const response = await BreezLiquidSDK.receiveOnchain(req)
    return response
}

export const listPayments = async (): Promise<Payment[]> => {
    const response = await BreezLiquidSDK.listPayments()
    return response
}

export const listRefundables = async (): Promise<RefundableSwap[]> => {
    const response = await BreezLiquidSDK.listRefundables()
    return response
}

export const prepareRefund = async (req: PrepareRefundRequest): Promise<PrepareRefundResponse> => {
    const response = await BreezLiquidSDK.prepareRefund(req)
    return response
}

export const refund = async (req: RefundRequest): Promise<RefundResponse> => {
    const response = await BreezLiquidSDK.refund(req)
    return response
}

export const rescanOnchainSwaps = async (): Promise<void> => {
    await BreezLiquidSDK.rescanOnchainSwaps()
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

export const fetchFiatRates = async (): Promise<Rate[]> => {
    const response = await BreezLiquidSDK.fetchFiatRates()
    return response
}

export const listFiatCurrencies = async (): Promise<FiatCurrency[]> => {
    const response = await BreezLiquidSDK.listFiatCurrencies()
    return response
}

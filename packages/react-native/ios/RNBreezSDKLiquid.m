#import <React/RCTBridgeModule.h>
#import <React/RCTEventEmitter.h>

@interface RCT_EXTERN_MODULE(RNBreezSDKLiquid, RCTEventEmitter)

RCT_EXTERN_METHOD(
    defaultConfig: (NSString*)network
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    parse: (NSString*)input
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    parseInvoice: (NSString*)input
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)
  
RCT_EXTERN_METHOD(
    setLogger: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    connect: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    addEventListener: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    removeEventListener: (NSString*)id
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    getInfo: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    prepareSendPayment: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    sendPayment: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    prepareReceivePayment: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    receivePayment: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    fetchLightningLimits: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    fetchOnchainLimits: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    preparePayOnchain: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    payOnchain: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    prepareBuyBitcoin: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    buyBitcoin: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    listPayments: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    listRefundables: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    prepareRefund: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    refund: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    rescanOnchainSwaps: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    sync: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    recommendedFees: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    backup: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    restore: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    disconnect: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    lnurlPay: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    lnurlWithdraw: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    lnurlAuth: (NSDictionary*)reqData
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    fetchFiatRates: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    listFiatCurrencies: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

@end
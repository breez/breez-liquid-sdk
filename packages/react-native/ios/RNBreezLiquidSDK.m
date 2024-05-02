#import <React/RCTBridgeModule.h>
#import <React/RCTEventEmitter.h>

@interface RCT_EXTERN_MODULE(RNBreezLiquidSDK, RCTEventEmitter)
  
RCT_EXTERN_METHOD(
    connect: (NSString*)mnemonic
    dataDir: (NSString*)dataDir
    network: (NSString*)network
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    getInfo: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
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
    backup: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

RCT_EXTERN_METHOD(
    restore: (NSDictionary*)req
    resolve: (RCTPromiseResolveBlock)resolve
    reject: (RCTPromiseRejectBlock)reject
)

@end
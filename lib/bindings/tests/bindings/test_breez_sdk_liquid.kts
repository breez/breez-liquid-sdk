
class SDKListener: breez_sdk_liquid.EventListener {
    override fun onEvent(e: breez_sdk_liquid.SdkEvent) {
        println(e.toString());
    }
}

try {
    var mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    var config = breez_sdk_liquid.defaultConfig(breez_sdk_liquid.LiquidNetwork.TESTNET)
    var connectRequest = breez_sdk_liquid.ConnectRequest(config, mnemonic)
    var sdk = breez_sdk_liquid.connect(connectRequest)

    var listenerId = sdk.addEventListener(SDKListener())

    var nodeInfo = sdk.getInfo()

    sdk.removeEventListener(listenerId)

    println("$nodeInfo")
    assert(nodeInfo.pubkey.equals("03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494"))
} catch (ex: Exception) {
    throw RuntimeException(ex.toString())
}
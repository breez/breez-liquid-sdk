
class SDKListener: breez_liquid_sdk.EventListener {
    override fun onEvent(e: breez_liquid_sdk.LiquidSdkEvent) {
        println(e.toString());
    }
}

try {
    var mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

    var connectReq = breez_liquid_sdk.ConnectRequest(mnemonic, breez_liquid_sdk.Network.LIQUID_TESTNET)
    var sdk = breez_liquid_sdk.connect(connectReq)

    var listenerId = sdk.addEventListener(SDKListener())

    var getInfoReq = breez_liquid_sdk.GetInfoRequest(false)
    var nodeInfo = sdk.getInfo(getInfoReq)

    sdk.removeEventListener(listenerId)

    println("$nodeInfo")
    assert(nodeInfo.pubkey.equals("03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494"))
} catch (ex: Exception) {
    throw RuntimeException(ex.toString())
}
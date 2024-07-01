import breez_sdk_liquid

class SDKListener: EventListener {
    func onEvent(e: SdkEvent) {
        print("Received event ", e);
    }
}

let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
let config = breez_sdk_liquid.defaultConfig(network: .testnet);
let connectRequest = breez_sdk_liquid.ConnectRequest(config: config, mnemonic: mnemonic);
let sdk = try breez_sdk_liquid.connect(req: connectRequest);

let listenerId = try sdk.addEventListener(listener: SDKListener());

let nodeInfo = try sdk.getInfo();

try sdk.removeEventListener(id: listenerId);

print(nodeInfo);
assert(nodeInfo.pubkey == "03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494", "nodeInfo.pubkey");
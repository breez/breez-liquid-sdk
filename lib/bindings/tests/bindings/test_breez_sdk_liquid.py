import breez_sdk_liquid


class SDKListener(breez_sdk_liquid.EventListener):
   def on_event(self, event):
      print(event)


def test():
    mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    config = breez_sdk_liquid.default_config(breez_sdk_liquid.LiquidNetwork.TESTNET)
    connect_request = breez_sdk_liquid.ConnectRequest(config=config, mnemonic=mnemonic)
    sdk = breez_sdk_liquid.connect(connect_request)

    listener_id = sdk.add_event_listener(SDKListener())

    node_info = sdk.get_info()

    sdk.remove_event_listener(listener_id)

    print(node_info)
    assert node_info.pubkey == "03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494"

test()
import breez_liquid_sdk


class SDKListener(breez_liquid_sdk.EventListener):
   def on_event(self, event):
      print(event)


def test():
    mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

    connect_req = breez_liquid_sdk.ConnectRequest(mnemonic=mnemonic, network=breez_liquid_sdk.Network.LIQUID_TESTNET)
    sdk = breez_liquid_sdk.connect(connect_req)

    listener_id = sdk.add_event_listener(SDKListener())

    get_info_req = breez_liquid_sdk.GetInfoRequest(with_scan=False)
    node_info = sdk.get_info(get_info_req)

    sdk.remove_event_listener(listener_id)

    print(node_info)
    assert node_info.pubkey == "03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494"

test()
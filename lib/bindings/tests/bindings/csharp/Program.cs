
using Breez.Sdk.Liquid;

try
{
    var mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    var config = BreezSdkLiquidMethods.DefaultConfig(LiquidNetwork.Testnet);

    var connectReq = new ConnectRequest(config, mnemonic);
    BindingLiquidSdk sdk = BreezSdkLiquidMethods.Connect(connectReq);

    GetInfoResponse? info = sdk.GetInfo();

    Console.WriteLine(info!.pubkey);
}
catch (Exception e)
{
    Console.WriteLine(e.Message);
}

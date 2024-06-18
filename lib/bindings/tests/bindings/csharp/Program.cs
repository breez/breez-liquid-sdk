
using Breez.Liquid.Sdk;

try
{
    var mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    var config = BreezLiquidSdkMethods.DefaultConfig(Network.Testnet);

    var connectReq = new ConnectRequest(config, mnemonic);
    BindingLiquidSdk sdk = BreezLiquidSdkMethods.Connect(connectReq);

    GetInfoResponse? info = sdk.GetInfo();

    Console.WriteLine(info!.pubkey);
}
catch (Exception e)
{
    Console.WriteLine(e.Message);
}

package main

import (
	"log"

	"example.org/golang/breez_liquid_sdk"
)

func main() {
	mnemonic := "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
	
	config := breez_liquid_sdk.DefaultConfig(breez_liquid_sdk.NetworkTestnet)

	sdk, err := breez_liquid_sdk.Connect(breez_liquid_sdk.ConnectRequest{
		Config: config,
		Mnemonic: mnemonic,
	})

	if err != nil {
		log.Fatalf("Connect failed: %#v", err)
	}

	info, err := sdk.GetInfo()

	if err != nil {
		log.Fatalf("GetInfo failed: %#v", err)
	}


	log.Print(info.Pubkey)
}

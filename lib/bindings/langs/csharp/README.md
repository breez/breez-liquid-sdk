# Breez Liquid SDK — C#.Net Bindings

## Usage
```
dotnet add package Breez.Sdk.Liquid
```

## Create a package
Run the GitHub workflow 'Publish C# Bindings' when creating a new release of Breez Liquid SDK.
It will create an artifact containing a zip file with the nuget package in it.

## Publish package

- Manually upload the package to the breeztech nuget organization, or
- `dotnet nuget push ./bin/Debug/Breez.Sdk.Liquid.{version}.nupkg --api-key PUT-API-KEY-HERE --source https://api.nuget.org/v3/index.json`
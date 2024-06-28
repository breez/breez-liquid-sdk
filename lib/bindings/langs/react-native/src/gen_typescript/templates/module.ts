import { NativeModules, Platform, EmitterSubscription, NativeEventEmitter } from "react-native"

const LINKING_ERROR =
    `The package 'react-native-breez-liquid-sdk' doesn't seem to be linked. Make sure: \n\n` +
    Platform.select({ ios: "- You have run 'pod install'\n", default: "" }) +
    "- You rebuilt the app after installing the package\n" +
    "- You are not using Expo managed workflow\n"

const BreezSDKLiquid = NativeModules.RNBreezSDKLiquid
    ? NativeModules.RNBreezSDKLiquid
    : new Proxy(
          {},
          {
              get() {
                  throw new Error(LINKING_ERROR)
              }
          }
      )

const BreezSDKLiquidEmitter = new NativeEventEmitter(BreezSDKLiquid)
{%- import "macros.ts" as ts %}
{%- include "Types.ts" %}
{% include "Helpers.ts" %}
{% for func in ci.function_definitions() %}
{%- if func.name()|ignored_function == false -%}
{%- include "TopLevelFunctionTemplate.ts" %}
{% endif -%}
{% endfor -%}
{%- include "Objects.ts" %}

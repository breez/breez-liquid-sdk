# Breez Liquid SDK React Native Example

## Build

Run the npm/yarn install to download dependences:
```bash
yarn
```
or
```bash
npm i
```

## Run

### Android

```bash
yarn android
```

#### Android Troubleshooting

* Before running `yarn android`, stop any `Metro` instances that may be running.
* If you get the error
  ```
  Failed to load dynamic library 'libbreez_sdk_liquid_bindings.so': dlopen failed: cannot locate symbol "__extenddftf2"
  ```
  that is likely due to a dependency issue affecting x86_64 images. Try to run the app on a physical Android device or on a x86 image.

### iOS

```bash
yarn pods
yarn ios
```

## Development

To develop the Breez Liquid SDK alongside the React Native module and example app, please read [DEVELOPING.md](../DEVELOPING.md) for details on how to setup your development environment.

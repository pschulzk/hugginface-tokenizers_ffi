# Tokenizer FFI

Tokenizer FFI is a Rust-based tokenization library with Flutter Foreign Function Interface (FFI) bindings. It provides a high-performance tokenization solution for Flutter applications, leveraging the speed and memory safety of Rust while seamlessly integrating with Flutter's Dart code.

## Original Source

This project is based on the Tokenizers library from Hugging Face. The original implementation can be found at:

[https://github.com/huggingface/tokenizers](https://github.com/huggingface/tokenizers)

Adapted the Rust implementation to work with Flutter via FFI, allowing for efficient tokenization in mobile applications.

## Features

- Fast tokenization using Rust implementation from Hugging Face
- Cross-platform support for Android and iOS
- Embedded tokenizer model for offline usage
- Flutter FFI bindings for easy integration

## Building

### Prerequisites

- Rust toolchain with cargo-ndk and cargo-lipo
- Android NDK for Android builds
- Xcode for iOS builds
- Flutter SDK

### Preparing the Environment

### lib.rs

```rust
// Embed the tokenizer JSON as a byte array
static TOKENIZER_JSON: &[u8] = include_bytes!("path/to/huggingface-tokenizers/tokenizers/tokenizer.json");
```

```bash
rustup target add aarch64-apple-ios x86_64-apple-ios armv7-linux-androideabi aarch64-linux-android
cargo install cargo-ndk cargo-lipo
```

```bash
cargo build --release
```

### Android Build

```bash
cargo ndk -t arm64-v8a build --release
```

### iOS Build

```bash
cargo lipo --release --targets aarch64-apple-ios
```

## Example Usage in Flutter

pubspec.yaml:

```yaml
dependencies:
  ffi: ^2.1.3
  tflite_flutter: ^0.11.0
```

```dart
import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';

typedef TokenizeFunc = Pointer<Utf8> Function(Pointer<Utf8> input);
typedef TokenizeFuncDart = Pointer<Utf8> Function(Pointer<Utf8> input);

typedef FreeStringFunc = Void Function(Pointer<Utf8>);
typedef FreeStringFuncDart = void Function(Pointer<Utf8>);

class TokenizerFFI {
  late DynamicLibrary _lib;
  late TokenizeFuncDart _tokenize;
  late FreeStringFuncDart _freeString;

  TokenizerFFI() {
    _loadLibrary();
    _loadFunctions();
  }

  void _loadLibrary() {
    try {
      if (Platform.isAndroid) {
        _lib = DynamicLibrary.open('libtokenizers_ffi.so');
      } else if (Platform.isIOS) {
        _lib = DynamicLibrary.process();
      } else {
        throw UnsupportedError('Unsupported platform');
      }
    } catch (e) {
      print('Error loading library: $e');
      rethrow;
    }
  }

  void _loadFunctions() {
    try {
      _tokenize = _lib.lookupFunction<TokenizeFunc, TokenizeFuncDart>('tokenize');
      _freeString = _lib.lookupFunction<FreeStringFunc, FreeStringFuncDart>('free_string');
    } catch (e) {
      print('Error loading functions: $e');
      rethrow;
    }
  }

  List<int> tokenize(String input) {
    final inputPointer = input.toNativeUtf8();

    try {
      final resultPointer = _tokenize(inputPointer);
      final result = resultPointer.toDartString();
      _freeString(resultPointer);

      if (result == "Error during tokenization") {
        throw Exception("Tokenization failed in native code");
      }

      print("Tokenization result length: ${result.length}");

      final tokens = result.split(',').map((s) => int.parse(s)).toList();
      print("Number of tokens: ${tokens.length}");

      return tokens;
    } catch (e) {
      print('Error during tokenization: $e');
      rethrow;
    } finally {
      calloc.free(inputPointer);
    }
  }
}
```

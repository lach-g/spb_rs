## Developer Dependencies

### Rust

rustc >= 1.75

### OpenSSL

Dependency from openssl-sys crate feature of "default" build to speed up the build. Look to https://github.com/eclipse/paho.mqtt.rust for more information on build tips if running into build errors.

#### Windows

1. Install vcpkg
```shell
> git clone https://github.com/microsoft/vcpkg.git
> .\vcpkg\bootstrap-vcpkg.bat
```

2. Install openssl
```shell
> .\vcpkg\vcpkg install openssl:x64-windows
```

3. Add system environment variable. Change path to your own install location:
```
OPENSSL_DIR=C:\Users\user\path\to\vcpkg\packages\openssl_x64-windows
```

### Docker

To run integration tests in ./tests folder Docker must be installed.
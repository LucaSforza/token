# Token

<!--toc:start-->

- [Token](#token)
  - [Installation](#installation)
    - [Docker](#docker)
    - [Podman](#podman)
    - [Brew](#brew)
    - [Manual](#manual)
  - [Documentation](#documentation)
  <!--toc:end-->

## Installation

### Docker

```bash
  docker pull imgname
```

### Podman

```bash
  podman pull imgname
```

### Brew

```bash
 brew Install TODO
```

### Manual

> [!CAUTION]
> install build dependencies on your distribution foundry,rust,gcc,g++

```bash
#Install foundry
curl -L https://foundry.paradigm.xyz | bash & & \
forge bind --crate-name auction  && \
# build  the binary
cargo build --release
```

## Documentation

we use :

- <https://book.getfoundry.sh/>

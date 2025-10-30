# Token

<!--toc:start-->

- [Token](#token)
  - [Description](#description)
  - [Installation](#installation)
    - [Docker](#docker)
    - [Brew](#brew)
    - [Manual](#manual)
  - [Usage](#usage)
    - [Docker](#docker)
    - [Anything else](#anything-else)
  - [Documentation](#documentation)
  <!--toc:end-->

## Description

_Cli_ tool for our blockchain course

## Installation

> [!NOTE]
> we recommend [docker](https://www.docker.com/products/docker-desktop/) for the best experience,
> [podman](https://podman-desktop.io/downloads/) is also supported.

### Docker

```bash
  docker pull docker.io/ar33d/auction_controller:base
```

### Brew

```bash
 brew tap LucaSforza/auction-controller &&  brew install lucasforza/auction-controller/auction_controller
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

## Usage

### Docker

```bash
 docker run --rm auction_controller:base  -a 0xeDCB9D33923EFd291534b74112fD99299BC7aEC4 -p 2050d2bde2a46a47d5cab597029c04c6ac0710a4f0976724f976e02506c51d39 -e 0x9f5A46DAB47760F8938a936eBc585e5Be1Ed68bD -r https://ethereum-sepolia-rpc.publicnode.com winner
```

### Anything else

```bash
auction_controller -a 0xeDCB9D33923EFd291534b74112fD99299BC7aEC4 -p 2050d2bde2a46a47d5cab597029c04c6ac0710a4f0976724f976e02506c51d39 -e 0x9f5A46DAB47760F8938a936eBc585e5Be1Ed68bD -r https://ethereum-sepolia-rpc.publicnode.com winner
```

```bash
 # this should be the result when you run the command
 Winner: 0x03237997256f8088aC568b4A66F307A7A001D3a6
```

## Documentation

we use :

- <https://book.getfoundry.sh/>

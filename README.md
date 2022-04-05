# Web3 Diagram

## **This is a cargo utility that allows developers to generate visualizations depicting data flows within their NEAR Rust smart contracts.**

<br>

### Which problem does it solve?
**Provides the ability to generate representations of methods to allow for anyone to quickly understand how the contract works.**

<br>

### Who can use this?
**Smart contract developers use it to generate the images, but the images can help anyone trying to understand what’s going on.**

<br>

## Key Features

| Feature | Description |
| ----------- | ----------- |
| View Functions | Near smart contract view only functions |
| Mutation | Near smart contract mutable functions |
| Process | Helper or utility functions, also functions out of smart contract scope |
| Events | Functions that falls into near event [standard]([https://link](https://github.com/near/near-sdk-rs/blob/master/near-contract-standards/src/event.rs)) |
| View Trait Impl | View only functions but trait implamentation |
| Mutation Trait Impl | Mutable functions but trait implamentation |
| Payable | Functions that except Near |
| Initializers | Smart contract initializer functions |

<br>

## Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
<br>

## Install cli with [brew](https://brew.sh/)

```bash
brew install mermaid-cli
```

<br>

## Install [node](https://gist.github.com/tomysmile/da3cb8194ec8f0e4df86#install-node)

```bash
brew install node
```

<br>

## Web3 Utility Usage


### 1) Clone raffle smart contract from near examples

```bash
git clone git@github.com:near-examples/rust-ft-raffle.git
```

<br>

### 2) Go to the root directory
```bash
cd rust-ft-raffle
```

### 3) Run utility againt cloned smart contract and open in web browser
```bash
web3d -O --input raffle.md
```

<br>

## Options

| Key/Command | Description |
| ----------- | ----------- |
| -b, --backgroundColor | Background color. Example: transparent, red, '#F0F0F0'. Optional. Default: white |
| -h, --help | Print help information |
| -H, --height | Height of the page. Optional. Default: 600 |
| -i, --input  | Markdown file name |
| -o, --output | Output file name. It should be either md, svg, png or pdf. Optional. Default:'./res/input_file_name.svg' |
| -O, --openb | Should open output file in browser |
| -q, --quiet | Suppress log output |
| -w, --width | Width of the page. Optional. Default: 800 |
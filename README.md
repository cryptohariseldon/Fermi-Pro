
# Fermi V1

Central-limit order-book program based on [Openbook V2]([https://github.com/openbook-dex/openbook-v2]) with redefined accounting and trasfer structures to implement an orderbook of intents, with just-in-time settlement.


## Deployed versions

| tag  | network | program ID                                  |
| ---- | ------- | ------------------------------------------- ||
| v1.0 | devnet  | 6pYD7cBvgQMCBHWQaKzL7k1qfBuG9RpFB2hmbszd4u1A |
| v1.1 | devnet | o9QBwW81vjiH22NWLpLZm23ifn5itMGz9Hka49YoJkv |


To test it, we recommend using the [Fermi SDK](https://github.com/Fermi-DEX/fermi-sdk/tree/main)
Frontend available at: https://fermilabs.xyz/ 




## Building & testing

### Pre-requisites

Before you can build the program, you will first need to install the following:

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/docs/installation) (v0.28.0)
- [Just](https://github.com/casey/just#installation)

### Installing

To install the repo, run:

```bash
git clone https://github.com/openbook-dex/openbook-v2.git --recursive
```

The recursive flag ensures that you receive all of the submodules. If you have already cloned without passing in this flag, you can run:

```bash
git submodule init
git submodule update
```

To ensure that you always have the latest submodules, you can configure your git like so:

```bash
git config --global submodule.recurse true
```

### Building

To build, run:

```bash
anchor build
```

### IDL

To generate the progam & typescript IDLs, run:

```bash
just idl
```

### Testing

To see whether all of the tests are passing, run:

```bash
just test-all
```

To drill down on a specific test (e.g., test_expired_order), run:

```bash
just test test_expired_order
```

If you want to have tests that automatically re-run when you edit a file, install
[entr](https://github.com/eradman/entr) and run:

```bash
just test-dev
```

### TS Client

```bash
yarn build

```

## License

See the [LICENSE file](LICENSE).

The majority of this repo is MIT-licensed, but some parts needed for compiling
the Solana program are under GPL.

All GPL code is gated behind the `enable-gpl` feature. If you use the `openbook-v2`
crate as a dependency with the `client` or `cpi` features, you use only MIT
parts of it.

The intention is for you to be able to depend on the `openbook-v2` crate for
building closed-source tools and integrations, including other Solana programs
that call into the Openbook program.

But deriving a Solana program with similar functionality to the Openbook program
from this codebase would require the changes and improvements to stay publicly
available under GPL.


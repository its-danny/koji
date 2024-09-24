# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -

## [3.0.0](https://github.com/cococonscious/koji/compare/v2.2.0...v3.0.0) - 2024-09-24

### Added

- *(gitignore)* add intellij, vim, vscode and git ([#96](https://github.com/cococonscious/koji/pull/96))
- *(cargo)* add documentation and repository links ([#88](https://github.com/cococonscious/koji/pull/88))

### Changed

- type filtering, multi-line body support ([#99](https://github.com/cococonscious/koji/pull/99))
- non-default vendored-openssl feature, pin cocogitto ([#98](https://github.com/cococonscious/koji/pull/98))
- shorter test asserts, vendored openssl, complete workflow overhaul ([#94](https://github.com/cococonscious/koji/pull/94))
- update repository url, badges and license ([#93](https://github.com/cococonscious/koji/pull/93))
- *(readme)* looking for maintainer
- use asdf
- use devenv

## [2.2.0](https://github.com/cococonscious/koji/compare/2.1.0..2.2.0) - 2024-01-06
#### Build system
- **(deps)** update cocogitto to 6.0 - ([503759c](https://github.com/cococonscious/koji/commit/503759c6368e85b73682c8792c272297eea897ee)) - Danny Tatom

- - -

## [2.1.0](https://github.com/cococonscious/koji/compare/2.0.0..2.1.0) - 2023-11-24
#### Features
- allow signing commits - ([66b9d9e](https://github.com/cococonscious/koji/commit/66b9d9e42e8b44b895e535a8ceaf2d399f2fbbee)) - Danny Tatom

- - -

## [2.0.0](https://github.com/cococonscious/koji/compare/1.5.3..2.0.0) - 2023-11-23
#### Build system
- **(deps)** update indexmap to 2.1 - ([72ba166](https://github.com/cococonscious/koji/commit/72ba1660980ab8da47b1b0d1aac6cacd8a5ea3c7)) - Danny Tatom
- **(deps)** update git2 to 0.18 - ([e05f7f7](https://github.com/cococonscious/koji/commit/e05f7f77b1b93c27cf5072b460620e8eaeefc534)) - Danny Tatom
- **(deps)** update toml to 0.8 - ([a757f3c](https://github.com/cococonscious/koji/commit/a757f3c009cff3d4a3a69dadde88cacdd3111837)) - Danny Tatom
- **(deps)** update deps, remove patch constraints - ([3ad9f87](https://github.com/cococonscious/koji/commit/3ad9f874a6b7443c280351b121fb618b8c2fa14a)) - Danny Tatom
- **(deps)** update clap and emojis - ([0475b64](https://github.com/cococonscious/koji/commit/0475b64668581d1208bec378ac092241a67de4c2)) - Danny Tatom
#### Documentation
- **(readme)** add new options - ([fcbbaae](https://github.com/cococonscious/koji/commit/fcbbaae75e5f2bf0dd3da64e25043763e3f5f946)) - Danny Tatom
- **(readme)** fix CI badge - ([29da976](https://github.com/cococonscious/koji/commit/29da97655acdede47aa74b9e0d797b9d79c2c462)) - Danny Tatom
- **(readme)** update demo gif - ([82a4b6f](https://github.com/cococonscious/koji/commit/82a4b6fcd7562597d18797aa409474671662fc23)) - Danny Tatom
#### Features
- add ability to skip breaking changes and issues questions - ([8cc6b30](https://github.com/cococonscious/koji/commit/8cc6b30f00e8cf5e128e48a6e8c49f15794df9dc)) - Danny Tatom
#### Refactoring
- **(config)** clean up config - ([02a60d3](https://github.com/cococonscious/koji/commit/02a60d36bb5a66dbaf81377d74ea51da49f6b9ab)) - Danny Tatom

- - -

## [1.5.3](https://github.com/cococonscious/koji/compare/1.5.2..1.5.3) - 2022-10-02
#### Build system
- **(deps)** update all the deps - ([af5687f](https://github.com/cococonscious/koji/commit/af5687f8d7c9a15ba53c6f9598be312948cfebd1)) - Danny Tatom

- - -

## [1.5.2](https://github.com/cococonscious/koji/compare/1.5.1..1.5.2) - 2022-08-11
#### Build system
- **(deps)** update all the deps - ([3033a04](https://github.com/cococonscious/koji/commit/3033a047fc6322d8508bbe28e189421e474ae920)) - Danny Tatom
- **(deps)** update rust to 1.61.0 - ([ca49ee8](https://github.com/cococonscious/koji/commit/ca49ee8c8eaf8228a5b910f8be5b0f34f2d3f450)) - Danny Tatom
- remove rust-toolchain file - ([b6d638b](https://github.com/cococonscious/koji/commit/b6d638b21831cd3eacb4e2783ee6ac3dad7c7035)) - Danny Tatom
#### Features
- finish cleaning up config - ([e108265](https://github.com/cococonscious/koji/commit/e1082657d68d4d40417d47cf21579d41e280a5ab)) - Danny Tatom
#### Miscellaneous Chores
- **(docs)** remove extra config examples - ([1ddbe21](https://github.com/cococonscious/koji/commit/1ddbe21e7deb3fdc4e019cce7c369e7e8d149dfe)) - Danny Tatom
- fix typo in comment - ([92c8bd7](https://github.com/cococonscious/koji/commit/92c8bd74972181102cd5bda6904e1a83b9fba43a)) - Danny Tatom
#### Refactoring
- clean up emoji handling - ([a7aaad9](https://github.com/cococonscious/koji/commit/a7aaad9c07369e2b4721ec0f58e2b8ef524b35a0)) - Danny Tatom
- start cleaning up config - ([6f2d2b0](https://github.com/cococonscious/koji/commit/6f2d2b04e4d6f12b2a23cdd8e46f9c01fa970d48)) - Danny Tatom
- move commit code to its own file - ([d6f91d7](https://github.com/cococonscious/koji/commit/d6f91d70fb75ffcc118427d2fad181c6f8fd8571)) - Danny Tatom
- clean up comments - ([68713ad](https://github.com/cococonscious/koji/commit/68713adc871403ccd23aa1472dd8cd303bb79978)) - Danny Tatom
- disable default features of cocogitto - ([94dc806](https://github.com/cococonscious/koji/commit/94dc80660e92efeb811c855dd7acc7205eb77e58)) - Danny Tatom
- replace linked-hash-map with indexmap - ([0b4d689](https://github.com/cococonscious/koji/commit/0b4d689c6d9dee871134c7a69957cc8f120e275d)) - Danny Tatom

- - -

## [1.5.1](https://github.com/cococonscious/koji/compare/1.5.0..1.5.1) - 2022-05-01
#### Bug Fixes
- only early return with message if we're in hook mode - ([94c156d](https://github.com/cococonscious/koji/commit/94c156d6ca291073869a03dab83e761a6c9e36f9)) - Danny Tatom
#### Documentation
- **(readme)** update hook usage - ([8547437](https://github.com/cococonscious/koji/commit/85474374b4bad97465b396359191247caa541f9a)) - Danny Tatom
#### Miscellaneous Chores
- add desc and license to cargo file - ([3d2bf72](https://github.com/cococonscious/koji/commit/3d2bf729aa6818a0eeaaf6a2f0ee239bae3cd6e8)) - Danny Tatom

- - -

## [1.5.0](https://github.com/cococonscious/koji/compare/1.4.0..1.5.0) - 2022-05-01
#### Bug Fixes
- use git2 to get repo dir - ([db5fa44](https://github.com/cococonscious/koji/commit/db5fa449b832c4a75bc264efe1e1e189519adb0b)) - Danny Tatom
#### Build system
- **(cog)** add post bump hooks - ([7e1cc59](https://github.com/cococonscious/koji/commit/7e1cc59cb8acd3acb3a7fd33d3879015ba799265)) - Danny Tatom
- **(deps)** update all the deps - ([4c438ea](https://github.com/cococonscious/koji/commit/4c438eab192b8453ec10a746b4f0d0f254377160)) - Danny Tatom
- **(deps)** update rust to 1.58.1 - ([2feb1be](https://github.com/cococonscious/koji/commit/2feb1be8b704837305a13e996f997f1b8875d46e)) - Danny
- **(deps)** update rust to 1.58.0 - ([2c5e6b7](https://github.com/cococonscious/koji/commit/2c5e6b7f3f51478fa818ce11f1e71caa17cd034f)) - Danny Tatom
- **(deps)** update clap to 3.0.0 - ([76231fb](https://github.com/cococonscious/koji/commit/76231fbff4e031ec77da3064514fadf805bf8719)) - Danny Tatom
- **(deps)** update requestty to 0.2.1 - ([e2daea9](https://github.com/cococonscious/koji/commit/e2daea959caada8da638b6dee22fd6aa30ff95da)) - Danny Tatom
#### Continuous Integration
- **(workflow)** rename build script - ([93764ae](https://github.com/cococonscious/koji/commit/93764aea001e180211881100b7afdeaaec50017b)) - Danny Tatom
- **(workflow)** remove audit workflow - ([2bad853](https://github.com/cococonscious/koji/commit/2bad853c2fdd2686ff6d926650e24e3deebc4acf)) - Danny Tatom
- **(workflows)** redo how releases work - ([02c5177](https://github.com/cococonscious/koji/commit/02c5177ec61a68179ddf06e7ba415f722f760e79)) - Danny Tatom
#### Documentation
- **(changelog)** clean up names - ([f8eaa31](https://github.com/cococonscious/koji/commit/f8eaa31d9b29b7cda3b50458aae408fd538bc0c5)) - Danny Tatom
- **(readme)** better explain how the git hook works - ([8d08912](https://github.com/cococonscious/koji/commit/8d08912611e8fb951b593a8de54c406ccd6b753f)) - Danny Tatom
- **(readme)** explain git hook usage - ([b12ff65](https://github.com/cococonscious/koji/commit/b12ff65857a841eae955126aa195bad5622428dd)) - Danny Tatom
- **(readme)** add config options - ([e2a3b76](https://github.com/cococonscious/koji/commit/e2a3b7603891c01c597f948821f238b531f6b2f9)) - Danny Tatom
- **(readme)** clean up a bit - ([418dbb8](https://github.com/cococonscious/koji/commit/418dbb890fb5cbd4cc5b38a44e0dbebc5ab473e2)) - Danny Tatom
- **(readme)** clean up - ([7f14e74](https://github.com/cococonscious/koji/commit/7f14e743f79bf1be50941306492256c79d669adc)) - Danny Tatom
- **(readme)** update - ([3ace4c7](https://github.com/cococonscious/koji/commit/3ace4c74f94d87f654655ed24a89d40cb7ffd71d)) - Danny Tatom
- **(readme)** add link for getting it working on M1 - ([69573be](https://github.com/cococonscious/koji/commit/69573be783c695d31fff49392b9926910812e84d)) - Danny Tatom
- **(security)** remove example committing secrets - ([a6b22dd](https://github.com/cococonscious/koji/commit/a6b22ddbabe8d3b34e97b0904a490892ab005c9d)) - AJ ONeal
#### Features
- **(config)** better config handling - ([d6ad1b9](https://github.com/cococonscious/koji/commit/d6ad1b9010c7fd7e5f693f0ed5e8b72b2df91a17)) - Danny Tatom
- return early if commit message is already conventional - ([caff83d](https://github.com/cococonscious/koji/commit/caff83d1cdce1516ee2f6e4e2cfbbd95a0d87a58)) - Danny Tatom
- use message passed in via -m flag - ([fe58e51](https://github.com/cococonscious/koji/commit/fe58e514e136633d95d9427a363288c282e38c81)) - Danny Tatom
#### Refactoring
- clippy cleanup - ([d12ad21](https://github.com/cococonscious/koji/commit/d12ad219f1e59d9c07efdb1676996f754d7cee64)) - Danny Tatom
#### Tests
- add test for replace_emoji_shortcodes - ([f3948e5](https://github.com/cococonscious/koji/commit/f3948e5d2a0bf348470304c20638318606faaab3)) - Danny Tatom
- add more tests for prompt - ([932bc4c](https://github.com/cococonscious/koji/commit/932bc4cdc1fad25ca090b4fd63430d1d709f3664)) - Danny Tatom

- - -

## 1.4.0 - 2021-12-29


### Documentation

7bc7a6 - add notes for autocomplete - Danny Tatom

1ed5dd - fix badge url - Danny Tatom

cdc36b - fix typo - Danny Tatom

622b5b - update feature list - Danny Tatom

2c2670 - fix typo - Danny Tatom


### Features

7de38d - add support for emoji shortcodes - Danny Tatom

920181 - add optional autocomplete for scope prompt - Danny Tatom


### Tests

92ad3b - remove silly test - Danny Tatom

fcf3f1 - fix tests - Danny Tatom

7ae71c - move get_conventional_message assertion to existing test - Danny Tatom

e56d87 - add a (redundant?) test for get_conventional_message - Danny Tatom

601dcd - add test for get_extracted_answers - Danny Tatom

dbfe90 - add test for get_commit_types - Danny Tatom


### Bug Fixes

75aaca - fix typo in help - Danny Tatom


### Continuous Integration

28b4c1 - fix codecov ignore path - Danny Tatom

84a9c1 - add codecov config file - Danny Tatom


### Build system

4994be - update clap to 3.0.0-rc.8 - Danny Tatom


### Refactoring

f1a000 - destructure get_extracted_answers return value - Danny Tatom

edd03f - clean up load_config - Danny Tatom

4cde0f - move some stuff around - Danny Tatom

4b0158 - little bit of code cleanup - Danny Tatom


- - -
## 1.3.4 - 2021-12-23


### Continuous Integration

b340c3 - make publish.sh executable - Danny Tatom


- - -
## 1.3.3 - 2021-12-23


### Continuous Integration

599ffe - trying again - Danny Tatom


- - -
## 1.3.2 - 2021-12-23


### Continuous Integration

c27062 - try again to fix publish - Danny Tatom


- - -
## 1.3.1 - 2021-12-23


### Continuous Integration

228a7e - attempt to fix publish workflow - Danny Tatom


- - -
## 1.3.0 - 2021-12-23


### Documentation

58c3fb - remove strikethru from hook feature - Danny Tatom

8752fd - add hook example - Danny Tatom


### Continuous Integration

70ce66 - update publish workflow - Danny Tatom

78db14 - update rusty-hook config - Danny Tatom


### Features

97adbc - add option to run as git hook - Danny Tatom

1fb6ac - allow passing path to a config file - Danny Tatom


### Build system

03cf21 - update deps - Danny Tatom


- - -
## 1.2.0 - 2021-10-28


### Documentation

413fe9 - fix typo in codecov badge - Danny Tatom

0bcc28 - add codecov badge - Danny Tatom

7a91bb - add feature list & more usage examples - Danny Tatom

12dc6f - clean up usage section - Danny Tatom

8d5b34 - update - Danny Tatom


### Refactoring

916a51 - load default commit types from config - Danny Tatom

2e7cd9 - clean up main func - Danny Tatom

db4756 - restructure app a bit - Danny Tatom


### Tests

b7986c - split up tests - Danny Tatom


### Bug Fixes

31ef10 - make error messages consistent - Danny Tatom


### Features

c3f3c6 - add support for commit types with no emoji - Danny Tatom


### Continuous Integration

b386dc - add codecov - Danny Tatom

7e86e9 - add rust-toolchain - Danny Tatom

8f7a8f - add audit workflow - Danny Tatom


- - -
## 1.1.2 - 2021-10-21


### Continuous Integration

87bef9 - maybe fix build - Danny Tatom


- - -
## 1.1.1 - 2021-10-21


### Miscellaneous Chores

15e89b - remove cargo-bump - Danny Tatom

76ca7f - set rust edition to 2021 - Danny Tatom


### Build system

286e83 - make release bin smaller - Danny Tatom

f977bb - get derive as a feature from serde - Danny Tatom


### Refactoring

9d3e12 - move answer functions to their own file - Danny Tatom

bc8aae - use const strings for answer keys - Danny Tatom

ce90dd - replace config loading with a single load_config function - Danny Tatom

dc718a - clean up get_amended_body - Danny Tatom

aee814 - clean up render_commit_type_choice - Danny Tatom


- - -
## 1.1.0 - 2021-10-21


### Refactoring

66c68d - remove unnecessary `Error`s from `Result`s - Danny Tatom

f3a875 - little bit of some cleanup - Danny Tatom

a93fc3 - put config file handling into its own file - Danny Tatom


### Features

ef923a - add validation to questions - Danny Tatom


### Documentation

d6cab7 - add better config examples - Danny Tatom

72450e - add deps.rs badge - Danny Tatom

468682 - add version badge - Danny Tatom

afcfe2 - capitalize cli - Danny Tatom

18cfcb - add link to releases page - Danny Tatom


### Miscellaneous Chores

9984fd - add issue templates - Danny Tatom


- - -
## 1.0.0 - 2021-10-20


- - -

This changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto).

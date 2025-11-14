# nanoserve

[![GitHub License](https://img.shields.io/github/license/PRO-2684/nanoserve?logo=opensourceinitiative)](https://github.com/PRO-2684/nanoserve/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/nanoserve/release.yml?logo=githubactions)](https://github.com/PRO-2684/nanoserve/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/nanoserve?logo=githubactions)](https://github.com/PRO-2684/nanoserve/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/nanoserve/total?logo=github)](https://github.com/PRO-2684/nanoserve/releases)
[![Crates.io Version](https://img.shields.io/crates/v/nanoserve?logo=rust)](https://crates.io/crates/nanoserve)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/nanoserve?logo=rust)](https://crates.io/crates/nanoserve)
[![docs.rs](https://img.shields.io/docsrs/nanoserve?logo=rust)](https://docs.rs/nanoserve)

> [!NOTE]
> This is a toy project, primarily used as my handin for the course [Computer Networks](https://jwcg.ucas.ac.cn/public/courseOutlines?courseId=289907).

Nanoserve is a lightweight, educational HTTP/1.1 server implementation built using TCP sockets and modern Rust async I/O. It demonstrates HTTP protocol fundamentals, asynchronous networking, and systems programming best practices. This server supports core HTTP/1.1 features including GET requests, range requests (partial content), graceful shutdown, and efficient file serving.

## ⚙️ Automatic Releases Setup

1. [Create a new GitHub repository](https://github.com/new) with the name `nanoserve` and push this generated project to it.
2. Enable Actions for the repository, and grant "Read and write permissions" to the workflow [here](https://github.com/PRO-2684/nanoserve/settings/actions).
3. [Generate an API token on crates.io](https://crates.io/settings/tokens/new), with the following setup:

    - `Name`: `nanoserve`
    - `Expiration`: `No expiration`
    - `Scopes`: `publish-new`, `publish-update`
    - `Crates`: `nanoserve`

4. [Add a repository secret](https://github.com/PRO-2684/nanoserve/settings/secrets/actions/new) named `CARGO_TOKEN` with the generated token as its value.
5. Consider removing this section and updating this README with your own project information.

[Trusted Publishing](https://crates.io/docs/trusted-publishing) is a recent feature added to crates.io. To utilize it, first make sure you've already successfully published the crate to crates.io. Then, follow these steps:

1. [Add a new trusted publisher](https://crates.io/crates/nanoserve/settings/new-trusted-publisher) to your crate.
    - Set "Workflow filename" to `release.yml`.
    - Keep other fields intact.
    - Click "Add".
2. Modify [`release.yml`](.github/workflows/release.yml).
    1. Comment out or remove the `publish-release` job.
    2. Un-comment the `trusted-publishing` job.
3. Remove the `CARGO_TOKEN` [repository secret](https://github.com/PRO-2684/nanoserve/settings/secrets/actions).

## 📥 Installation

### Using [`binstall`](https://github.com/cargo-bins/cargo-binstall)

```shell
cargo binstall nanoserve
```

### Downloading from Releases

Navigate to the [Releases page](https://github.com/PRO-2684/nanoserve/releases) and download respective binary for your platform. Make sure to give it execute permissions.

### Compiling from Source

```shell
cargo install nanoserve
```

## 💡 Examples

TODO

## 📖 Usage

TODO

## ✅ TODO

- [ ] Accept `HEAD` and `OPTIONS`, returning file metadata
- [ ] `Content-Length` header

## 🎉 Credits

TODO

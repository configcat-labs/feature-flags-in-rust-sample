# ConfigCat Feature Flags in Rust Sample

[Read the blog post here](https://configcat.com/blog/how-to-use-configcat-feature-flags-in-rust)

A mock cargo delivery application written in Rust. The cargo delivery truck has two modes: Standard and Turbo. With the help of a [ConfigCat feature flag](https://configcat.com/featureflags), we can toggle between them. When the flag is **ON**, Turbo mode is activated, and the truck goes from **🏁 0-60 mph in 3.5 seconds** in the terminal. When the flag is **OFF**, Standard mode is used instead, and the truck goes from **🏁 0-60 mph in 10.0 seconds**.

## Build & Run

Instructions on how to build and run the application locally.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed. Installing Rust with `rustup` also installs Cargo, which we'll use to create the project, manage dependencies, and run the application.
- A ConfigCat account. You can [sign up for the Forever Free plan](https://app.configcat.com/auth/signup).
- [Visual Studio Code](https://code.visualstudio.com/) installed as the code editor. You can also install the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) for Rust language support, including code completion, diagnostics, and navigation.
- [Git](https://git-scm.com/install/) installed if you want to clone the sample project.

### Building and running the app

1. Add your [ConfigCat SDK key](https://app.configcat.com/sdkkey) in the `main.rs` file. You can also update the feature flag key if yours is different.

2. In your terminal, navigate to the cloned repository and run the following command to build the application:

   ```bash
   cargo build
   ```

3. Then, run the application with:

  ```bash
  cargo run
  ```

## Learn more

Useful links to technical resources.

- [Rust Documentation](https://doc.rust-lang.org/stable/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/index.html)
- [The Rust Programming Language book](https://doc.rust-lang.org/stable/book/index.html)

[**ConfigCat**](https://configcat.com) supports many other frameworks and languages. Check out the full list of supported SDKs [here](https://configcat.com/docs/sdk-reference/overview/).

You can also explore other code samples for various languages, frameworks, and topics in [ConfigCat labs](https://github.com/configcat-labs) on GitHub.

Keep up with ConfigCat on [X](https://x.com/configcat), [Facebook](https://www.facebook.com/configcat), [LinkedIn](https://www.linkedin.com/company/configcat/), [GitHub](https://github.com/configcat), and the [News & Product Updates](https://configcat.com/docs/news/) page.

## Author

[Chavez Harris](https://github.com/codedbychavez)

## Contributions

Contributions are welcome!
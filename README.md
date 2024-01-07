# Madara CLI

A command-line tool for rapid deployment of Madara app chains.

## Dependencies

1. [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
2. [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
3. [Docker](https://docs.docker.com/desktop/)

## Quick Start

1. Clone the `madara-cli` repo:

   ```shell
      git clone https://github.com/karnotxyz/madara-cli
   ```

2. Navigate to the madara-cli directory and build the tool:

    ```shell
      cd madara-cli
      cargo build --release 
    ```
   
3. Initialize a new app chain. Ensure to fund the logged account, especially if you've chosen Avail as your DA Layer.

   ```shell
      ./target/release/madara-cli init  
   ```

4. Run your app chain:

   ```shell
      ./target/release/madara-cli run
   ```

5. Optionally, explore the StarkScan explorer. Accessible at http://localhost:4000.

   ```shell
      ./target/release/madara-cli explorer
   ```
   
Congratulations! You now have a custom madara app running.

# Madara CLI

A command-line tool for rapid deployment of Madara app chains.

## Dependencies

There are a few dependencies that need to be installed to smoothly `madara-cli`

[Installing dependencies] (./docs/setup.md)

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
      ./target/release/madara init  
   ```

4. Run your app chain:

   ```shell
      ./target/release/madara run
   ```

5. Optionally, explore the StarkScan explorer. Accessible at http://localhost:4000.

   ```shell
      ./target/release/madara explorer
   ```
   
Congratulations! You now have a custom madara app running.

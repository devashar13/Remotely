# Remotely
A tool for managing remote machines with ssh




## Install:
  * Install Rust
    ```
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   * Clone the repo 
     ```
      git clone https://github.com/devashar13/pulao-cli.git
     ```
   * Run this
    ```
    cargo install --path . 
    ```

## Usage:
  * Add ssh
    ```
      pulao-cli add <ip> <name> <user>
     ```
   * List 
     ```
      pulao-cli list
     ```
   * Run
    ```
    pulao-cli run <name>
    ```


    
## Issues
  There would be like 1000 issues , issok chill

## Future
   Thought of some features
   - [ ] Download files from remote server
   - [ ] Upload files to remote server 

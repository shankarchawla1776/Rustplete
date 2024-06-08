# rusty-notes

A project to function as a notetaking app for your terminal using rustyline and chrono. 

### Usage 

1. Clone the repository to your local machine.
```zsh
git clone https://github.com/shankarchawla1776/rusty-notes.git
```

2. Navigate to the project and run it with cargo. 
```sh
cd rusty-notes
cargo build --release 
cargo run
```

3. Optionally, you can add a quick-run function to your ```~/.zshrc``` or ```~/.bashrc``` file and source it. 
```zsh
function note() {
    cd ~/rusty-notes
    cargo run
}
```

### Example Usage
<img width="869" alt="Screenshot 2024-03-26 at 11 13 36 AM" src="https://github.com/shankarchawla1776/rusty-notes/assets/139474458/5c97eef6-d53a-4a1b-8a26-9f9cfa01494b">

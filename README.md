# RustyC2

This is a repo where we try to learn how to write a C2 Framework using Rust. 

live-streamed on [Twitch](https://twitch.tv/maikroservice)

notes: [Notion](https://maikroservice.notion.site/RustyC2-Write-y-our-own-C2-Framework-in-Rust-64a312444029476baeccd786bdd375fd)
## Todos:


### Client (Piglets/Bacons) - Rust
* send HTTP Request with Rust to a netcat Listener
* make that HTTPS
* listen for "Kommandos" and execute "Kommando" once received
* play back the result of the "Kommando" to the Barn
* Have a TaskList that is basically a Queue
* Baconify Function to exit the Piglets

### TeamServer (Barn) - probably in Python/JavaScript
* listen for "Hello" Messages and print the Messages
* show available commands
* show connected Bacons
* show time since last connect / alive signal from Bacons


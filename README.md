# RustyC2

This is a repo where we try to learn how to write a C2 Framework using Rust.

live-streamed on [Twitch](https://twitch.tv/maikroservice)

notes: [Notion](https://maikroservice.notion.site/RustyC2-Write-y-our-own-C2-Framework-in-Rust-64a312444029476baeccd786bdd375fd)

## Todos:

### Client (Piglets/Bacons) - Rust

- send HTTP Request with Rust to a netcat Listener
- make that HTTPS
- listen for "Kommandos" and execute "Kommando" once received
- play back the result of the "Kommando" to the Barn
- Have a TaskList that is basically a Queue
- Baconify Function to exit the Piglets

### TeamServer (Barn) - probably in Python/JavaScript

- listen for "Hello" Messages and print the Messages
- show available commands
- show connected Bacons
- show time since last connect / alive signal from Bacons

## sources

- [@\_RastaMouse](https://twitter.com/_rastamouse) made an incredible [course](https://courses.zeropointsecurity.co.uk/courses/c2-development-in-csharp) on how to design a C2 Framework
- [Sylvain](https://twitter.com/SylvainKerkour) wrote a [book](https://academy.kerkour.com/black-hat-rust) about Black-Hat Rust which gave me a primer to rusty concepts
- [Mr D0nutptr](https://twitter.com/d0nutptr) for inspiration to use Rust as a language and what to do with it
- my team at work for using Rust and "forcing" me to learn it to be a better team mate
- [Link: rust-based C2](https://github.com/postrequest/link) - as inspiration
- A [blog](https://harrison-technology.net/learning-rust-by-creating-a-basic-c2/) post by [Michael Harrison](https://harrison-technology.net/)
- Shogun lab's [posts](https://shogunlab.gitbook.io/building-c2-implants-in-cpp-a-primer/) about C++ C2 Development
- [ippsec's](https://ippsec.rocks) [youtube video](https://www.youtube.com/watch?v=FiT7-zxQGbo) on C2 implant development and the [video](https://www.youtube.com/watch?v=6l4ZIKwzW8U) on building empire modules
- [WhiteHat Hacking's](https://www.youtube.com/channel/UC2JICTAVKyfXI4g9zITJ7tw) [videos](https://www.youtube.com/playlist?list=PLwVUVslZT0K1qX67U_bFsf6puBrnEWhMa) on Rust Malware Development

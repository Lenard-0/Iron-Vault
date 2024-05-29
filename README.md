# Iron-Vault
A redis style in-memory unified storage and (in future) db written in Rust.

Early stages of dev.

However, basic set and get functions in memory works, with time to live and auto cleanup.

Also, has functionality for collections.

Use this internally in with your Rust project to handle shared memory natively,
or use it to communicate with external programs.

Uses TCP for external communication.
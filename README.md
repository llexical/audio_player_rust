# Audio Player - Rust
This is my own pet project, I am not going to support anyone else using this. I have no idea what I am doing anyway xD.

Trying out making a streaming audio player using rust for the majority of it. UI will not be Rust because the options hurt my soul.

Why am I making it? Because having your own music that you paid for streamed in good quality is no longer a thing you can really do [I mean the hard stuff that comes in files]. With copywriting laws and piracy understandably no paid for business wants to get involved. SOO along comes stubborness and curiosity. Will this likely take me so long the technology will be obsolete once I am done? Absolutely. Am I going to try anyway? Of course. I am also making it just because its fun (reminder to self this should be fun, if not stop).


## Architecture Plan

This audio player will have 4 major parts a data server, a HLS streaming server, a frontend lib and some kind of UI for different platforms.

### The Data Server
This will contain what albums, songs etc there are as well as user accounts. Has as is suggested in the name, a database. Built 100 of these so should be pretty comfy. Always enjoyable to make something you know well in another language.

### The HLS Streaming Server
This is the really fun bit, handles the upload of media files, transforming them into the right format then provides an API to get a playlist(M3U8 file) which the frontend lib can use to stream. I've never made one of these before, didn't even know what one was so looking forward to this.

### Frontend Lib
This library should be able to be used by any frontend UI framework,haven't worked that bit out yet, but c-libs work well and discord uses a rust backend with Electron so we can figure it. The idea is that this app can work on Windows, Mac and Android (Maybe Ipad but lowest priority).

### UI
What we will use for UI is yet undecided outside of: Not Rust. Quite likely electron will be a contendor for the first version because I have a vauge familiarity with it and know React well. Later on may choose to make platform specific builds because performance but also curiosity. Never had a good reason to put in the effort before.

## Getting Started
### Required:
- Rust.

NOTE: Likely will end up dockerised in the end because... easier. But for now just rust because my partner tells me dockerising everything is my form of procrastination xD.

### HLS Media Server
- `cd hls_media_server`
- `cargo run`

Right now runs a server with a default endpoint you can post a media file to to upload into a /tmp folder.

### Sandbox
- `cd sandbox`
- `cargo run`

Plays a music file it downloads from a link that changes all the time. So likely won't work. Put in any link to an MP3 file. Anyway unlikely to use this for any final stuff it was just a test to see whats already out there.
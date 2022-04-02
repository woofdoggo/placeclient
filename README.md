# placeclient
Horrendous abomination of an automated /r/place scraper

# Why
I got bored after getting myself IP(?) banned from placing pixels and having to
make a new account every 15 minutes. I intended to make a standalone viewer but
gave up because it's harder to figure out how to read image data than it is to
send pixel update requests automatically (thanks for the bot prevention Reddit)

# Data
I've been running this myself since ~2:20 PM EST, April 2. I plan to convert
the image diffs into a more compact format after /r/place is over and publish
the data for free use. Check back on the 4th or the 5th.

# Running
Good luck this thing is bad

- Change the hardcoded paths in `download/src/main.rs` and `proxy/proxy.py`
- Get Firefox, Rust/Cargo, and Python 3
- `cd proxy` and generate a key+cert (`key.pem` and `cert.pem`) with OpenSSL
- Add the extension in `firefox/` through the `about:debugging` page
- `cd download && cargo run --release`
- Open a new terminal and `cd proxy && python proxy.py`
- Go to the /r/place canvas and zoom out

Reload the page every now and then because the website sucks. Also, make sure
you don't run out of storage space - depending on what images your client
requests, storage usage can increase by as much as ~26mb/min. That number might
climb further if/when the canvas gets expanded again.

### Output
The `download` program will save whatever /r/place images your browser tries
to retrieve. All of the images from any given minute are placed into a single
gzipped tarball.

There are two types of images - full and diff. The names and the contents of
them should be fairly self explanatory; you can sort them from one another
pretty easily.

# Todo
- add automatic refresh to firefox extension
- extract scraper logic to make it standalone? probably too annoying

# License
placeclient is licensed under the BSD 2-clause license.

```
Copyright 2022 dog

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
this list of conditions and the following disclaimer in the documentation
and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

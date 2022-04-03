# placeclient
Horrendous abomination of an automated /r/place scraper.

# Why
I got bored after getting myself shadowbanned from placing pixels and having to
make a new account every 15 minutes.

I intended to make a standalone viewer but gave up because reading pixel data
seemed harder and I didn't have a use for it without being able to participate 
anyways. So, I made this instead (yes it reads image data; no, it didn't at
first - it used a browser extension and a proxy and it was really bad don't look at
the git history)

The code in this repository is a complete disaster. I don't plan on improving it
~~until~~ unless it stops working again.

# Data
I've been running this myself since ~2:20 PM EST, April 2. There are some gaps
in the data, particularly in the first 5 hours (due to my bad code.) I plan to
convert the image diffs into a more compact format after /r/place is over and
publish the data for free use. Check back on the 4th or the 5th.

# Running
Good luck. Most likely, I will not provide you with any help. You're on your
own if something doesn't work.

Make sure you have at least a few gigabytes of disk space if you plan to run this
for a while. If you have a bad internet connection this will probably not run
well (if at all); it uses 50kb/s at virtually all times.

```sh
# clone the repository
git clone https://github.com/woofdoggo/placeclient.git
cd placeclient

# setup reddit auth
# go to https://reddit.com/prefs/apps and create a script app
cd ws-scrape
echo YOURUSERNAME > username.auth
echo YOURPASSWORD > password.auth
echo YOURCLIENTID > client.auth
echo YOURSECRET > secret.auth

# you will also have to adjust hardcoded paths in the download source file:
# change /mnt/hdd/place.log to YOURLOGPATH.log
# change /mnt/hdd/place2 to YOURIMAGEPATH
touch YOURLOGPATH.log
mkdir YOURIMAGEPATH

# build and run
cd ws-scrape && cargo build --release
cd ../download && cargo run --release

# open another terminal for the last command
# you need to run download first, then ws-scrape
# but both need to be running for it to do anything
cd placeclient/ws-scrape && cargo run --release
```

### Output
placeclient will store gzipped tarballs of /r/place images, with 1024 images in
each tarball. These tarballs will contain a mixture of full and diff images,
whose names should be fairly self explanatory.

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

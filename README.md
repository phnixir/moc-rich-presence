# Discord rich presence for Music on Console
<a href="https://github.com/phnixir/moc-rich-presence" >
<img src="https://i.ibb.co/qkNd5z0/moclogo.png" alt="Music on Console" width="128" height="128">
</a>

This is a small rust program using the moc-rs crate to add discord rich presence
functionality to the MoC server, I use MoC a LOT and I love it, I also enjoy
using discord and I couldn't find anything that would add rich presence
functionality to MoC, so I wrote it myself!

## Dependencies
⚠️ This utility assumes that you have the program `mocp` in your path, since MoC
is a linux only program you can find an up to date version in most distribution's
repositories in a package called `moc`

For arch you would do:
```
sudo pacman -S moc
```

And for debian or ubuntu:
```
sudo apt install moc
```

Which would make the `mocp` program available in path, if you can't install it
have a look over at the official [moc download page](https://moc.daper.net/download).

## Features
- Differentiating between playing local files and playing internet radio stations
- Button to join in listening along for radio stations
- Different states, Playing, Streaming, Paused and Stopped, all with their own small
image, except the Streaming state which uses the small image for Playing.
- Shows full title as details, if not available it uses the filename instead
- Elapsed time for when you're streaming a radio station
- Time left until complete for local files

## Help needed!
Right now the source code seems very unclean, I barely know how to use lifetimes in
rust, I'd be happy if you could have a look at the source code!

The main logo right now is alright however if you have any other design you would
like to share don't hestitate to open an issue!

## Contributing
Thanks for your interest in contributing! please open an issue or merge request
to contibute. Code contributions submitted for inclusion in the work by you, as
defined in the MPL2.0 license, shall be licensed as the above without any
additional terms or conditions.


## License
This project is licenced under [MPL 2.0][license].

[license]: https://www.mozilla.org/en-US/MPL/2.0/

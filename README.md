# xfce4-dynamic-workspaces

This is a rewrite of https://github.com/DimseBoms/XFCE-Dynamic-Workspace but rewritten in the lord's language (rust) and uses the libraries that the python version uses under the hood through an FFI layer.

The goal is to reduce the amount of memory that this program uses, because this is supposed to be running on *every* Unicorn instance, so any wasted RAM is permanently wasted as long as it is running.

So far, I have been able to reduce the idling memory by 50%, from around 40Mb (python) to 20Mb (rust).

When developing or making PRs, please use any and all optimizations that follow these principles:

* Try not to introduce new crates.
* If you have to choose between readability and absolute performance, choose readability.
* Try not to heap allocate if possible.

## Other

This project aims to be as tiny as possible, with the exception that it must implement all features from the python version. One such hurdle is having notification support, which (relatively) balloons the binary size, so for users (or distributions) that don't want notification support or the dependency chain, you can disable the `notify` feature.

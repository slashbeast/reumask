reumask
=======

About
-----
This is a tiny tool written in Rust to bump filesystem permissions as if all the files and directories were created under given umask.

In order to preserve executable permissions the tool will look up current permissions of file, and if it has executable bit set on owner, it will then enable it for owner, group and others and after that apply the umask on the top of it.

Note that as for this very moment the tool will *not* preserve sticky, suid and sgid bits on files and directories, be mindful of it as running it as root on `/` mount point will most likely lead to a broken system.

Usecase
--------
Quite often I found myself dealing with random `permission denied` errors cause by my default umask of `077`, which makes files only accessible by the user that created them. This is great to have as default however can backfire when, for example, one built kernel under `/usr/src/` as root with umask of `077` and then ask Portage to rebuild out-of-tree kernel modules, then it will violently fail with permission denied. While it is easy to grant the directories permissions as if those were created under umask of `022` by running `find /path -type d -exec chmod -c 755 '{}' +`, this is however bigger of an issue with files, because we either make all of them executable with `755` mode or we need a tool or script that would first look up the current file permisisons and then bump them with potential executable bit. The reumask is exactly that tool.

Usage
-----
::

    reumask <umask> <path_to_file_or_directory>
    reumask 022 /usr/src/
    reumask 077 ~/

Building
--------
If you need instructions how to build it, the best would be to just stick to GNU Make::

    make release

Or if you'd like to get a statically linked binary run instead::

    make release-static

Your binary will be then in `target/release/reumask`. Keep in mind that glibc does not really support statically linking binaries and the resulting binary might not work unless a compatible glibc runtime is present, for true static builds link with musl, for example by running the `release-static` target inside Alpine container or virtual machine.

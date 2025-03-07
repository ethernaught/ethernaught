How To
=====

Structure
-----
```
my-app.deb
├── DEBIAN/
│   └── control
├── usr/
│   └── local/
│       └── bin/
│           └── my-app
├── etc/
│   └── my-app/
│       └── config.toml
└── usr/
    └── share/
        ├── applications/
        │   └── my-app.desktop   <-- The .desktop file
        ├── icons/
        │   └── hicolor/
        │       ├── 16x16/
        │       │   └── apps/
        │       │       └── my-app.png    <-- 16x16 icon
        │       ├── 32x32/
        │       │   └── apps/
        │       │       └── my-app.png    <-- 32x32 icon
        │       ├── 48x48/
        │       │   └── apps/
        │       │       └── my-app.png    <-- 48x48 icon
        │       └── 64x64/
        │           └── apps/
        │               └── my-app.png    <-- 64x64 icon
        └── var/
            └── lib/
                └── my-app/
                    └── database.sql   <-- Offline Query DB
```

DEBIAN/control
-----
```
Package: ethernaut
Version: 0.1.0
Architecture: amd64
Maintainer: DrBrad <brad@bradeagle.com>
Installed-Size: 2048
Description: Packet sniffer
```

Build
-----

```
dpkg-deb --build deb/
```


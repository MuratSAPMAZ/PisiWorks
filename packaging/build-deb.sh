#!/bin/sh
# Builds a .deb package of PisiWorks for Pardus and other Debian-based systems.
#
#   packaging/build-deb.sh
#
# The package lands in target/deb/. It needs cargo, dpkg-deb, strip and objdump.
# The maintainer written into the package is taken from DEB_MAINTAINER, or
# failing that from git's user.name and user.email:
#
#   DEB_MAINTAINER="Name <mail@example.org>" packaging/build-deb.sh
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(dirname -- "$here")
cd "$root"

version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
arch=$(dpkg --print-architecture)
maintainer=${DEB_MAINTAINER:-"$(git config user.name || true) <$(git config user.email || true)>"}
if [ "$maintainer" = " <>" ]; then
    echo "Set DEB_MAINTAINER=\"Name <mail>\" or git's user.name and user.email." >&2
    exit 1
fi

cargo build --release

stage="target/deb/pisiworks_${version}_${arch}"
rm -rf "$stage"
install -Dm755 target/release/pisiworks "$stage/usr/bin/pisiworks"
strip --strip-unneeded "$stage/usr/bin/pisiworks"
install -Dm644 packaging/pisiworks.desktop "$stage/usr/share/applications/pisiworks.desktop"
install -Dm644 packaging/pisiworks.xml "$stage/usr/share/mime/packages/pisiworks.xml"
install -Dm644 packaging/pisiworks.svg "$stage/usr/share/icons/hicolor/scalable/apps/pisiworks.svg"
for size in 16 32 48 64 128 256; do
    install -Dm644 "packaging/icons/pisiworks-$size.png" "$stage/usr/share/icons/hicolor/${size}x${size}/apps/pisiworks.png"
done
install -Dm644 docs/guide.tr.md "$stage/usr/share/doc/pisiworks/kilavuz.md"
install -Dm644 docs/guide.md "$stage/usr/share/doc/pisiworks/guide.md"
install -Dm644 examples/solstice-landing.pisi "$stage/usr/share/doc/pisiworks/examples/solstice-landing.pisi"

# The license, in the form Debian keeps it in.
{
    echo "Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/"
    echo "Upstream-Name: PisiWorks"
    echo "Source: https://github.com/MuratSAPMAZ/PisiWorks"
    echo "Comment: PisiWorks is a fork of Emberframe, https://github.com/gregstein/emberframe"
    echo
    echo "Files: *"
    echo "Copyright: 2026 Med Amine Belbachir (https://medamineweb.com)"
    echo "           2026 Murat SAPMAZ (https://github.com/MuratSAPMAZ/PisiWorks)"
    echo "License: Expat"
    sed -e '1,/^$/d' -e '/^Copyright/,/^$/d' -e 's/^$/./' -e 's/^/ /' LICENSE
} > "$stage/usr/share/doc/pisiworks/copyright"
chmod 644 "$stage/usr/share/doc/pisiworks/copyright"

# The oldest C library the program runs with is the newest one it asks for a symbol from.
glibc=$(objdump -T "$stage/usr/bin/pisiworks" | grep -o 'GLIBC_[0-9.]*' | sed 's/GLIBC_//' | sort -V | tail -n 1)
installed=$(du -sk "$stage/usr" | cut -f1)
mkdir -p "$stage/DEBIAN"
cat > "$stage/DEBIAN/control" <<CONTROL
Package: pisiworks
Version: $version
Section: graphics
Priority: optional
Architecture: $arch
Maintainer: $maintainer
Installed-Size: $installed
Depends: libc6 (>= $glibc), libgcc-s1, libxcb1, libxkbcommon0, libxkbcommon-x11-0, libvulkan1
Recommends: mesa-vulkan-drivers, libwayland-client0, fonts-noto-core | fonts-dejavu-core, cups-client, zenity, enchant-2 | hunspell, hunspell-tr
Homepage: https://github.com/MuratSAPMAZ/PisiWorks
Description: Fireworks-like screen design editor with a Turkish interface
 PisiWorks draws vector shapes and edits bitmaps in one docked workspace,
 with pages, layers, states, symbols, live filters, masks and export for
 the web, in the manner of Adobe Fireworks. Its menus and commands are in
 Turkish or English.
 .
 It is a fork of Emberframe and contains no Adobe code or artwork.
CONTROL

( cd "$stage" && find usr -type f -exec md5sum {} + > DEBIAN/md5sums )
dpkg-deb --root-owner-group --build "$stage" "target/deb/pisiworks_${version}_${arch}.deb"
echo "target/deb/pisiworks_${version}_${arch}.deb"

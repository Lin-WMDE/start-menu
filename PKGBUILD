# Maintainer: WMDE <https://wmde.fun>
# Contributor: Kamil Lihan <k.lihan@outlook.com> (original cosmic-ext-classic-menu)
#
# Builds our fork Lin-WMDE/start-menu (default branch `master`).
pkgname=wmde-start-menu
pkgver=0.0.14
pkgrel=1
pkgdesc="WMDE Start Menu - a classic Windows-style launcher applet for the COSMIC desktop (fork of cosmic-ext-classic-menu)"
arch=('x86_64')
url="https://wmde.fun"
license=('GPL-3.0-only')
# depends: readelf NEEDED gives glibc/gcc-libs/libxkbcommon; wayland, mesa,
# fontconfig and freetype2 are dlopen'd by libcosmic at runtime. Verify with namcap.
depends=('glibc' 'gcc-libs' 'libxkbcommon' 'wayland' 'mesa' 'fontconfig' 'freetype2')
# makedepends: same toolchain/libs that build the fork in Docker (Dockerfile.build).
makedepends=('rust' 'cargo' 'just' 'git' 'clang' 'lld' 'pkgconf' 'mesa' 'wayland'
             'libxkbcommon' 'fontconfig' 'freetype2' 'expat')
optdepends=('cosmic-panel: run as a COSMIC panel applet')
source=("$pkgname::git+https://github.com/Lin-WMDE/start-menu.git#branch=master")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  git describe --long --tags --abbrev=7 2>/dev/null | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g' ||
    printf '0.0.14.r%s.g%s' "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
}

build() {
  cd "$srcdir/$pkgname"
  # x86-64-v3 (AVX2/BMI2) baseline for the WMDE repo; runs on Haswell+ (and the VM).
  export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C target-cpu=x86-64-v3"
  just build-release
}

package() {
  cd "$srcdir/$pkgname"
  # installs wmde-start-menu-{applet,settings} to /usr/bin and the
  # fun.wmde.start-menu .desktop/metainfo/icon/applet-buttons to /usr/share
  just rootdir="$pkgdir" prefix=/usr install
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

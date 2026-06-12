# note: I can't test arm64 support.

pkgname=biomejs-bin
pkgver=2.5.0
pkgrel=1
pkgdesc="A toolchain for the web: formatter, linter and more"
arch=('x86_64' 'aarch64')
url="https://github.com/biomejs/biome"
license=('MIT OR Apache-2.0')
depends=()
provides=('biome')
# too lazy to make a PKGBUILD to install from source or git lol
# conflicts=('biomejs')

source_x86_64=("biome::https://github.com/biomejs/biome/releases/download/@biomejs/biome@$pkgver/biome-linux-x64")
source_aarch64=("biome::https://github.com/biomejs/biome/releases/download/@biomejs/biome@$pkgver/biome-linux-arm64")
sha256sums_x86_64=('e7df298f0551dd90bea4425779369aa3130d9817f4acc4f663ef63c327206a19')
sha256sums_aarch64=('27c9bc5994dfb5711f5f09a4c3c35749ca9c4a898a063bb062e6b932dbc2571d')

# they publish BiomeJS to the NPM registry, but I wanted to make a PKGBUILD because why not (and of course, use it myself).
package() {
    mkdir -p "$srcdir"
    local bin_name="biome"
    local bin_path="$srcdir/$bin_name"
    local dest_folder="$pkgdir/usr/bin"
    local bin_dest="$dest_folder/biome"
    mkdir -p $dest_folder
    # copy + make executable
    install -m 755 $bin_path "$bin_dest"
}
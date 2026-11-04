# note: I can't test arm64 support.

pkgname=biomejs-bin
pkgver=2.6.5
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
sha256sums_x86_64=('0653fd798e50565efac8b1958c9a2786fada55cc6c362b7da4f16efff8dd30d8')
sha256sums_aarch64=('5d57334b79e00d312e0363a71157ad00d3a66f2a6b35ca707b065e3b32099235')

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
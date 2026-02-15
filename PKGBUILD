# note: I can't test arm64 support.

pkgname=biomejs-bin
pkgver=2.4.0
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
sha256sums_x86_64=('1fdf25c2dec5799b258de6eefff25f1533dc26e56ed8d0e08503272c63644af5')
sha256sums_aarch64=('4743f6b00b7a6680054167b8da76f211abe814ea1568f429aedfe4cc163784ad')

# they publish BiomeJS to the NPM registry, but I wanted to make a PKGBUILD because why not (and of course, use it myself).
package() {
    local bin_name="biome"
    local bin_path="$srcdir/$bin_name"
    local dest_folder="$pkgdir/usr/bin"
    local bin_dest="$dest_folder/biome"
    mkdir -p $dest_folder
    # copy + make executable
    install -m 755 $bin_path "$bin_dest"
}
# note: I can't test arm64 support.
# makepkg --printsrcinfo > .SRCINFO

pkgname=biomejs-bin
pkgver=2.3.11
pkgrel=1
pkgdesc="A toolchain for the web: formatter, linter and more"
arch=('x86_64' 'aarch64')
url="https://github.com/biomejs/biome"
license=('MIT OR Apache-2.0')
depends=()
provides=('biome')
# biome JS source package coming NEVER...
# conflicts=('biomejs')

source_x86_64=("biome::https://github.com/biomejs/biome/releases/download/@biomejs/biome@$pkgver/biome-linux-x64")
source_aarch64=("biome::https://github.com/biomejs/biome/releases/download/@biomejs/biome@$pkgver/biome-linux-arm64")
sha256sums_x86_64=('92587fac102e33cbf8ab1fdb4884f8f4dcbf1117259fc6decd5cb5b5f5e48e67')
sha256sums_aarch64=('036ce4b0adabac048c1e5f28539d04fdff0b0f81471657244f9cfcf7c8525578')

# they publish BiomeJS to the NPM registry, but I wanted to make a PKGBUILD because why not (and of course, use it myself).
package() {
    local bin_name="biome"
    local bin_path="$srcdir/$bin_name"
    local dest_folder="$pkgdir/usr/bin/"
    local bin_dest="$dest_folder/biome"
    mkdir -p $dest_folder
    cp "$bin_path" "$bin_dest"
    chmod +x $bin_dest
}
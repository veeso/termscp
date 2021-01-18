# Maintainer: Christian Visintin
pkgname=termscp
pkgver=0.3.1
pkgrel=1
pkgdesc="TermSCP is a SCP/SFTP/FTPS client for command line with an integrated UI to explore the remote file system. Basically WinSCP on a terminal."
url="https://github.com/veeso/termscp"
license=("GPL-3.0")
arch=("x86_64")
provides=("termscp")
options=("strip")
source=("https://github.com/veeso/termscp/releases/download/v$pkgver/termscp-$pkgver-x86_64.tar.gz")
sha256sums=("dd056531554737595cbe5ac9ff741fdabf5e386299bbc0c81ea9e0f00fbbe2d0")

package() {
    install -Dm755 termscp -t "$pkgdir/usr/bin/"
}

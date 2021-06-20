# Maintainer: Christian Visintin
pkgname=termscp
pkgver=0.5.1
pkgrel=1
pkgdesc="termscp is a SCP/SFTP/FTPS client for command line with an integrated UI to explore the remote file system. Basically WinSCP on a terminal."
url="https://github.com/veeso/termscp"
license=("MIT")
arch=("x86_64")
provides=("termscp")
options=("strip")
source=("https://github.com/veeso/termscp/releases/download/v$pkgver/termscp-$pkgver-x86_64.tar.gz")
sha256sums=("f66a1d1602dc8ea336ba4a42bfbe818edc9c20722e1761b471b76109c272094c")

package() {
    install -Dm755 termscp -t "$pkgdir/usr/bin/"
}

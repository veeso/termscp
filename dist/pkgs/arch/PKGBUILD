# Maintainer: Christian Visintin
pkgname=termscp
pkgver=0.4.0
pkgrel=1
pkgdesc="TermSCP is a SCP/SFTP/FTPS client for command line with an integrated UI to explore the remote file system. Basically WinSCP on a terminal."
url="https://github.com/veeso/termscp"
license=("MIT")
arch=("x86_64")
provides=("termscp")
options=("strip")
source=("https://github.com/veeso/termscp/releases/download/v$pkgver/termscp-$pkgver-x86_64.tar.gz")
sha256sums=("7a8c70add8306a2cb3f2ee1d075a00fef143fc9aad4199797c7462bab1649296")

package() {
    install -Dm755 termscp -t "$pkgdir/usr/bin/"
}

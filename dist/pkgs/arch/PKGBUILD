# Maintainer: Christian Visintin
pkgname=termscp
pkgver=0.3.0
pkgrel=1
pkgdesc="TermSCP is a SCP/SFTP/FTPS client for command line with an integrated UI to explore the remote file system. Basically WinSCP on a terminal."
url="https://github.com/veeso/termscp"
license=("GPL-3.0")
arch=("x86_64")
provides=("termscp")
options=("strip")
source=("https://github.com/veeso/termscp/releases/download/v$pkgver/termscp-$pkgver-x86_64.tar.gz")
sha256sums=("c9e777c48e30ff1ebf84dbe10f5471b2da753e324753bda2cb08109beab0637d")

package() {
    install -Dm755 termscp -t "$pkgdir/usr/bin/"
}

# Maintainer: Christian Visintin
pkgname=termscp
pkgver=0.3.2
pkgrel=1
pkgdesc="TermSCP is a SCP/SFTP/FTPS client for command line with an integrated UI to explore the remote file system. Basically WinSCP on a terminal."
url="https://github.com/veeso/termscp"
license=("GPL-3.0")
arch=("x86_64")
provides=("termscp")
options=("strip")
source=("https://github.com/veeso/termscp/releases/download/v$pkgver/termscp-$pkgver-x86_64.tar.gz")
sha256sums=("e2700e2e9b741eb273e2633d5cf24ad620365d059bdd4f2b42f3737a7c28a2c7")

package() {
    install -Dm755 termscp -t "$pkgdir/usr/bin/"
}

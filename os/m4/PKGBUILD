# Maintainer: Lukas Fleischer <lfleischer@archlinux.org>
# Contributor: Allan McRae <allan@archlinux.org>
# Contributor: Andreas Radke <andyrtr@archlinux.org>

pkgname=m4
pkgver=1.4.20
pkgrel=1
pkgdesc="The GNU macro processor"
arch=('x86_64')
url="https://www.gnu.org/software/m4"
license=('GPL3')
depends=('glibc' 'bash')
source=("https://ftp.gnu.org/gnu/m4/$pkgname-$pkgver.tar.xz"{,.sig})
sha256sums=('e236ea3a1ccf5f6c270b1c4bb60726f371fa49459a8eaaebc90b216b328daf2b'
            'SKIP')
validpgpkeys=('71C2CC22B1C4602927D2F3AAA7A16B4A2527436A')   # Eric Blake

build() {
	cd "$pkgname-$pkgver"
	./configure --prefix=/usr
	make
}

check() {
	cd "$pkgname-$pkgver"
	make check
}

package() {
	cd "$pkgname-$pkgver"
	make prefix="${pkgdir}"/usr install
}

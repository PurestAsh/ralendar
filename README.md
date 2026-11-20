

Archlinux 打包
CARGO_NET_GIT_FETCH_WITH_CLI=true pkgctl build
makechrootpkg -c -r $CHROOT

依赖
noto fonts
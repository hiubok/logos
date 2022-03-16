{ pkgs }: {
    deps = [
        pkgs.bashInteractive
        pkgs.cargo.out
        pkgs.openssl
        pkgs.openssl.dev
        pkgs.pkgconfig
    ];
}
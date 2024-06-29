# LimoneneOS
30日で出来るOS自作入門のMikanOSをベースにRustで記述したOSです。

uefiは使用せず、基本的にcoreのみを使っています（一部使用しているクレートは、今後置き換える予定です）

# Build
`cargo +nightly build`

# Run (WSL2上での動作を確認しています)
`script/run_qemu.sh`

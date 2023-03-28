## Nginxの設定ファイル

Nginxの設定ファイルは通常`/etc/nginx`の下にある。
その中の`nginx.conf`が最初に読み込まれる設定ファイル。
それ以外の設定ファイルは、`nginx.conf`からincludeして使用する。

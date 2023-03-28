## その他

`ps aux | grep nginx`でちゃんとNginxのmasterプロセスとworkerプロセスが動いていることが確認できた。

```bash
[root@ff9fe778dbfb /]# ps aux | grep nginx
root         136  0.0  0.1 103332  2172 ?        Ss   14:36   0:00 nginx: master process /usr/sbin/ngin
nginx        137  0.0  0.3 122556  7880 ?        S    14:36   0:00 nginx: worker process
nginx        138  0.0  0.3 122556  7880 ?        S    14:36   0:00 nginx: worker process
root         146  0.0  0.0   9184   972 pts/1    S+   14:38   0:00 grep --color=auto nginx
```

PrettyDNS
===

```
$ cargo run -- --port 10053 --debug
```

```
$ dig @127.0.0.1 -p 10053 +noedns google.com
```

## ref

- [RFC1035 「ドメイン名：実装と仕様」 - JPRS](https://jprs.jp/tech/material/rfc/RFC1035-ja.txt)
- [DNSパケットフォーマットと、DNSパケットの作り方](https://www.atmarkit.co.jp/ait/articles/1601/29/news014.html)
- [アドレス検索 - DNSの話](http://park12.wakwak.com/~eslab/pcmemo/dns/dns5.html)

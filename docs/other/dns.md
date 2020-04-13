## 设置永久(Permanent)Dns

```bash
$ ls -l /etc/resolv.conf
lrwxrwxrwx 1 root root 29 Mar 16 19:37 /etc/resolv.conf -> ../run/resolvconf/resolv.conf
```

安装**resolvconf**服务
```bash
# 安装服务
sudo apt install resolvconf

# 配置dns
sudo vi /etc/resolvconf/resolv.conf.d/head
# 内容：
nameserver 61.139.2.69
nameserver 8.8.8.8

# 启动服务
sudo systemctl start resolvconf.service
sudo systemctl enable resolvconf.service
sudo systemctl status resolvconf.service
```


## 参考

- [How to set permanent dns nameservers](https://www.tecmint.com/set-permanent-dns-nameservers-in-ubuntu-debian/)

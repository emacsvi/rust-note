## ubuntu禁用netplan改用ifupdown配置网络

1. 先安装 ifupdown、resolvconf 软件包。

```bash
sudo apt install ifupdown resolvconf
```

2. 修改配置文件配置好网络连接。

```bash
sudo vi /etc/network/interfaces
```

静态 IP 网络配置示例：

```ini
# interfaces(5) file used by ifup(8) and ifdown(8)
auto lo
iface lo inet loopback
auto enp67s0
iface enp67s0 inet static
address 182.140.213.144
netmask 255.255.255.192
gateway 182.140.213.129
auto enp67s0:0
iface enp67s0:0 inet static
address 10.0.20.16
netmask 255.255.255.0
gateway 10.0.20.1
```

3. 关闭 & 重开网络接口，设置 `networking` 服务开机启动和重启使设置生效。

```bash
# sudo ifdown --force eth0 lo && ifup -a 这个会断网，不能这样干
sudo systemctl enable networking
sudo systemctl restart networking
```

4. 停止、取消和禁止 `Netplan` 相关服务运行，并移除 `Netplan` 软件包及其配置文件。

解释下所操作的服务：

- `networkd-dispatcher` 是 Netplan 后端服务（桌面版则是 `NetworkManager`）；
- `networkd-dispatcher` 是 Netplan 守护服务，可通过监听连接状态以执行特定事件操作；
- `systemd-networkd-wait-online` 用于检测 `systemd-networkd` 所管理网络接口的连接状态；
- `systemd-resolved` 新的 DNS 管理服务，其配置文件里有的参数依赖 Netplan 后端服务工作；

```bash
sudo systemctl stop systemd-networkd networkd-dispatcher systemd-networkd-wait-online systemd-resolved
sudo systemctl disable systemd-networkd networkd-dispatcher systemd-networkd-wait-online systemd-resolved
sudo systemctl mask systemd-networkd networkd-dispatcher systemd-networkd-wait-online systemd-resolved
sudo apt purge nplan netplan.io
```

后续如要切换回 `Netplan`，别忘了恢复上述系统服务。将带有` mask` 的命令换成 `unmask` 运行一次，不然即使 `start disable` 了也没法使用服务。

其它可能用到的命令扩展：

- `systemctl list-dependencies systemd-networkd --reverse` 查询指定服务所关联的依赖项；
- `systemctl list-unit-files --type=masked --all` 列出 `/lib/systemd/system` 下禁用服务；

5. 完成以上操作就设置完成了，全文实测在` Ubuntu Server 18.04` 系统下通过。

如果 DNS 解析不了，请检查 `/etc/resolv.conf` 所链接的文件是否是 `/run/resolvconf/resolv.conf`。如果不是，运行下面命令设置。

```bash
sudo ln -sf /run/resolvconf/resolv.conf /etc/resolv.conf
```

## 参考

- [在ubuntu禁用netplan](https://www.hostarr.com/disable-netplan-and-enable-ifupdown-in-ubuntu/)

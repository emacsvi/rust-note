## 修改hostname的两种方式

方法一： **hostnamectl**命令修改：
```bash
sudo hostnamectl set-hostname cd-xx-08
```

方法二：**/etc/hosts**文件修改
```bash
127.0.0.1   localhost
127.0.0.1   cd-xx-08
```

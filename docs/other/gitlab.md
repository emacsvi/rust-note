# gitlab原生安装

```bash
sudo apt-get update
sudo apt-get install -y curl openssh-server ca-certificates


udo apt-get install -y postfix

curl https://packages.gitlab.com/install/repositories/gitlab/gitlab-ee/script.deb.sh | sudo bash

sudo EXTERNAL_URL="ip地址或者域名xxx.xxx.xxx.xxx:8899" apt-get install gitlab-ee
```

接下来就访问8899页面，注册，登录，将自己的`~/.ssh/id_rsa.pub`公钥上传到服务器上面。

### 修改端口与ip
`gitlab`使用`ssh帐户`是`git`,端口也是你本地机器上面的`ssh.port`，我这里是`44022`所以将端口修改一下重启。

**sudo vi /etc/gitlab/gitlab.rb**
```ruby
### GitLab Shell settings for GitLab
gitlab_rails['gitlab_shell_ssh_port'] = 44022
# gitlab_rails['gitlab_shell_git_timeout'] = 800

external_url "http://182.140.213.137:8899"
```

重启：`sudo gitlab-ctl reconfigure`

搞定。
```bash
# 将自己的项目推上去
# git remote remove origin 如果加错了可以删除之后再重新添加
git remote add origin ssh://git@182.140.213.137:44022/liwei/store.git
git push -u origin master
```

# docker 部署

[注意权限与端口的设置](https://thisiswangle.com/posts/2018-12-18-gitlab-docker-install/)

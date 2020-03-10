## 制作一个能用镜像

我的一个`srv`程序，是一个`rpc server`类的服务程序。制作能用镜像的`Dockerfile`:

```dockerfile
FROM ubuntu:18.04
LABEL maintainer="The Program Authors <liwei@xjgw.com>"

RUN mkdir -p /data/logs
WORKDIR /data
ADD ./sectorid_unix /data/sectorid
ADD ./cfg_release.json /data/cfg.json
EXPOSE 8899
VOLUME [ "/data" ]
ENTRYPOINT [ "/data/sectorid" ]
CMD [ "-c=/data/cfg.json" ]
```

打包镜像并且指定tag:
```bash
docker build -t emacsvi.com/lw-lotus/srv:0.0.1 .
```

## 启动镜像

```bash
# 启动
docker rm -f srv

# 182.140.213.131
docker run -d --name srv --user $(id -u):$(id -g) --restart=always --net=host -v /home/xjgw/docker/srv/data:/data:rw emacsvi.com/lw-lotus/srv:0.0.1
```

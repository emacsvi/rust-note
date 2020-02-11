prometheus相关的一些使用。
<!--more-->

## Prometheus


## Grafana使用

在Grafana之中，由于node_export都是9100，而我的其他一些应用是9101端口的，在配置变量的时候。想将所有的instance="10.0.20.x:9100"的替换为instance="10.0.20.x:9101"即可以同步node的dashboard界面一起输出。

并且尽量不要修改prometheus本身的数据。

新建一个hip的变量，

```go
Query=label_value(node_uname_info{job=~"$job",instance=~"$node"}, instance)
// 只获取ip地址，不要端口号的正则
Regex=/([\d.]*):.*/


Query=query_result(label_replace(node_uname_info{job=~"$job",instance=~"$node"}, "instance", "$1:9101", "instance", "(.*):.*"))
Regex=/.*instance="(.*)".*/
```

## 参考文献

- [模板化Dashboard](https://yunlzheng.gitbook.io/prometheus-book/part-ii-prometheus-jin-jie/grafana/templating)
- []()
- []()



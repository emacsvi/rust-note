prometheus相关的一些使用。
<!--more-->

## Prometheus


## Grafana使用

**替换标签**：

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

另外一个替换外网标签的例子：

例如，有时候我们想要动态修改变量查询结果。比如某一个节点绑定了多个ip，一个用于内网访问，一个用于外网访问，此时prometheus采集到的指标是内网的ip，但我们需要的是外网ip。这里我们想要能在Grafana中动态改变标签值，进行ip段的替换，而避免从prometheus或exporter中修改采集指标。
这时需要使用grafana的`query_result`函数
```go
// 将10.10.15.xxx段的ip地址替换为10.20.15.xxx段 注：替换端口同理
query_result(label_replace(kube_pod_info{pod=~"$pod"}, "node", "10.20.15.$1", "node", "10.10.15.(.*)"))
// 通过正则从返回结果中匹配出所需要的ip地址
regex：/.*node="(.*?)".*/
```

## 参考文献

- [模板化Dashboard](https://yunlzheng.gitbook.io/prometheus-book/part-ii-prometheus-jin-jie/grafana/templating)
- [prometheus的所有函数](https://prometheus.io/docs/prometheus/latest/querying/functions/#label_replace)
- []()



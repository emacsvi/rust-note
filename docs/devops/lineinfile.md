## lineinfile使用
常用参数：
- path: 必须指定参数，和file模块的path参数一样，指定要操作的文件，别名有：dest, destfile, name
- state: 确保某一行存在(state=present, 替换行),或者不存在(state=absent, 删除行)


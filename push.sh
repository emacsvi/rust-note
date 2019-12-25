echo "gitautopush start..."
GIT=`which git`
$GIT add --all
if [ ! -n "$1" ] ; then
  comm=`date "+%Y-%m-%d %H:%M:%S"`
else
  comm=$1
fi
$GIT commit -m ${comm}
echo "git提交注释:${comm}"
$GIT push origin master
echo "gitautopush end..."

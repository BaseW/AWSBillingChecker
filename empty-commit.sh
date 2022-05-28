if [ $# != 1 ];then
  echo "Please commit message"
  exit 1
fi

commit_message=$1

git commit --allow-empty -m "$commit_message"

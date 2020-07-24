# 生成对应题的文件

cwd=$(cd `dirname $0`;pwd);
num=$1
src=$cwd"/src"
solution=$src"/solution"
tFile=$solution"/l$num.rs"
modFile=$solution"/mod.rs"

if [ -e $tFile ];then 
	echo "file exist"
	exit -1
fi

echo "pub struct Solution{}" > $tFile
pbpaste >> $tFile

echo  "#[cfg(test)]\nmod test {\n\tuse super::*;\n\t#[test]\n\tfn test_l$num() {}\n}" >> $tFile

echo "pub mod l$num" >> $modFile
echo "done"



# 生成对应题的文件

cwd=$(cd `dirname $0`;pwd);
num=$1
p=$2
src=$cwd"/src"
solution=$src"/solution"
tFile=$solution"/l$num.rs"
modFile=$solution"/mod.rs"

if [ -e $tFile ];then 
	echo "file exist"
	exit -1
fi

echo "pub struct Solution{}" > $tFile

if [ "$p" -eq "1" ];then
	pbpaste >> $tFile
fi

echo  "#[cfg(test)]\nmod test {\n\tuse super::*;\n\t#[test]\n\tfn test_l$num() {}\n}" >> $tFile

echo "pub mod l$num;" >> $modFile
echo "done"



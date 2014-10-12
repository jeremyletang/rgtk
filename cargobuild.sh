set +e

./configure
export GTK_VERSION=${GTK_VERSION:=GTK_3_8}
echo $GTK_VERSION
make glue

mkdir -p /mnt/hdd/place-ext
cd /mnt/hdd/copy-place2

for f in *.tar.gz; do
    tar -xf $f -C /mnt/hdd/place-ext/
    echo $f
done

mkdir -p /mnt/hdd/place-final
cd /mnt/hdd/place-ext

for f in *-f-*.png; do
    cp $f /mnt/hdd/place-final/$f
    echo $f
done

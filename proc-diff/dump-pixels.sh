mkdir -p /mnt/hdd/place-final
cd /mnt/hdd/place-ext

for f in *.png; do
    sf=${f%%.*}
    if [[ $sf == *"-d-"* ]]; then
        # diff
        sem -j +0 convert $f sparse-color: | zstd > /mnt/hdd/place-final/$sf
    else
        # full
        cp $f /mnt/hdd/place-final/$f
    fi

    echo $sf
done

sem --wait

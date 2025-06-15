# Quick Start about QEMU on MacOS
Build QEMU:
`../configure --enable-hvf --enable-vnc --enable-virtfs --enable-vhost-user --enable-vhost-user-blk-server --target-list=aarch64-softmmu && make`

Build vhost-user-blk:
`make contrib/vhost-user-blk/vhost-user-blk`

Build qemu-storage-daemon:
`make storage-daemon/qemu-storage-daemon`

Download image and vmlinux(Optional):
```
wget https://repo.openeuler.org/openEuler-24.03-LTS/stratovirt_img/aarch64/openEuler-24.03-LTS-stratovirt-aarch64.img.xz && xz -d openEuler-24.03-LTS-stratovirt-aarch64.img.xz && wget https://repo.openeuler.org/openEuler-24.03-LTS/stratovirt_img/aarch64/std-vmlinux.bin
```

Run QEMU:
```
qemu-system-aarch64 \
    -m 256 \
    -machine virt \
    -accel hvf  \
    -cpu host \
    -nographic \
    -smp 4 \
    -kernel std-vmlinux-6.6.bin \
    -append "console=ttyAMA0 root=/dev/vda rw" \
    -drive if=none,id=image,file=openEuler-24.03-LTS-stratovirt-aarch64.img,format=raw \
    -device virtio-blk-device,drive=image \
    -serial stdio \
    -nodefaults
```
user: root password: openEuler12#$

Use vhost-user-blk:

```
./build/contrib/vhost-user-blk/vhost-user-blk -s /tmp/vhost.socket -b openEuler-24.03-LTS-stratovirt-aarch64.img
```

```
qemu-system-aarch64 \
    -m 512M \
    -machine virt,memory-backend=mem \
    -accel hvf  \
    -cpu host \
    -nographic \
    -smp 4 \
    -kernel std-vmlinux-6.6.bin \
    -append "console=ttyAMA0 root=/dev/vda rw" \
    -object memory-backend-shm,id=mem,size=512M \
    -device vhost-user-blk-pci,num-queues=1,chardev=char0 \
    -chardev socket,id=char0,path=/tmp/vhost.socket \
    -serial stdio \
    -nodefaults
```


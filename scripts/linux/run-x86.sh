sudo qemu-system-x86_64 -m 2G -cpu host -accel kvm -smp 8 \
  -drive file=openEuler-24.03-LTS-SP1-x86_64.qcow2,if=virtio,index=0,media=disk,format=qcow2 \
  -object memory-backend-memfd,id=mem0,size=2G \
  -numa node,memdev=mem0 \
  -chardev socket,id=char0,reconnect=0,path=/tmp/vhost4.socket \
  -device vhost-user-vsock-pci,chardev=char0 \
  -nographic \
  -serial mon:stdio \
  -vnc :1

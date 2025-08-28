# GSoC 2025 Project Report

* Organization: QEMU/[rust-vmm](https://github.com/rust-vmm)
* Project: [vhost-user_devices_in_Rust_on_macOS_and_*BSD](https://wiki.qemu.org/Google_Summer_of_Code_2025#vhost-user_devices_in_Rust_on_macOS_and_*BSD)
* Student: Wenyu Huang (uran0sH)
* Mentors: Stefano Garzarella (stefano-garzarella), German Maglione (germag), Oliver Steffen (osteffenrh)
* PRs:
  * [rust-vmm/vmm-sys-util](https://github.com/rust-vmm/vmm-sys-util):
    * [Make sock_ctrl_msg work on unix](https://github.com/rust-vmm/vmm-sys-util/pull/245) [merged]
    * [Implement EventNotifier/EventConsumer as a generic event notification](https://github.com/rust-vmm/vmm-sys-util/pull/244) [merged]
  * [rust-vmm/vhost](https://github.com/rust-vmm/vhost):
    * [Replace Eventfd with EventNotifier/EventConsumer](https://github.com/rust-vmm/vhost/pull/308) [merged]
    * [Use mio to replace Epoll](https://github.com/rust-vmm/vhost/pull/316) [pending]
  * [rust-vmm/vhost-device](https://github.com/rust-vmm/vhost-device):
    * [Make vhost-user-vsock run on macOS.](https://github.com/uran0sH/vhost-device/pull/1) [draft]
    * [Make vhost-user-console run on macOS](https://github.com/uran0sH/vhost-device/pull/2) [draft]

## Project Description

Extend rust-vmm crates (vhost, vhost-user-backend) to enable vhost-user device support on non-Linux POSIX systems (macOS/*BSD). The goal of this project is to make sure that we can use rust-vmm's vhost and vhost-user-backend crates on other POSIX systems besides Linux.

## My Contributions
In order to make vhost-user-device run on MacOS/BSD, we need to enable vmm-sys-util and vhost and other related components to run on macOS/BSD.

1. Make sock_ctrl_msg in vmm-sys-util work on unix.

In sock_ctrl_msg.rs, the CMSG_*! macros only support linux before adapting. So we need to adapt it to work on macOS/BSD. And in the unit test, eventfd was originally used to test the functionality of sock_ctrl_msg. However, eventfd is not supported on macOS/BSD. Therefore, we need to replace eventfd with pipefd to ensure that the unit test can run successfully on macOS/BSD. And then, fix some issues about macOS/BSD.

* PR(s): [#245](https://github.com/rust-vmm/vmm-sys-util/pull/245)

2. Implement EventNotifier/EventConsumer as a generic event notification

EventFd is used for event notification in vhost-user-backend. But eventfd is linux-specific. On macOS/BSD, we need to fallback to pipefd. I implemented an abstraction layer to unify eventfd and pipefd, called EventNotifier/EventConsumer. EventNotifier is used to send a notification. On macOS/BSD, it is pipefd's write end, and on Linux, it is eventfd. EventConsumer
is used to receive a notification. On macOS/BSD, it is pipefd's read end, and on Linux, it is eventfd.

* PR(s): [#244](https://github.com/rust-vmm/vmm-sys-util/pull/244)

3. Replace Eventfd with EventNotifier/EventConsumer in vhost-user-backend

Use EventNotifier/EventConsumer implemented in vmm-sys-util to replace eventfd in vhost-user-backend.

* PR(s): [#308](https://github.com/rust-vmm/vhost/pull/308)

4. Use mio to replace Epoll in vhost-user-backend

vhost-user-backend originally used epoll to monitor event notifications. But epoll is linux-specific. On macOS/BSD, we need to fallback to kqueue. mio is a cross-platform event notification library. It provides a unified interface for event notification on different platforms. So I use mio to replace epoll.

* PR(s): [#316](https://github.com/rust-vmm/vhost/pull/316)

5. Make vhost-device-console/vsock run on macOS/BSD

Apply the above changes to vhost-device-\*. Replace epoll with mio in vhost-device-\*.

* PR(s): [vhost-device-vsock](https://github.com/uran0sH/vhost-device/pull/1) [vhost-device-console](https://github.com/uran0sH/vhost-device/pull/2)

6. Others

Write some scripts to build and run qemu and vhost-user-device.(https://github.com/uran0sH/GSoC2025-vhost-user-bsd-macos/tree/main/scripts)

## Future Work

Continue to complete the adaptation of vhost-user-vsock and vhost-user-console.

* vhost-user-vsock:
  * Still not work on Linux. Test is blocking also on linux on this line: https://github.com/uran0sH/vhost-device/blob/c262162be7c5b512434f52d589b1d52c25f8fe6b/vhost-device-vsock/src/vhu_vsock.rs#L469 So the next step is try to fix this issue.
* vhost-user-console:
  * It can work on Linux, But not work on macOS. It can receive the first character, and then no response for the second character. See: https://github.com/uran0sH/vhost-device/pull/2#issuecomment-3218170160 . So the next step is try to fix this issue.

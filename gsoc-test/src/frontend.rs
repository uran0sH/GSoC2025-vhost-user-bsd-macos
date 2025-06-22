use std::io;
use std::os::fd::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;
use std::thread::sleep;
use std::time::Duration;

use crate::sock_ctrl_msg::ScmSocket;
mod errno;
mod sock_ctrl_msg;

fn send_event(fd: RawFd) -> io::Result<()> {
    let v = 1_u64;
    let ret = unsafe {
        libc::write(
            fd,
            &v as *const u64 as *const libc::c_void,
            size_of::<u64>(),
        )
    };
    if ret <= 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}



pub fn pipe() -> io::Result<(RawFd, RawFd)> {
    let mut fds: [RawFd; 2] = [-1, -1];
    let ret = unsafe { libc::pipe(fds.as_mut_ptr()) };
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        // read, write
        Ok((fds[0], fds[1]))
    }
}


fn main() {
    println!("Frontend: start");

    let (read_fd, write_fd) = pipe().expect("pipe failed");

    println!("Frontend: connect and send FD");
    let socket = UnixStream::connect("vhost-fake.sck").expect("failed to connect to frontend");
    socket
        .send_with_fds(&[[237].as_ref()], &[read_fd.as_raw_fd(), write_fd.as_raw_fd()])
        .expect("failed to send FD");

    for e in 0..3 {
        sleep(Duration::from_secs(2));

        println!("Frontend: send event: {e}");
        send_event(write_fd.as_raw_fd()).expect("failed to send event");
    }

    println!("Frontend: done");
}

use libc::{iovec, read};
use std::os::fd::RawFd;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::{io, mem};
#[cfg(target_os = "linux")]
use vmm_sys_util::epoll::{ControlOperation, Epoll, EpollEvent, EventSet};

mod errno;
mod sock_ctrl_msg;
use sock_ctrl_msg::ScmSocket;

struct Listener(UnixListener);

impl Listener {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let _ = std::fs::remove_file(&path);
        let fd = UnixListener::bind(&path)?;
        Ok(Listener { 0: fd })
    }
}

fn read_event(fd: RawFd) -> io::Result<()> {
    let mut buf: u64 = 0;
    let ret = unsafe {
        read(
            fd,
            &mut buf as *mut u64 as *mut libc::c_void,
            size_of::<u64>(),
        )
    };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn main() {
    println!("Backend: start");
    let listener = Listener::new("vhost-fake.sck").expect("failed to create listener");

    println!("Backend: waiting for connections");
    let (socket_fd, _saddr) = listener.0.accept().expect("failed to accept connection");

    // receive an FD for events
    let mut files = [0; 2];
    let mut buf = [0u8];
    let mut iovecs = [iovec {
        iov_base: buf.as_mut_ptr() as *mut libc::c_void,
        iov_len: buf.len(),
    }];

    println!("Backend: waiting for FDs");
    let (_read_count, file_count) = unsafe {
        socket_fd
            .recv_with_fds(&mut iovecs[..], &mut files)
            .expect("failed to recv fd")
    };

    assert_eq!(file_count, 2, "checking the number of received fds");
    assert!(files[0] >= 0);

    let fd = files[0];
    println!("Backend: waiting for events");
    #[cfg(target_os = "linux")]
    {
        use vmm_sys_util::epoll::{ControlOperation, Epoll, EpollEvent, EventSet};
        let epoll = Epoll::new().expect("failed to create epoll");
        epoll
            .ctl(
                ControlOperation::Add,
                fd,
                EpollEvent::new(EventSet::IN, fd as u64),
            )
            .expect("failed to ctl epoll");

        // let's poll the FD until is ready to be read
        let mut num_evnt = 0;
        const EPOLL_EVENTS_LEN: usize = 100;
        let mut events = vec![EpollEvent::new(EventSet::empty(), 0); EPOLL_EVENTS_LEN];
        'epoll: loop {
            let num_events = match epoll.wait(-1, &mut events[..]) {
                Ok(res) => res,
                Err(e) => {
                    if e.kind() == io::ErrorKind::Interrupted {
                        continue;
                    }
                    panic!("failed to wait for epoll: {}", e);
                }
            };

            for event in events.iter().take(num_events) {
                let _evset = match EventSet::from_bits(event.events) {
                    Some(evset) => evset,
                    None => {
                        let evbits = event.events;
                        println!("epoll: ignoring unknown event set: 0x{:x}", evbits);
                        continue;
                    }
                };

                // we received and event in FD, we must read it
                read_event(fd).expect("failed to read event");
                println!("Backend: event {num_evnt} received.");
                num_evnt += 1;

                if num_evnt >= 3 {
                    break 'epoll;
                }
            }
        }
    }
    #[cfg(all(target_os = "macos", feature = "smol"))]
    {
        use polling::{Event, Events, Poller};
        let key = 1;
        let poller = Poller::new().unwrap();
        unsafe {
            poller
                .add_with_mode(fd, Event::readable(key), polling::PollMode::Level)
                .unwrap();
        }
        let mut events = Events::new();
        let mut num_event = 0;
        loop {
            events.clear();
            poller.wait(&mut events, None).unwrap();

            for ev in events.iter() {
                if ev.key == key {
                    read_event(fd).expect("failed to read event");
                    println!("receive events {}", num_event);
                    num_event = num_event + 1;
                }
            }

            if num_event >= 3 {
                break;
            }
        }
    }
    #[cfg(all(target_os = "macos", feature = "tokio"))]
    {
        use mio::unix::SourceFd;
        use mio::{Events, Interest, Poll, Token};

        let mut poll = Poll::new().unwrap();
        let mut events = Events::with_capacity(128);
        poll.registry()
            .register(&mut SourceFd(&fd), Token(0), Interest::READABLE)
            .unwrap();

        let mut num_evet = 0;
        loop {
            poll.poll(&mut events, None).unwrap();

            for event in events.iter() {
                match event.token() {
                    Token(0) => {
                        read_event(fd).expect("failed to read event");
                        println!("receive evnet {}", num_evet);
                        num_evet = num_evet + 1;
                    }
                    _ => unreachable!(),
                }
            }
            if num_evet >= 3 {
                break;
            }
        }
    }

    println!("Backend: done.");
}

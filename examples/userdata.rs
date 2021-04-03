use gmrs::prelude::*;
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    sync::mpsc,
};

struct GmodTcp {
    sender: mpsc::Sender<Vec<u8>>,
    on_receive: AtomicRef,
}
impl GmodTcp {
    fn new(stream: TcpStream) -> std::io::Result<UserData<Self>> {
        let (data_tx, data_rx) = mpsc::channel();

        let reader = stream.try_clone()?;
        let writer = stream;
        let on_receive = AtomicRef::default();

        let reader_on_receive = on_receive.clone();
        let writer_on_receive = on_receive.clone();

        let ud = UserData::new(Self {
            sender: data_tx,
            on_receive,
        });

        let reader_ud = ud.clone();
        std::thread::spawn(move || {
            GmodTcp::reader_loop(reader, reader_ud, reader_on_receive);
        });

        let writer_ud = ud.clone();
        std::thread::spawn(move || {
            GmodTcp::writer_loop(writer, writer_ud, data_rx, writer_on_receive);
        });

        Ok(ud)
    }

    fn reader_loop(mut reader: TcpStream, ud: UserData<Self>, on_receive: AtomicRef) {
        loop {
            let mut buffer = vec![0u8; 4096];
            match reader.read(&mut buffer) {
                Err(e) => {
                    Self::failure(on_receive, ud.clone(), e);
                    break;
                }
                Ok(0) => {
                    let callback = on_receive.clone();
                    let sock = ud.clone();
                    gmrs::remote_try_execute(move |state| {
                        gmrs::print(state, "Socket closed, calling on_receive");
                        lua::push(state, callback);
                        lua::pcall_result_with(state, 0, |state| {
                            lua::push(state, sock);
                            lua::push_nil(state);
                            lua::push(state, "closed");
                        })
                    });
                    break;
                }
                Ok(amount) => {
                    buffer.resize(amount, 0);
                    let callback = on_receive.clone();
                    let sock = ud.clone();
                    gmrs::remote_try_execute(move |state| {
                        gmrs::print(state, "Data received, calling on_receive");
                        lua::push(state, callback);
                        lua::pcall_result_with(state, 0, |state| {
                            lua::push(state, sock);
                            lua::push(state, buffer.as_slice());
                        })
                    });
                }
            }
        }
    }

    fn writer_loop(
        mut writer: TcpStream,
        ud: UserData<Self>,
        receiver: mpsc::Receiver<Vec<u8>>,
        on_receive: AtomicRef,
    ) {
        while let Ok(data) = receiver.recv() {
            match writer.write_all(&data) {
                Err(e) => {
                    Self::failure(on_receive, ud.clone(), e);
                    break;
                }
                _ => continue,
            }
        }
    }

    fn failure(on_receive: AtomicRef, ud: UserData<Self>, error: std::io::Error) {
        gmrs::remote_try_execute(move |state| {
            gmrs::print(state, "Socket failure, calling on_receive");
            lua::push(state, on_receive);
            lua::pcall_result_with(state, 0, |state| {
                lua::push(state, ud);
                lua::push_nil(state);
                lua::push_string(state, &format!("{}", error));
            })
        });
    }
}
impl UserType for GmodTcp {
    fn build_metatable(builder: &mut MetatableBuilder<Self>) {
        builder.method("send", gmod_tcp_send);
        builder.method("on_recv", gmod_tcp_on_receive);
    }
}

#[gmrs::function]
fn gmod_tcp_send(this: UserData<GmodTcp>, data: Vec<u8>) -> lua::Result<()> {
    this.with(|tcp| tcp.sender.send(data))?;
    Ok(())
}

#[gmrs::function]
// on_receive : function(socket, data, error)
fn gmod_tcp_on_receive(
    this: UserData<GmodTcp>,
    on_receive: OwnedRef,
    state: LuaState,
) -> lua::Result<()> {
    gmrs::print(state, "Setting on_receive callback");
    this.with(|tcp| tcp.on_receive.replace(on_receive));
    Ok(())
}

#[gmrs::function]
// on_connect : function(socket, err)
fn socket_connect(addr: String, on_connect: OwnedRef) -> lua::Result<()> {
    let addr: SocketAddr = addr.parse()?;
    std::thread::spawn(move || {
        match TcpStream::connect(addr).and_then(|stream| GmodTcp::new(stream)) {
            Ok(tcp) => {
                gmrs::remote_try_execute(move |state| {
                    lua::push(state, on_connect);
                    lua::pcall_result_with(state, 0, |state| {
                        lua::push(state, tcp);
                    })
                });
            }
            Err(e) => {
                gmrs::remote_try_execute(move |state| {
                    lua::push(state, on_connect);
                    lua::pcall_result_with(state, 0, |state| {
                        lua::push_nil(state);
                        lua::push(state, format!("{}", e));
                    })
                });
            }
        }
    });
    Ok(())
}

#[gmrs::entry]
fn main(state: LuaState) {
    let socket = lua::create_table(state);
    socket.set(state, "connect", NativeFunc::new(socket_connect));
    gmrs::set_global(state, "socket", socket);
}

#[gmrs::exit]
fn exit(_state: LuaState) {}

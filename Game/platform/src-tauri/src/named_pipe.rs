use std::ffi::CString;
use std::ptr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::fileapi::{ReadFile, WriteFile};
use winapi::um::handleapi::CloseHandle;
use winapi::um::namedpipeapi::{ConnectNamedPipe, DisconnectNamedPipe};
use winapi::um::winbase::{
    CreateNamedPipeA, PIPE_ACCESS_DUPLEX, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE, PIPE_WAIT,
};
use winapi::um::winnt::{HANDLE, LONG, LPCSTR};

const BUFFER_SIZE: u32 = 1024;
const MAX_CLIENTS: u32 = 2;

pub type CallFunction = fn(String);

pub fn named_pipe(counter: Arc<Mutex<i32>>, caller: CallFunction) {
    let num_clients = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];

    loop {
        if num_clients.load(Ordering::SeqCst) < MAX_CLIENTS {
            let pipe_name: LPCSTR = "\\\\.\\pipe\\my_bidirectional_pipe\0".as_ptr() as *const i8;

            let h_pipe = unsafe {
                CreateNamedPipeA(
                    pipe_name,
                    PIPE_ACCESS_DUPLEX,
                    PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                    MAX_CLIENTS,
                    BUFFER_SIZE,
                    BUFFER_SIZE,
                    0,
                    ptr::null_mut(),
                )
            };

            if h_pipe == (winapi::um::handleapi::INVALID_HANDLE_VALUE as HANDLE) {
                eprintln!("Failed to create named pipe.");
                return;
            }

            let result = unsafe { ConnectNamedPipe(h_pipe, ptr::null_mut()) };
            if result != 0 {
                println!(
                    "[*] Conectado - Cliente [{}]",
                    num_clients.load(Ordering::SeqCst)
                );
                num_clients.fetch_add(1, Ordering::SeqCst);

                if let Ok(mut count) = counter.lock() {
                    *count += 1;
                }

                let h_pipe_clone = h_pipe;
                let num_clients_clone = num_clients.clone();
                let thread_args = PipeThreadArgs {
                    h_pipe: h_pipe_clone,
                    num_clients: num_clients_clone,
                    counter: Arc::clone(&counter),
                    call: caller
                };

                let thread_handle = thread::spawn(move || {
                    message_thread(&thread_args as *const _ as LPVOID);
                });

                handles.push(thread_handle);
            }
        }
    }
}

struct PipeThreadArgs {
    h_pipe: HANDLE,
    num_clients: Arc<AtomicU32>,
    counter: Arc<Mutex<i32>>,
    call: CallFunction,
}

unsafe impl Send for PipeThreadArgs {}

fn message_thread(lp_parameter: LPVOID) -> LONG {
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut bytes_read: DWORD = 0;
    let mut bytes_written: DWORD = 0;

    unsafe {
        let args: *const PipeThreadArgs = lp_parameter as *const PipeThreadArgs;
        let h_pipe: HANDLE = (*args).h_pipe;
        let num_clients = (*args).num_clients.clone();

        loop {
            if ReadFile(
                h_pipe,
                buffer.as_mut_ptr() as *mut _,
                buffer.len() as DWORD,
                &mut bytes_read,
                ptr::null_mut(),
            ) != 0
            {
                let received_message = String::from_utf8_lossy(&buffer[..bytes_read as usize]);
                println!("Received: {}", received_message);
                ((*args).call)(received_message.to_string());
                if received_message.trim() == "exit" {
                    break;
                }

                // Respond back to the client
                let response = format!("Server received: {}", received_message);
                let c_response = CString::new(response).expect("CString conversion failed");
                WriteFile(
                    h_pipe,
                    c_response.as_ptr() as *const c_void,
                    c_response.as_bytes_with_nul().len() as DWORD,
                    &mut bytes_written,
                    ptr::null_mut(),
                );
            } else {
                break;
            }
        }

        println!("Desconectando pipe !");
        num_clients.fetch_sub(1, Ordering::SeqCst);

        if let Ok(mut count) = (*args).counter.lock() {
            *count -= 1;
        }

        DisconnectNamedPipe(h_pipe);
        CloseHandle(h_pipe);
    }

    0 // Return a suitable value here
}

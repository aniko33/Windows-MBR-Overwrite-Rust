use windows::{
    core::w,
    Win32::{Storage::FileSystem::{
        self, WriteFile, FILE_FLAGS_AND_ATTRIBUTES, FILE_GENERIC_WRITE, FILE_SHARE_READ,
        FILE_SHARE_WRITE, OPEN_EXISTING,
    }, Foundation::CloseHandle},
};

use std::{fs, io::Read};

fn main() {
    let mut mbrbuf = Vec::new();

    {
        let mut mbrbin = fs::File::open("mbr.bin").unwrap();
        mbrbin.read_to_end(&mut mbrbuf).expect("Failed reading: new MBR");
    }

    unsafe {
        let mbrh = FileSystem::CreateFileW(
            w!("\\\\.\\PhysicalDrive0"),
            FILE_GENERIC_WRITE.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,                                                                                                                                      
            FILE_FLAGS_AND_ATTRIBUTES(0                                                                                                                                     ),
            None,
        )
        .expect("CreateFileW failed");

        WriteFile(mbrh, Some(mbrbuf.as_slice()), None, None).expect("WriteFile failed");

        CloseHandle(mbrh).unwrap();
    }
}

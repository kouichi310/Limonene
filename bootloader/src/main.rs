// ブートローダはUEFIアプリなんだね
//標準ライブラリstdを使わない（OS依存なので） stdioみたいなもんやん
#![no_std]
//default entry pointのmain関数を使わん
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;
mod console;
mod uefi;
mod uefi_alloc;

use alloc::format;
use console::*;
use core::{panic::PanicInfo, ptr::null};
use uefi::*;

fn get_memory_map_unicode(memory_type_number: u32) -> &'static str {
    match memory_type_number {
        0 => "EfiReservedMemoryType",
        1 => "EfiLoaderCode",
        2 => "EfiLoaderData",
        3 => "EfiBootServicesCode",
        4 => "EfiBootServicesData",
        5 => "EfiRuntimeServiceCode",
        6 => "EfiRuntimeServiceData",
        7 => "EfiConventionalMemory",
        8 => "EfiUnusableMemory",
        9 => "EfiACPIReclaimMemory",
        10 => "EfiACPIMemoryNVS",
        11 => "EfiMemoryMappedIO",
        12 => "EfiMemoryMappedIOPortSpace",
        13 => "EfiPalCode",
        14 => "EfiPersistentMemory",
        15 => "EfiUnacceptedMemoryType",
        16 => "EfiMaxMemoryType",
        _ => "Unknown Memory Type",
    }
}

fn open_root_dir(
    image_handle: EfiHandle,
    bs: &EfiBootServices,
) -> Result<&EfiFileProtocol, EfiStatus> {
    unsafe {
        let _loaded_image = bs
            .open_protocol(
                image_handle,
                &EFI_LOADED_IMAGE_PROTOCOL,
                image_handle,
                null(),
                EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
            )
            .unwrap();

        let loaded_image = ((_loaded_image as *const _) as *const EfiLoadedImageProtocol)
            .as_ref()
            .unwrap();

        println!("[open-root-dir] open image done!");

        let _fs = bs
            .open_protocol(
                loaded_image.device_handle,
                &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
                image_handle,
                null(),
                EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
            )
            .unwrap();

        let fs = ((_fs as *const _) as *const EfiSimpleFileSystemProtocol)
            .as_ref()
            .unwrap();

        println!("[open-root-dir] open file system done!");

        fs.open_volume()
    }
}

fn save_memory_map(
    map: &[u8],
    file: &EfiFileProtocol,
    descriptor_size: usize,
    map_size: usize,
) -> EfiStatus {
    let header = "Index,\tType,\tType(name),\tPhysicalStart,\tNumberOfPages,\tAttribute\n";
    let len = header.len();

    let written_size = file.write(len, header).unwrap();

    if written_size != len {
        panic!(
            "Faild to write completely. len:{} done: {}",
            len, written_size
        );
    }

    let mut index = 0;
    let mut offset = 0;

    while offset < map_size {
        let memory_descriptor = unsafe {
            (map.as_ptr().add(offset) as *const EfiMemoryDescriptor)
                .as_ref()
                .unwrap()
        };
        let mem_region_info = format!(
            "{:},\t0x{:x},\t{:},\t0x{:x},\t0x{:x},\t0x{:x}\n",
            index,
            memory_descriptor.memory_type,
            get_memory_map_unicode(memory_descriptor.memory_type),
            memory_descriptor.physical_start,
            memory_descriptor.number_of_pages,
            memory_descriptor.attribute
        );
        file.write(mem_region_info.len(), &mem_region_info);

        index += 1;
        offset += descriptor_size;
    }

    file.close().unwrap();
    EfiStatus::Success
}

struct MemoryMap<'a> {
    buffer_size: usize,
    buffer: &'a mut [u8],
    map_size: usize,
    map_key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

//コンパイル時にマングリングしないように指定する
//マングリング(名前修飾)とは、コンパイラが関数や変数の名前を変更し一意にすること
// void hoge(int)を _ZNhogeEintみたいな感じにかえちゃうらしいC++とか
// UEFIは仕様書でC言語の呼び出し規約を使わないとらしい
//　呼び出し規約は、ABIの一部でサブルーチン呼び出し時とかに、何をどうスタックに格納するかとか書いてあるとか
#[no_mangle]
pub extern "C" fn efi_main(image_handle: EfiHandle, system_table: &EfiSystemTable) -> EfiStatus {
    uefi_alloc::init(system_table.boot_services(), system_table.con_out());
    console::init(system_table.con_out());

    println!("Hello Limonene");

    let mut memory_map: [u8; 4096] = [0; 4096];

    let (map_size, _, descriptor_size, _) = system_table
        .boot_services()
        .get_memory_map(&mut memory_map)
        .unwrap();

    let efi_file_protocol = open_root_dir(image_handle, system_table.boot_services()).unwrap();

    let opened_handle = efi_file_protocol
        .open(
            "\\memmap",
            EfiFileOpenMode::CreateReadWrite,
            EfiFileAttribute::None,
        )
        .unwrap();
    save_memory_map(&memory_map, &opened_handle, descriptor_size, map_size);
    if efi_file_protocol.close().unwrap() == EfiStatus::Success {
        println!("file closed");
    }
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("{}", _panic);
    loop {}
}

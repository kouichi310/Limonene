use core::{ffi::c_void, ptr::{self, null_mut}};
use alloc::{string::String, vec::Vec}; //将来的に消したいね


#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(C)]
pub enum EfiStatus {
    Success = 0,
}

#[repr(C)]
pub struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

type CHAR16 = u16;
type EfiVirtualAddress = u64;
type EfiPhysicalAddress = u64;
type NotImplemented = usize;
pub type EfiHandle = *const c_void;

pub struct EfiSimpleTextInputProtocol {}
pub struct EfiRuntimeServices {}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct EfiMemoryDescriptor {
    pub memory_type: u32, //UEFI-Specではtypeって名前
    pub physical_start: EfiPhysicalAddress,
    pub virtual_start: EfiVirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64,
}

#[repr(C)]
pub enum EfiMemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType
}

#[repr(u64)]
pub enum EfiFileOpenMode {
    Read = 0x1,
    ReadWrite = 0x2 | 0x1,
    CreateReadWrite = 0x8000_0000_0000_0000| 0x1 | 0x2,
}

#[repr(u64)]
pub enum EfiFileAttribute {
    None = 0x0,
    ReadOnly = 0x1,
    Hidden = 0x2,
    System = 0x4,
    Reserved = 0x8,
    Directory = 0x10,
    Archive = 0x20,
    ValidAttribute = 0x37,
}

#[repr(C)]
pub struct EfiGuid {
    data_1:      u32,
    data_2:      u16,
    data_3:      u16,
    data_4:      [u8; 8],
}

pub const EFI_LOADED_IMAGE_PROTOCOL: EfiGuid = EfiGuid {
    data_1: 0x5b1b31a1,
    data_2: 0x9562,
    data_3: 0x11d2,
    data_4: [0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
};

pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data_1: 0x0964e5b22,
    data_2: 0x6459,
    data_3: 0x11d2,
    data_4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
};

pub const EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL: u32 = 0x00000001;

#[repr(C)]
pub struct EfiBootServices {
    create_event: NotImplemented,
    create_event_ex: NotImplemented,
    close_event: NotImplemented,
    signal_event: NotImplemented,
    wait_for_event: NotImplemented,
    check_event: NotImplemented,
    set_timer: NotImplemented,
    raise_tpl: NotImplemented,
    restore_tpl: NotImplemented,
    allocate_page: NotImplemented,
    free_page: NotImplemented,
    get_memory_map: fn(
        memory_map_size:&mut usize,
        memory_map: &mut [EfiMemoryDescriptor],
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> EfiStatus,
    allocate_pool: fn(pool_type: EfiMemoryType, size: usize, buffer: &mut *mut u8) -> EfiStatus,
    free_pool: fn(address: *mut u8) -> EfiStatus,
    install_protocol_interface: NotImplemented,
    uninstall_protocol_interface: NotImplemented,
    reinstall_protocol_interface: NotImplemented,
    register_protocol_interface: NotImplemented,
    register_protocol_notify: NotImplemented,
    locate_handle: NotImplemented,
    handle_protocol: NotImplemented,
    locate_device_path: NotImplemented,
    open_protocol: fn(handle: EfiHandle, protocl: &EfiGuid, interface: &mut *mut c_void,agent_handle: EfiHandle, controller_handle: EfiHandle, attributes: u32) -> EfiStatus,
    close_protocol: NotImplemented,
    open_protocol_information: NotImplemented,
    connect_controller: NotImplemented,
    disconnect_controller: NotImplemented,
    protocols_per_handle: NotImplemented,
    locate_handle_buffer: NotImplemented,
    locate_protocol: NotImplemented,
    install_multiple_protocol_interfaces: NotImplemented,
    uninstall_multiple_protocol_interfaces: NotImplemented,
    load_image: NotImplemented,
    start_image: NotImplemented,
    unload_image: NotImplemented,
    efi_image_entry_point: NotImplemented,
    exit: NotImplemented,
    exit_boot_services: NotImplemented,
	set_watch_dog_timer: NotImplemented,
	stall:	NotImplemented,
	copy_mem:	NotImplemented,
	set_mem:NotImplemented,
	get_next_monotonic_count:NotImplemented,
	install_configuration_table:NotImplemented,
	calculate_crc_32:NotImplemented,
}

impl EfiBootServices {
	pub fn get_memory_map(&self, memory_map: &mut [EfiMemoryDescriptor]) -> Result<(usize, usize, usize, u32), EfiStatus> {
		let mut memory_map_size = memory_map.len() * core::mem::size_of::<EfiMemoryDescriptor>();
		let mut map_key = 0;
		let mut descriptor_size = 0;
		let mut descriptor_version = 0;

		let _res = (self.get_memory_map)(
			&mut memory_map_size,
			memory_map,
			&mut map_key,
			&mut descriptor_size,
			&mut descriptor_version,
		);

		if _res == EfiStatus::Success {
			Ok((
				memory_map_size,
				map_key,
				descriptor_size,
				descriptor_version,
			))
		} else {
			Err(_res)
		}
	}

    pub unsafe fn open_protocol(&self, handle: EfiHandle, protocl: &EfiGuid, agent_handle: EfiHandle, controller_handle: EfiHandle, attributes: u32) -> Result<&c_void, EfiStatus> {
        let mut _interface: *mut c_void = null_mut();
        let interface_ptr = &mut _interface;

        let _res = (self.open_protocol)(
            handle,
            protocl,
            interface_ptr,
            agent_handle,
            controller_handle,
            attributes,
        );

        if _res == EfiStatus::Success {
            if interface_ptr.is_null() {
                // !todo!("RETURN NULL")
            }
            Ok(interface_ptr.as_ref().unwrap())
        } else {
            Err(_res)
        }
    }

    pub fn allocate_pool(&self, pooltype: EfiMemoryType, size: usize) -> Result<*mut u8, ()> {
        let mut buffer = ptr::null_mut();
        let buffer_ptr = &mut buffer;
        if (self.allocate_pool)(pooltype, size, buffer_ptr) as i32 == 0 {
            assert!(!((*buffer_ptr).is_null()));
            Ok(*buffer_ptr)
        } else {
            Err(())
        }
    }

    pub fn free_pool(&self, buffer: *mut u8) -> Result<(), ()> {
        if (self.free_pool)(buffer) == EfiStatus::Success {
            Ok(())
        } else {
            Err(())
        }
    }
}
pub struct EfiConfigurationTable {}


#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    reset: fn(This: &EfiSimpleTextOutputProtocol, ExtendedVerification: bool) -> EfiStatus,
    output_string: fn(This: &EfiSimpleTextOutputProtocol, String: *const CHAR16) -> EfiStatus,
    _unuse0: u64,
    _unuse1: u64,
    _unuse2: u64,
    _unuse3: u64,
    clear_screen: fn(This: &EfiSimpleTextOutputProtocol) -> EfiStatus,
    _unuse5: u64,
    _unuse6: u64,
    _unuse7: u64,
}

impl EfiSimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> EfiStatus {
        (self.reset)(self, extended_verification)
    }
    pub fn output_string(&self, string: *const CHAR16) -> EfiStatus {
        (self.output_string)(self, string)
    }
}

#[repr(C)]
pub struct EfiSystemTable {
    hdr: EfiTableHeader,
    firmware_vender: *const CHAR16,
    firmware_revision: u32,
    console_in_handle: EfiHandle,
    con_in: *mut EfiSimpleTextInputProtocol,
    console_out_handle: EfiHandle,
    con_out: *mut EfiSimpleTextOutputProtocol,
    standard_error_handle: EfiHandle,
    std_err: *mut EfiSimpleTextOutputProtocol,
    runtime_services: *mut EfiRuntimeServices,
    boot_services: *mut EfiBootServices,
    number_of_table_entries: usize, //32bitCPUではu32
    configuration_table: *mut EfiConfigurationTable,
}

impl EfiSystemTable {
    pub fn con_out(&self) -> &mut EfiSimpleTextOutputProtocol {
        unsafe { &mut *self.con_out }
    }

    pub fn boot_services(&self) -> &EfiBootServices {
        unsafe { &*self.boot_services }
    }
}

#[repr(C)]
pub struct EfiDevicePathProtocol {
    protocol_type:  u8,
    sub_type:       u8,
    length:         [u8;2],
}

#[repr(C)]
pub struct EfiLoadedImageProtocol<'a> {
    revision:   u32,
    parent_handle:  EfiHandle,
    system_table:   &'a EfiSystemTable,
    pub device_handle:  EfiHandle,
    file_path:      &'a EfiDevicePathProtocol,
    reserved:       &'a c_void,
    load_option_size:   u32,
    load_options:   &'a c_void,
    image_base:     &'a c_void,
    image_size:     u64,
    image_code_type:    EfiMemoryType,
    image_data_type:    EfiMemoryType,
    unload:         fn(imageHandle: EfiHandle) -> EfiStatus,
}

pub struct EfiFileIoToken {}

#[repr(C)]
pub struct EfiFileProtocol {
    pub revision:   u64,
    open:   fn(this: &EfiFileProtocol, newHandle: &mut *mut EfiFileProtocol, filename: *const CHAR16, openMode: u64, attribute: u64) -> EfiStatus,
    close:  fn(this: &EfiFileProtocol) -> EfiStatus,
    delete: fn(this: &EfiFileProtocol) -> EfiStatus,
    read:   fn(this: &EfiFileProtocol, bufferSize: &usize, buffer: &c_void) -> EfiStatus,
    write:  fn(this: &EfiFileProtocol, bufferSize: &usize, buffer: *const c_void) -> EfiStatus,
    get_position:   fn(this: &EfiFileProtocol, position: &u64) -> EfiStatus,
    set_position:   fn(this: &EfiFileProtocol, position: &u64) -> EfiStatus,
    get_info:       fn(this: &EfiFileProtocol, informationType: &EfiGuid, bufferSize: &usize, buffer: &c_void) -> EfiStatus,
    set_info:       fn(this: &EfiFileProtocol, informationType: &EfiGuid, bufferSize: &usize, buffer: &c_void) -> EfiStatus,
    flash:  fn(this: &EfiFileProtocol) -> EfiStatus,
    open_ex:    fn(this: &EfiFileProtocol, newHandle: &&EfiFileProtocol, fileName: &CHAR16, openMode: &u64, attribute: &u64, token: &EfiFileIoToken) -> EfiStatus,
    read_ex:    fn(this: &EfiFileProtocol, token: &EfiFileIoToken) -> EfiStatus,
    write_ex:   fn(this: &EfiFileProtocol, token: &EfiFileIoToken) -> EfiStatus,
    flash_ex:   fn(this: &EfiFileProtocol, token: &EfiFileIoToken) -> EfiStatus,
}

impl EfiFileProtocol {
    pub fn open(&self, file_name: &str, open_mode: EfiFileOpenMode, attribute: EfiFileAttribute) -> Result<&EfiFileProtocol, EfiStatus> {
        let mut new_handle = ptr::null_mut();
        let new_handle_ptr = &mut new_handle;
        let _text = String::from(file_name);
        let _null_terminated_text = _text + "\0";
        let u16_str: Vec<u16> = _null_terminated_text.encode_utf16().into_iter().collect();
        let u16ed_filename_ptr = u16_str.as_ptr();

        let _res = (self.open)(
            &self,
            new_handle_ptr,
            u16ed_filename_ptr,
            open_mode as u64,
            attribute as u64,
        );

        if _res == EfiStatus::Success {
            unsafe { Ok(new_handle.as_ref().unwrap())}
        } else {
            Err(_res)
        }
    }

    pub fn write(&self, buffer_size: usize, buffer: &str) -> Result<usize, EfiStatus> {
        let mut written_buffer_size = buffer_size;
        let _res = (self.write)(self, &mut written_buffer_size, buffer.as_ptr() as *const _);

        if _res == EfiStatus::Success {
            Ok(written_buffer_size)
        } else {
            Err(_res)
        }
    }

    pub fn close(&self) -> Result<EfiStatus, EfiStatus> {
        let _res = (self.close)(self);
        if _res == EfiStatus::Success {
            Ok(_res)
        } else {
            Err(_res)
        }
    }
}

#[repr(C)]
pub struct EfiSimpleFileSystemProtocol {
    revision:   u64,
    open_volume:    fn(this: &EfiSimpleFileSystemProtocol, root: &mut *mut EfiFileProtocol) -> EfiStatus,
}

impl EfiSimpleFileSystemProtocol {
    pub unsafe fn open_volume(&self) -> Result<&EfiFileProtocol, EfiStatus> {
        let mut efi_file_proto = ptr::null_mut();
        let mut efi_file_proto_ptr  = &mut efi_file_proto;

        let _res = (self.open_volume)(self, efi_file_proto_ptr);
        if _res == EfiStatus::Success {
            Ok(efi_file_proto_ptr.as_ref().unwrap())
        } else {
            Err(_res)
        }
    }
}

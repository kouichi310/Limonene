use core::ffi::c_void;

#[repr(C)]
pub enum EfiStatus {
	Success = 0
}

#[repr(C)]
pub struct EfiTableHeader {
	signature:      u64,
	revision:       u32,
	header_size:    u32,
	crc32:          u32,
	reserved:       u32,
}

type CHAR16 = u16;

pub struct EfiSimpleTextInputProtocol{}
pub struct EfiRuntimeServices{}
pub struct EfiBootServices{}
pub struct EfiConfigurationTable{}
#[repr(C)]
pub struct EfiHandle(*mut c_void);

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
	_unuse7: u64
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
	hdr:                        EfiTableHeader,
	firmware_vender:            *const CHAR16,
	firmware_revision:          u32,
	console_in_handle:          EfiHandle,
	con_in:                     *mut EfiSimpleTextInputProtocol,
	console_out_handle:         EfiHandle,
	con_out:                    *mut EfiSimpleTextOutputProtocol,
	standard_error_handle:      EfiHandle,
	std_err:                    *mut EfiSimpleTextOutputProtocol,
	runtime_services:           *mut EfiRuntimeServices,
	boot_services:              *mut EfiBootServices,
	number_of_table_entries:    usize, //32bitCPUではu32
	configuration_table:        *mut EfiConfigurationTable,
}

impl EfiSystemTable {
	pub fn con_out(&self) -> &mut EfiSimpleTextOutputProtocol {
		unsafe {&mut *self.con_out}
	}
}

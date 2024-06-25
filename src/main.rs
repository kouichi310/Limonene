// ブートローダはUEFIアプリなんだね
//標準ライブラリstdを使わない（OS依存なので） stdioみたいなもんやん
#![no_std]
//default entry pointのmain関数を使わん
#![no_main]

use core::panic::PanicInfo;
use utf16_literal::utf16;
use core::arch::asm;

mod uefi;

//コンパイル時にマングリングしないように指定する
//マングリング(名前修飾)とは、コンパイラが関数や変数の名前を変更し一意にすること
// void hoge(int)を _ZNhogeEintみたいな感じにかえちゃうらしいC++とか
// UEFIは仕様書でC言語の呼び出し規約を使わないとらしい
//　呼び出し規約は、ABIの一部でサブルーチン呼び出し時とかに、何をどうスタックに格納するかとか書いてあるとか
#[no_mangle]
pub extern "C" fn efi_main(_image_handle: uefi::EfiHandle, system_table: &uefi::EfiSystemTable) -> uefi::EfiStatus {
    let _conout = system_table.con_out();
    _conout.reset(false);
    _conout.output_string(utf16!("Hello, World by Limonene\r\n").as_ptr());

    loop{}

    uefi::EfiStatus::Success
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{
        unsafe { asm!("hlt") };
    }
}

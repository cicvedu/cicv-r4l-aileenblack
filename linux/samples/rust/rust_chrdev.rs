// SPDX-License-Identifier: GPL-2.0

//! Rust character device sample.

use core::result::Result::Err;

use kernel::prelude::*;
use kernel::sync::Mutex;
use kernel::{chrdev, file};

const GLOBALMEM_SIZE: usize = 0x1000;

module! {
    type: RustChrdev,
    name: "rust_chrdev",
    author: "Rust for Linux Contributors",
    description: "Rust character device sample",
    license: "GPL",
}

static GLOBALMEM_BUF: Mutex<[u8;GLOBALMEM_SIZE]> = unsafe {
    Mutex::new([0u8;GLOBALMEM_SIZE])
};

struct RustFile {
    #[allow(dead_code)]
    inner: &'static Mutex<[u8;GLOBALMEM_SIZE]>,
   // inner: &'static Mutex<Vec<u8>>,
   //inner:Mutex<Vec<u8>>,
}

#[vtable]
impl file::Operations for RustFile {
    type Data = Box<Self>;

    fn open(_shared:&(), _file: &file::File) -> Result<Box<Self>> {
        Ok(
        //unsafe {
            Box::try_new(RustFile {
                inner: &GLOBALMEM_BUF
               
               //inner:_shared.inner.clone(),
               
            })?
        //    }
        )
       // Ok(())
    }

    fn write(_this: &Self,_file: &file::File,_reader: &mut impl kernel::io_buffer::IoBufferReader,_offset:u64,) -> Result<usize> {
        //Err(EPERM)
       
        pr_info!("character write\n");
        let copy = _reader.read_all()?;
        let len = copy.len();
        let v=&copy;
        //let mut det=&*_this.inner.lock();
         let mut inner = _this.inner.lock();
        if len < GLOBALMEM_SIZE{
        let mut id=0;
        while id<len{
        inner[id]=v[id];
        id+=1;
        }
        Ok(len)
        }else{
        Ok(0)
        }
        
    }

    fn read(_this: &Self,_file: &file::File,_writer: &mut impl kernel::io_buffer::IoBufferWriter,_offset:u64,) -> Result<usize> {
        //Err(EPERM)
         pr_info!("character read\n", );
        let offset = _offset.try_into()?;
        let vec = _this.inner.lock(); // 获取锁，避免脏读
        let len = core::cmp::min(_writer.len(), vec.len().saturating_sub(offset));
        _writer.write_slice(&vec[offset..][..len])?;
        //pr_info!("{}\n",vec[offset..][...len]);
        Ok(len)
    }
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl kernel::Module for RustChrdev {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust character device sample (init)\n");

        let mut chrdev_reg = chrdev::Registration::new_pinned(name, 0, module)?;

        // Register the same kind of device twice, we're just demonstrating
        // that you can use multiple minors. There are two minors in this case
        // because its type is `chrdev::Registration<2>`
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustChrdev { _dev: chrdev_reg })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust character device sample (exit)\n");
    }
}

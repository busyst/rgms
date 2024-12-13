use std::fs::{File, OpenOptions};
use std::io::Read;
use std::ops::{Index, IndexMut};
use std::cmp::max;

// Constants and type definitions
const SIG: u32 = (b'M' as u32) << 24 | (b'I' as u32) << 16 | (b'D' as u32) << 8 | (b'G' as u32);

#[repr(u8)]
#[derive(Clone, Copy)]
enum GBImageFlags {
    Alpha = 0,
    Luminosity = 1,
    RGB = 2,
    RGBA = 4,
    ExtendedHeader = 8,
    Reserved1 = 16,
    Reserved2 = 32,
    Reserved3 = 64,
    UseIndices = 128,
}

#[repr(C)]
#[derive(Debug, Clone)]
struct GDImageHeader {
    signature: u32,
    width: u16,
    height: u16,
    flags: u8,
    mipmap_count: u8,
    unique_colors: u16,
    checksum: u32,
}

impl Default for GDImageHeader {
    fn default() -> Self {
        GDImageHeader {
            signature: SIG,
            width: 0,
            height: 0,
            flags: 0,
            mipmap_count: 0,
            unique_colors: 0,
            checksum: 0,
        }
    }
}

struct GDMainImageData {
    data: Vec<u8>,
}

// Helper functions
fn calc_crc16_1021f(mut curcrc: u32, block: &[u8], len: usize) -> u32 {
    for i in 0..len {
        curcrc ^= (block[i] as u32) << 8;
        for _ in 0..8 {
            if (curcrc & 0x8000) != 0 {
                curcrc = (curcrc << 1) ^ 0x1021F;
            } else {
                curcrc = curcrc << 1;
            }
        }
    }
    curcrc
}

fn bytes_per_pixel(flags: u8) -> u8 {
    let x = flags & 7;
    1 + (x / 2) + if x > 1 { 1 } else { 0 }
}

fn fast_image_check(data: &[u8]) -> bool {
    if data.len() < std::mem::size_of::<GDImageHeader>() {
        return false;
    }

    // Safe because we checked the length
    let header = unsafe {
        &*(data.as_ptr() as *const GDImageHeader)
    };

    if header.signature != SIG {
        println!("Original signature: {}", SIG);
        println!("Calculated signature: {}", header.signature);
        return false;
    }

    let mut calculated_checksum = calc_crc16_1021f(0, &data[4..12], 8);
    calculated_checksum = (calculated_checksum << 16) | calc_crc16_1021f(calculated_checksum, &data[4..12], 8);

    if header.checksum != calculated_checksum {
        println!("Original checksum: {}", header.checksum);
        println!("Calculated checksum: {}", calculated_checksum);
        return false;
    }

    if (header.width == 0 || header.height == 0) && 
       (header.flags & GBImageFlags::ExtendedHeader as u8) != 0 {
        return false;
    }

    true
}

fn mipmaps_size(mut init_w: u16, mut init_h: u16, bytes_per_pixel: u16) -> u64 {
    let mut data_len = 0u64;
    loop {
        data_len += (init_w as u64) * (init_h as u64) * (bytes_per_pixel as u64);
        if init_w != 1 || init_h != 1 {
            init_w = max(1, (init_w + 1) / 2);
            init_h = max(1, (init_h + 1) / 2);
        } else {
            break;
        }
    }
    data_len
}

// Main image handling struct
pub struct GDImageFile {
    file: Option<File>,
}

impl GDImageFile {
    pub fn new() -> Self {
        GDImageFile { file: None }
    }

    pub fn open(&mut self, path: &str) -> std::io::Result<()> {
        self.file = Some(OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(path)?);
        Ok(())
    }

    pub fn open_and_fast_check(&mut self, path: &str) -> std::io::Result<bool> {
        let metadata = std::fs::metadata(path)?;
        if metadata.len() < 21 {
            return Ok(false);
        }

        self.file = Some(OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(path)?);

        let mut header = GDImageHeader::default();
        let header_slice = unsafe {
            std::slice::from_raw_parts_mut(
                &mut header as *mut GDImageHeader as *mut u8,
                std::mem::size_of::<GDImageHeader>(),
            )
        };

        self.file.as_mut().unwrap().read_exact(header_slice)?;
        
        Ok(fast_image_check(header_slice))
    }
}

// Image data template equivalent
pub struct ImageData<T> {
    width: u16,
    height: u16,
    data: Vec<T>,
}

impl<T: Default + Clone> ImageData<T> {
    pub fn new(width: u16, height: u16) -> Self {
        ImageData {
            width,
            height,
            data: vec![],
        }
    }
    pub fn new_allocate(width: u16, height: u16) -> Self {
        ImageData {
            width,
            height,
            data: vec![T::default(); width as usize * height as usize],
        }
    }
    pub fn with_data(width: u16, height: u16, data: Vec<T>) -> Self {
        ImageData { width, height, data }
    }
    pub fn allocate(&mut self){
        self.data = vec![T::default(); self.width as usize * self.height as usize];
    }
    pub fn pixel(&mut self, x: u16, y: u16) -> &mut T {
        if x >= self.width || y >= self.height {
            panic!("x or y is out of bounds of the image");
        }
        &mut self.data[y as usize * self.width as usize + x as usize]
    }

    pub fn get_data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr() as *const u8,
                self.data.len() * std::mem::size_of::<T>(),
            )
        }
    }
    // Basic map that takes current value and returns new value
    pub fn map_each<F>(&mut self, mut function: F)
    where
        F: FnMut(T) -> T,
        T: Clone,
    {
        self.data.iter_mut()
            .for_each(|item| *item = function(item.clone()));
    }
    // Map that provides x, y coordinates
    pub fn map_coords<F>(&mut self, mut function: F)
    where
        F: FnMut(u16, u16) -> T,
        T: Clone,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y as usize * self.width as usize) + x as usize;
                self.data[idx] = function(x, y);
            }
        }
    }
    pub fn get_data_size(&self) -> usize {
        self.width as usize * self.height as usize * std::mem::size_of::<T>()
    }
    // You might also want a mutable version:
    pub fn as_u8_slice_mut(&mut self) -> &mut [u8] {
        // SAFETY: This is safe because we're reading the bytes of a properly aligned T array
        // and the lifetime is tied to self
        unsafe {
            std::slice::from_raw_parts_mut(
                self.data.as_mut_ptr() as *mut u8,
                self.data.len() * std::mem::size_of::<T>()
            )
        }
    }
    
    pub fn width(&self) -> u16 {
        self.width
    }
    
    pub fn height(&self) -> u16 {
        self.height
    }
}
impl<T> Index<(u16, u16)> for ImageData<T> {
    type Output = T;

    fn index(&self, idx: (u16, u16)) -> &Self::Output {
        let (x, y) = idx;
        if x >= self.width || y >= self.height {
            panic!("Index out of bounds: trying to access ({}, {}) in {}x{} image", 
                  x, y, self.width, self.height);
        }
        &self.data[y as usize * self.width as usize + x as usize]
    }
}

impl<T> IndexMut<(u16, u16)> for ImageData<T> {
    fn index_mut(&mut self, idx: (u16, u16)) -> &mut Self::Output {
        let (x, y) = idx;
        if x >= self.width || y >= self.height {
            panic!("Index out of bounds: trying to access ({}, {}) in {}x{} image", 
                  x, y, self.width, self.height);
        }
        &mut self.data[y as usize * self.width as usize + x as usize]
    }
}


use crate::{Color, Framebuffer};

impl Framebuffer {
    pub fn rgb24_into(&self, buf: &mut [u8], pitch: Option<usize>) {
        let row_sz = self.width * size_of::<Color>();

        match pitch {
            None | Some(0) => {
                let src_bytes = unsafe {
                    std::slice::from_raw_parts(self.pixels.as_ptr() as *const u8, row_sz)
                };

                buf.copy_from_slice(src_bytes);
                return;
            }
            _ => {}
        }

        let pitch = pitch.unwrap();

        for y in 0..self.height {
            let dest_start = y * pitch;
            let dest_slice = &mut buf[dest_start..dest_start + row_sz];

            let src_start = y * self.width;
            let src_slice = &self.pixels[src_start..src_start + self.width];

            let src_bytes =
                unsafe { std::slice::from_raw_parts(src_slice.as_ptr() as *const u8, row_sz) };

            dest_slice.copy_from_slice(src_bytes);
        }
    }
}

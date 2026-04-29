/*
 * Copyright (c) Radzivon Bartoshyk 2025/5. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1.  Redistributions of source code must retain the above copyright notice, this
 * list of conditions and the following disclaimer.
 *
 * 2.  Redistributions in binary form must reproduce the above copyright notice,
 * this list of conditions and the following disclaimer in the documentation
 * and/or other materials provided with the distribution.
 *
 * 3.  Neither the name of the copyright holder nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
use pic_scale::{
    BufferStore, ImageStore, ImageStoreMut, ResamplingFunction, Scaler, ThreadingPolicy,
    WorkloadStrategy,
};
use std::slice;

#[unsafe(no_mangle)]
pub extern "C" fn pixart_scale_plane_u16(
    src: *const u16,
    src_stride: usize,
    width: u32,
    height: u32,
    dst: *mut u16,
    new_width: u32,
    new_height: u32,
    bit_depth: usize,
) {
    unsafe {
        let source_image: std::borrow::Cow<[u16]>;
        let mut j_src_stride = src_stride / 2;

        if src as usize % 2 != 0 || src_stride % 2 != 0 {
            let mut _src_slice = vec![0u16; width as usize * height as usize];
            let j = slice::from_raw_parts(src as *const u8, src_stride * height as usize);

            for (dst, src) in _src_slice
                .chunks_exact_mut(width as usize)
                .zip(j.chunks_exact(src_stride))
            {
                for (dst, src) in dst.iter_mut().zip(src.chunks_exact(2)) {
                    let pixel = u16::from_ne_bytes([src[0], src[1]]);
                    *dst = pixel;
                }
            }
            source_image = std::borrow::Cow::Owned(_src_slice);
            j_src_stride = width as usize;
        } else {
            source_image = std::borrow::Cow::Borrowed(slice::from_raw_parts(
                src,
                src_stride / 2 * height as usize,
            ));
        }

        let _source_store = ImageStore::<u16, 1> {
            buffer: source_image,
            channels: 1,
            width: width as usize,
            height: height as usize,
            stride: j_src_stride,
            bit_depth,
        };

        let scaler = Scaler::new(ResamplingFunction::Lanczos3)
            .set_threading_policy(ThreadingPolicy::Single)
            .set_workload_strategy(WorkloadStrategy::PreferQuality);

        if dst as usize % 2 != 0 {
            let mut dst_store =
                ImageStoreMut::alloc_with_depth(new_width as usize, new_height as usize, bit_depth);

            let plan = scaler
                .plan_planar_resampling16(_source_store.size(), dst_store.size(), bit_depth)
                .unwrap();

            plan.resample(&_source_store, &mut dst_store).unwrap();

            let dst_slice = slice::from_raw_parts_mut(
                dst as *mut u8,
                new_width as usize * 4 * new_height as usize,
            );

            for (src, dst) in dst_store
                .as_bytes()
                .chunks_exact(dst_store.stride())
                .zip(dst_slice.chunks_exact_mut(new_width as usize))
            {
                for (src, dst) in src.iter().zip(dst.as_chunks_mut::<2>().0.iter_mut()) {
                    let bytes = src.to_ne_bytes();
                    dst[0] = bytes[0];
                    dst[1] = bytes[1];
                }
            }
        } else {
            let dst_stride =
                slice::from_raw_parts_mut(dst, new_height as usize * new_width as usize);
            let buffer = BufferStore::Borrowed(dst_stride);
            let mut dst_store = ImageStoreMut::<u16, 1> {
                buffer,
                width: new_width as usize,
                height: new_height as usize,
                bit_depth,
                channels: 1,
                stride: new_width as usize,
            };

            let plan = scaler
                .plan_planar_resampling16(_source_store.size(), dst_store.size(), bit_depth)
                .unwrap();

            plan.resample(&_source_store, &mut dst_store).unwrap();
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pixart_scale_plane_u8(
    src: *const u8,
    src_stride: u32,
    width: u32,
    height: u32,
    dst: *mut u8,
    dst_stride: u32,
    new_width: u32,
    new_height: u32,
) {
    unsafe {
        let origin_slice = slice::from_raw_parts(src, src_stride as usize * height as usize);
        let dst_slice = slice::from_raw_parts_mut(dst, dst_stride as usize * new_height as usize);

        let source_store = ImageStore::<u8, 1> {
            buffer: std::borrow::Cow::Borrowed(origin_slice),
            channels: 1,
            width: width as usize,
            height: height as usize,
            stride: src_stride as usize,
            bit_depth: 8,
        };

        let scaler = Scaler::new(ResamplingFunction::Lanczos3)
            .set_threading_policy(ThreadingPolicy::Single)
            .set_workload_strategy(WorkloadStrategy::PreferQuality);

        let mut dst_store = ImageStoreMut::<u8, 1> {
            buffer: BufferStore::Borrowed(dst_slice),
            channels: 1,
            width: new_width as usize,
            height: new_height as usize,
            stride: dst_stride as usize,
            bit_depth: 8,
        };
        let plan = scaler
            .plan_planar_resampling(source_store.size(), dst_store.size())
            .unwrap();

        plan.resample(&source_store, &mut dst_store).unwrap();
    }
}

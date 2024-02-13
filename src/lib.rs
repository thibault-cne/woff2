#![allow(clippy::too_many_arguments)]
#![allow(clippy::result_unit_err)]

#[macro_use]
extern crate cpp;

use ::std::mem::MaybeUninit;

#[link(name = "brotli")]
extern "C" {}

cpp! {{
    #include <woff2/encode.h>

    using std::string;
    using woff2::MaxWOFF2CompressedSize;
    using woff2::ConvertTTFToWOFF2;
    using woff2::WOFF2Params;
}}

/// brotliQuality should normally be 11, allowTransforms should normally be true
pub fn convert_ttf_to_woff2(
    ttf_font_bytes: &[u8],
    additional_extended_metadata_bytes: &[u8],
    brotli_quality: u8,
    allow_transforms: bool,
) -> Result<Vec<u8>, ()> {
    debug_assert!(
        brotli_quality < 12,
        "brotliQuality should be between 0 and 11 inclusive"
    );

    let capacity = _private::max_woff2_compressed_size(
        ttf_font_bytes.len(),
        additional_extended_metadata_bytes.len(),
    );
    let mut woff_font_bytes = Vec::with_capacity(capacity);
    let mut woff_font_bytes_length = MaybeUninit::<usize>::uninit();

    let success = _private::convert_ttf_to_woff2(
        ttf_font_bytes.as_ptr(),
        ttf_font_bytes.len(),
        woff_font_bytes.as_mut_ptr(),
        woff_font_bytes_length.as_mut_ptr(),
        additional_extended_metadata_bytes.as_ptr(),
        additional_extended_metadata_bytes.len(),
        brotli_quality as i32,
        allow_transforms,
    );
    if success {
        unsafe { woff_font_bytes.set_len(*woff_font_bytes_length.as_ptr()) };
        woff_font_bytes.shrink_to_fit();
        Ok(woff_font_bytes)
    } else {
        Err(())
    }
}

mod _private {
    #[inline(always)]
    pub fn max_woff2_compressed_size(length: usize, extended_metadata_length: usize) -> usize {
        length + 1024 + extended_metadata_length
    }

    pub fn convert_ttf_to_woff2(
        data: *const u8,
        length: usize,
        result: *mut u8,
        result_length: *mut usize,
        extended_metadata: *const u8,
        extended_metadata_length: usize,
        brotli_quality: i32,
        allow_transforms: bool,
    ) -> bool {
        unsafe {
            cpp!([data as "const uint8_t *", length as "size_t", result as "uint8_t *", result_length as "size_t *", extended_metadata as "const char *", extended_metadata_length as "size_t", brotli_quality as "int", allow_transforms as "bool"] -> bool as "bool"
            {
                string copyOfExtendedMetadata(extended_metadata, extended_metadata_length);

                struct WOFF2Params params;
                params.extended_metadata = copyOfExtendedMetadata;
                params.brotli_quality = brotli_quality;
                params.allow_transforms = allow_transforms;

                return ConvertTTFToWOFF2(data, length, result, result_length, params);
            })
        }
    }
}

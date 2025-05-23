//
//  PlatformImage.h
//  avif.swift [https://github.com/awxkee/avif.swift]
//
//  Created by Radzivon Bartoshyk on 04/05/2022.
//
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
//
//  The above copyright notice and this permission notice shall be included in
//  all copies or substantial portions of the Software.
//
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
//  THE SOFTWARE.
//

#ifndef PlatformImage_h
#define PlatformImage_h

#import "TargetConditionals.h"

#if TARGET_OS_OSX
#import <AppKit/AppKit.h>
#define Image   NSImage
#else
#import <UIKit/UIKit.h>
#define Image   UIImage
#endif

typedef NS_ENUM(NSUInteger, PreferredCodec) {
    kAOM NS_SWIFT_NAME(AOM),
    kSVTAV1 NS_SWIFT_NAME(SVTAV1)
};

static void AV1CGDataProviderReleaseDataCallback(void * _Nullable info, const void * _Nullable data, size_t size) {
    if (data) free((void*)data);
}

typedef NS_ENUM(NSUInteger, EnclosedColorSpace) {
    kSRGB NS_SWIFT_NAME(sRGB),
    kBt2020 NS_SWIFT_NAME(bt2020),
    kBt2020PQ NS_SWIFT_NAME(bt2020pq),
    kBt2020HLG NS_SWIFT_NAME(bt2020hlg),
    kDisplayP3 NS_SWIFT_NAME(displayP3),
    kDisplayP3PQ NS_SWIFT_NAME(displayP3pq),
    kDisplayP3HLG NS_SWIFT_NAME(displayP3hlg),
    kBt709 NS_SWIFT_NAME(bt709),
    kBt709PQ NS_SWIFT_NAME(bt709pq),
    kBt709HLG NS_SWIFT_NAME(bt709hlg),
};

@interface EnclosedImage : NSObject
-(nonnull uint8_t*)data;
-(EnclosedColorSpace)colorSpace;
-(bool)sourceHasAlpha;
@end

@interface Image (ColorData)
-(nullable EnclosedImage*)rgbaPixels:(nonnull uint32_t*)imageWidth imageHeight:(nonnull uint32_t*)imageHeight;
@end


#endif /* PlatformImage_h */

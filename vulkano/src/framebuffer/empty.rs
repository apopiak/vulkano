// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use framebuffer::traits::RenderPassDesc;
use framebuffer::traits::LayoutAttachmentDescription;
use framebuffer::traits::LayoutPassDescription;
use framebuffer::traits::LayoutPassDependencyDescription;

/// Description of an empty render pass.
///
/// Can be used to create a render pass with one subpass and no attachment.
///
/// # Example
///
/// ```
/// use vulkano::framebuffer::EmptySinglePassRenderPassDesc;
/// use vulkano::framebuffer::RenderPassDesc;
///
/// # let device: std::sync::Arc<vulkano::device::Device> = return;
/// let rp = EmptySinglePassRenderPassDesc.build_render_pass(device.clone());
/// ```
///
#[derive(Debug, Copy, Clone)]
pub struct EmptySinglePassRenderPassDesc;

unsafe impl RenderPassDesc for EmptySinglePassRenderPassDesc {
    #[inline]
    fn num_attachments(&self) -> usize {
        0
    }

    #[inline]
    fn attachment(&self, num: usize) -> Option<LayoutAttachmentDescription> {
        None
    }

    #[inline]
    fn num_subpasses(&self) -> usize {
        1
    }

    #[inline]
    fn subpass(&self, num: usize) -> Option<LayoutPassDescription> {
        if num == 0 {
            Some(LayoutPassDescription {
                color_attachments: vec![],
                depth_stencil: None,
                input_attachments: vec![],
                resolve_attachments: vec![],
                preserve_attachments: vec![],
            })
        } else {
            None
        }
    }

    #[inline]
    fn num_dependencies(&self) -> usize {
        0
    }

    #[inline]
    fn dependency(&self, num: usize) -> Option<LayoutPassDependencyDescription> {
        None
    }

    #[inline]
    fn num_color_attachments(&self, subpass: u32) -> Option<u32> {
        if subpass == 0 {
            Some(0)
        } else {
            None
        }
    }

    #[inline]
    fn num_samples(&self, _: u32) -> Option<u32> {
        None
    }

    #[inline]
    fn has_depth_stencil_attachment(&self, subpass: u32) -> Option<(bool, bool)> {
        if subpass == 0 {
            Some((false, false))
        } else {
            None
        }
    }

    #[inline]
    fn has_depth(&self, subpass: u32) -> Option<bool> {
        if subpass == 0 {
            Some(false)
        } else {
            None
        }
    }

    #[inline]
    fn has_writable_depth(&self, subpass: u32) -> Option<bool> {
        if subpass == 0 {
            Some(false)
        } else {
            None
        }
    }

    #[inline]
    fn has_stencil(&self, subpass: u32) -> Option<bool> {
        if subpass == 0 {
            Some(false)
        } else {
            None
        }
    }

    #[inline]
    fn has_writable_stencil(&self, subpass: u32) -> Option<bool> {
        if subpass == 0 {
            Some(false)
        } else {
            None
        }
    }
}

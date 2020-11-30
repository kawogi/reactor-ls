use glium::glutin::{ContextBuilder, GlProfile, GlRequest, Robustness, event_loop::EventLoop, dpi::Size, window::WindowBuilder};
use glium::backend::glutin::DisplayCreationError;
use log::debug;

use std::string::ToString;

// Version details about WebGL 2.0 - Source: https://en.wikipedia.org/wiki/WebGL#Design
// "WebGL 2.0 is based on OpenGL ES 3.0"

// Version details about OpenGL ES 3.0 - Source: https://en.wikipedia.org/wiki/OpenGL_ES#OpenGL_ES_3.0
// "OpenGL 4.3 provides full compatibility with OpenGL ES 3.0. Version 3.0 is also the basis for WebGL 2.0. Actual is version 3.0.6."

// Version details about OpenGL ES 3.0 Shading Language - Source: https://en.wikipedia.org/wiki/OpenGL_Shading_Language#Versions
// GLSL ES version: 3.00.6
// OpenGL ES version: 3.0
// WebGL version: 2.0
// Based on GLSL version: 3.30
// date: 29 January 2016
// Shader Preprocessor: #version 300 es 

// Version details about GLSL 3.30 - Source: https://en.wikipedia.org/wiki/OpenGL_Shading_Language#Versions
// GLSL version: 3.30.6
// OpenGL Version: 3.3
// date: 11 March 2010
// Shader Preprocessor: #version 330 

// Version details about OpenGL 4.3 - Source: https://en.wikipedia.org/wiki/OpenGL#OpenGL_4.3
// Version: 4.3
// Release Date: August 6, 2012
// Features: GLSL 4.30, … , compatibility with OpenGL ES 3.0

// Version details about GLSL 4.3 - Source: https://en.wikipedia.org/wiki/OpenGL_Shading_Language#Versions
// GLSL Version: 4.30.8
// OpenGL version: 4.3
// Date: 7 February 2013
// Shader Preprocessor: #version 430 

// Summing this mess up we need to stay compatible with (most restricted API first):
// WebGL 2.0
// OpenGL ES 3.0 with GLSL ES 3.0 (aka `#version 300 es`)
// OpenGL 4.3 with GLSL 4.3 (aka `#version 430`)


#[allow(clippy::doc_markdown)]
/// The OpenGL version to use; WebGL 2.0 is based on OpenGL ES 3.0 which is based on OpenGL 4.3
const GL_VERSION: (u8 ,u8) = (4, 3);
const GLES_VERSION: (u8 ,u8) = (3, 0);
const GL_REQUEST: GlRequest = GlRequest::GlThenGles{ opengl_version: GL_VERSION, opengles_version: GLES_VERSION };

/// The OpenGL profile to use; we play it safe for now
const GL_PROFILE: GlProfile = GlProfile::Core;

/// What errors to check for and what to do in an emergency situation. We try to catch everything for now.
const GL_ROBUSTNESS: Robustness = Robustness::TryRobustNoResetNotification;

/// Number of bits to use for the depth buffer
const GL_DEPTH_BUFFER_BITS: u8 = 24; // TODO this is a wild guess; should rather be based on some facts


#[allow(clippy::clippy::module_name_repetitions)]
pub fn create<T>(event_loop: &EventLoop<T>, size: Size) -> Result<glium::Display, DisplayCreationError> {

    let window_builder = WindowBuilder::new()
            .with_inner_size(size)
            ;
    let context_builder = ContextBuilder::new()
            .with_gl(GL_REQUEST)
            .with_gl_profile(GL_PROFILE)
            .with_gl_robustness(GL_ROBUSTNESS)
            .with_vsync(true) // no constant as this might be configurable soon
            .with_multisampling(0) // no constant as this might be configurable soon
            .with_depth_buffer(GL_DEPTH_BUFFER_BITS)
            ;

    glium::Display::new(window_builder, context_builder, &event_loop)
}

pub fn dump_details(display: &glium::Display) {
    let (max_viewport_width, max_viewport_height) = display.get_max_viewport_dimensions();
    debug!("max viewport dimensions: {} × {}", max_viewport_width, max_viewport_height);
    let (framebuffer_width, framebuffer_height) = display.get_framebuffer_dimensions();
    debug!("framebuffer dimensions : {} × {}", framebuffer_width, framebuffer_height);
    let free_memory_str = display.get_free_video_memory().map_or_else(|| "(unknown)".to_string(), |memory| memory.to_string());
    debug!("free video memory      : {}", free_memory_str);
    let max_anisotropy_str = display.get_max_anisotropy_support().map_or_else(|| "(unknown)".to_string(), |anisotropy| anisotropy.to_string());
    debug!("max anisotropy support : {}", max_anisotropy_str);
    let opengl_profile_str = display.get_opengl_profile().map_or_else(|| "(unknown)".to_string(), |profile| format!("{:?}", profile));
    debug!("opengl profile         : {}", opengl_profile_str);
    debug!("opengl renderer string : {}", display.get_opengl_renderer_string());
    debug!("opengl vendor string   : {}", display.get_opengl_vendor_string());
    debug!("opengl version         : {:?}", display.get_opengl_version());
    debug!("opengl version string  : {}", display.get_opengl_version_string());
    debug!("release behavior       : {:?}", display.get_release_behavior());
    debug!("supported glsl version : {:?}", display.get_supported_glsl_version());
    debug!("context loss possible  : {}", if display.is_context_loss_possible() { "yes" } else { "no" });
    debug!("debug                  : {}", if display.is_debug() { "yes" } else { "no" });
    debug!("forward compatible     : {}", if display.is_forward_compatible() { "yes" } else { "no" });
    debug!("robust                 : {}", if display.is_robust() { "yes" } else { "no" });
}


use vulkano::buffer::cpu_pool::CpuBufferPool;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::format::Format;
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass};
use vulkano::image::attachment::AttachmentImage;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;
use vulkano::pipeline::vertex::TwoBuffersDefinition;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::swapchain;
use vulkano::swapchain::{
    AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
    SwapchainCreationError,
};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};

use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use std::iter;
use std::sync::Arc;
use std::time::Instant;
use std::borrow::Borrow;

#[derive(Clone)]
pub struct VulkanContext {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

#[derive(Clone)]
pub struct VulkanWindow {
    pub context: VulkanContext,
    pub swapchain: Arc<Swapchain<Window>>,
}

impl VulkanContext {
    pub fn new() -> VulkanContext {
        let required_extensions = vulkano_win::required_extensions();
        let instance = Instance::new(None, &required_extensions, None).unwrap();
        let physical: PhysicalDevice = PhysicalDevice::enumerate(&instance).next().unwrap();
        println!(
            "Using device: {} (type: {:?})",
            physical.name(),
            physical.ty()
        );

        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_graphics() && q.supports_compute())
            // .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
            .unwrap();

        let device_ext = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };

        let (device, mut queues) = Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned(),
        ).unwrap();

        VulkanContext {
            device,
            queue: queues.next().unwrap(),
        }
    }
}

impl VulkanWindow {
    pub fn new(context: &VulkanContext) -> VulkanWindow {
        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, context.device.instance().clone())
            .unwrap();
        let dimensions: [u32; 2] = surface.window().inner_size().into();

        let (mut swapchain, images) = {
            let caps = surface.capabilities(context.device.physical_device()).unwrap();
            let format = caps.supported_formats[0].0;
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();

            Swapchain::new(
                context.device.clone(),
                surface.clone(),
                caps.min_image_count,
                format,
                dimensions,
                1,
                ImageUsage::color_attachment(),
                &context.queue,
                SurfaceTransform::Identity,
                alpha,
                PresentMode::Fifo,
                FullscreenExclusive::Default,
                true,
                ColorSpace::SrgbNonLinear,
            ).unwrap()
        };

        VulkanWindow {
            context: context.clone(),
            swapchain,
        }
    }
}

impl Drop for VulkanWindow {
    fn drop(&mut self) {
        println!("Vulkan Window drop");
    }
}
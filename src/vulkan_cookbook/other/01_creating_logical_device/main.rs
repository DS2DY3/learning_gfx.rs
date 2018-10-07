extern crate winit;
#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;
extern crate gfx_hal as hal;

use hal::format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle};
use hal::pass::Subpass;
use hal::pso::{PipelineStage, ShaderStageFlags};
use hal::queue::Submission;
use hal::{
    buffer, command, format as f, image as i, memory as m, pass, pool, pso, window::Extent2D,
};
use hal::{Backbuffer, DescriptorPool, FrameSync, Primitive, SwapchainConfig};
use hal::{Device, Instance, PhysicalDevice, Surface, Swapchain};
use hal::QueueFamily;


fn main() {
    let mut events_loop = winit::EventsLoop::new();

    let wb = winit::WindowBuilder::new()
        .with_title("01 - Creating Logical Device");

    let (_window, _instance, mut adapters, mut surface) = {
        let window = wb.build(&events_loop).unwrap();
        let instance = back::Instance::create("creating_logical_device", 1);
        let surface = instance.create_surface(&window);
        let adapters = instance.enumerate_adapters();
        (window, instance, adapters, surface)
    };

    println!("there are {} device on the complter", adapters.len());
    if adapters.len() > 0 {
        // enumerating available physical devices
        let adapter = &adapters[0];
        // print features
        println!("device features: {:?}", adapter.physical_device.features());
        println!("device limits: {:?}", adapter.physical_device.limits());
        println!("device info: {:?}", adapter.info);
        // queuefamily
        let queue_families = &adapter.queue_families;
        for queue_family in queue_families {

            println!("adapter queue families: {:?}, {:?}, {:?}, {:?}, {:?}",
                     queue_family.id(), queue_family.max_queues(), queue_family.queue_type(), queue_family.supports_graphics(), queue_family.supports_compute());
        }
        // checking available device extensions
    }


    events_loop.run_forever(|event| {
        // println!("{:?}", event);

        match event {
            winit::Event::WindowEvent {
                event: winit::WindowEvent::CloseRequested,
                ..
            } => winit::ControlFlow::Break,
            _ => winit::ControlFlow::Continue,
        }
    });
}

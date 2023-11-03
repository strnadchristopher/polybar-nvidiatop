use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::Clock;
use nvml_wrapper::enum_wrappers::device::ClockId;
use std::thread::sleep;
use std::time::Duration;
use std::env;
use std::fmt;
#[derive(Debug, Clone, Copy)]
enum OutputOption{
    MemFreePercentage,
    MemFreeMb,
    MemUsagePercentage,
    MemUsageMb,
    MemTotalMb,
    ClockSpeed,
    MaxBoostClockSpeed,
    Temperature,
    GpuUtilization,
    GpuMemoryUtilization,
}
impl fmt::Display for OutputOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
fn main() {
    const SHOULD_LOOP: bool = false;
    
    let nvml = match Nvml::init(){
        Ok(result) => result,
        Err(_error) => panic!("Nvml not installed."),
    };

    let device = match nvml.device_by_index(0){
        Ok(device) => device,
        Err(_error) => panic!("Could not find NVIDIA device.")
    };
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let output_option = match args[1].as_str(){
            "--free-memory" => OutputOption::MemFreePercentage,
            "--free-memory-mb" => OutputOption::MemFreeMb,
            "--used-memory" => OutputOption::MemUsagePercentage,
            "--used-memory-mb" => OutputOption::MemUsageMb,
            "--total-memory-mb" => OutputOption::MemTotalMb,
            "--clock-speed" => OutputOption::ClockSpeed,
            "--max-boost-clock-speed" => OutputOption::MaxBoostClockSpeed,
            "--temperature" => OutputOption::Temperature,
            "--gpu-utilization" => OutputOption::GpuUtilization,
            "--gpu-memory-utilization" => OutputOption::GpuMemoryUtilization,
            _ => panic!("Invalid output option"),
        };
        if SHOULD_LOOP{
            start_loop(output_option, &device);
        }else{
            run_once(output_option, &device);
        }
    }else{
        if SHOULD_LOOP{
            start_loop(OutputOption::MemFreePercentage, &device);
        }else{
            run_once(OutputOption::MemFreePercentage, &device);
        }
    }
}

fn run_once(output_option: OutputOption, device: &nvml_wrapper::device::Device){
    let output_text = match output_option{
        OutputOption::MemFreePercentage => get_free_memory_percentage(&device),
        OutputOption::MemFreeMb => get_free_memory_mb(&device),
        OutputOption::MemUsagePercentage => get_used_memory_percentage(&device),
        OutputOption::MemUsageMb => get_used_memory_mb(&device),
        OutputOption::MemTotalMb => get_total_memory_mb(&device),
        OutputOption::ClockSpeed => get_clock_speed(&device),
        OutputOption::MaxBoostClockSpeed => get_max_boost_clock_speed(&device),
        OutputOption::Temperature => get_temperature(&device),
        OutputOption::GpuUtilization => get_gpu_utilization(&device),
        OutputOption::GpuMemoryUtilization => get_gpu_memory_utilization(&device),
    };
    println!("{}", output_text);
}

fn start_loop(output_option: OutputOption, device: &nvml_wrapper::device::Device){
    loop {
        run_once(output_option, &device);
        sleep(Duration::from_secs(1));
    }
}

fn get_used_memory_percentage(device: &nvml_wrapper::device::Device) -> String{
    let memory_info = match device.memory_info(){
        Ok(memory_info) => memory_info,
        Err(_error) => panic!("Error gathering memory info"),
    };

    let used_memory_percentage = (memory_info.used * 100) / memory_info.total;

    format!("{:?}%", used_memory_percentage)
}

fn get_used_memory_mb(device: &nvml_wrapper::device::Device) -> String{
    let memory_info = match device.memory_info(){
        Ok(memory_info) => memory_info,
        Err(_error) => panic!("Error gathering memory info"),
    };

    let used_memory_mb = memory_info.used / 1024 / 1024;

    format!("{:?} MB", used_memory_mb)
}

fn get_free_memory_percentage(device: &nvml_wrapper::device::Device) -> String{
    let memory_info = match device.memory_info(){
        Ok(memory_info) => memory_info,
        Err(_error) => panic!("Error gathering memory info"),
    };

    let free_memory_percentage = (memory_info.free * 100) / memory_info.total;

    format!("{:?}%", free_memory_percentage)
}

fn get_free_memory_mb(device: &nvml_wrapper::device::Device) -> String{
    let memory_info = match device.memory_info(){
        Ok(memory_info) => memory_info,
        Err(_error) => panic!("Error gathering memory info"),
    };

    let free_memory_mb = memory_info.free / 1024 / 1024;

    format!("{:?} MB", free_memory_mb)
}

fn get_total_memory_mb(device: &nvml_wrapper::device::Device) -> String{
    let memory_info = match device.memory_info(){
        Ok(memory_info) => memory_info,
        Err(_error) => panic!("Error gathering memory info"),
    };

    let total_memory_mb = memory_info.total / 1024 / 1024;

    format!("{:?} MB", total_memory_mb)
}

fn get_clock_speed(device: &nvml_wrapper::device::Device) -> String{
    let clock_speed = match device.clock(Clock::Graphics, ClockId::Current){
        Ok(clock_speed) => clock_speed,
        Err(_error) => panic!("Error gathering clock info"),
    };

    format!("{:?} MHz", clock_speed)
}

fn get_max_boost_clock_speed(device: &nvml_wrapper::device::Device) -> String{
    let clock_speed = match device.clock(Clock::Graphics, ClockId::CustomerMaxBoost){
        Ok(clock_speed) => clock_speed,
        Err(_error) => panic!("Error gathering clock info"),
    };

    format!("{:?} MHz", clock_speed)
}

fn get_temperature(device: &nvml_wrapper::device::Device) -> String{
    let temperature = match device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu){
        Ok(temperature) => temperature,
        Err(_error) => panic!("Error gathering temperature info"),
    };

    format!("{:?} C", temperature)
}

fn get_gpu_utilization(device: &nvml_wrapper::device::Device) -> String{
    let utilization = match device.utilization_rates(){
        Ok(utilization) => utilization,
        Err(_error) => panic!("Error gathering utilization info"),
    };

    format!("{:?}%", utilization.gpu)
}

fn get_gpu_memory_utilization(device: &nvml_wrapper::device::Device) -> String{
    let utilization = match device.utilization_rates(){
        Ok(utilization) => utilization,
        Err(_error) => panic!("Error gathering utilization info"),
    };

    format!("{:?}%", utilization.memory)
}



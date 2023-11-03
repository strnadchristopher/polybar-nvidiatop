use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::Clock;
use nvml_wrapper::enum_wrappers::device::ClockId;
use std::thread::sleep;
use std::time::Duration;
use std::env;
use std::fmt;
#[derive(Debug)]
enum OutputOption{
    MemFreePercentage,
    MemFreeMb,
    MemUsagePercentage,
    MemUsageMb,
    MemTotalMb,
    ClockSpeed,
    MaxBoostClockSpeed,
}
impl fmt::Display for OutputOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if(args.len() > 1){
        let output_option = match args[1].as_str(){
            "--free-memory" => OutputOption::MemFreePercentage,
            "--free-memory-mb" => OutputOption::MemFreeMb,
            "--used-memory" => OutputOption::MemUsagePercentage,
            "--used-memory-mb" => OutputOption::MemUsageMb,
            "--total-memory-mb" => OutputOption::MemTotalMb,
            "--clock-speed" => OutputOption::ClockSpeed,
            "--max-boost-clock-speed" => OutputOption::MaxBoostClockSpeed,
            _ => panic!("Invalid output option"),
        };
        start_loop(output_option);
        // println!("{}", output_option);
    }else{
        start_loop(OutputOption::MemUsagePercentage);
    }
    
}

fn start_loop(output_option: OutputOption){
    let nvml = match Nvml::init(){
        Ok(result) => result,
        Err(error) => panic!("Nvml not installed."),
    };

    let device = match nvml.device_by_index(0){
        Ok(device) => device,
        Err(error) => panic!("Could not find NVIDIA device.")
    };
    loop {
        let output_text = match output_option{
            OutputOption::MemFreePercentage => get_free_memory_percentage(&device),
            OutputOption::MemFreeMb => get_free_memory_mb(&device),
            OutputOption::MemUsagePercentage => get_used_memory_percentage(&device),
            OutputOption::MemUsageMb => get_used_memory_mb(&device),
            OutputOption::MemTotalMb => get_total_memory_mb(&device),
            OutputOption::ClockSpeed => get_clock_speed(&device),
            OutputOption::MaxBoostClockSpeed => get_max_boost_clock_speed(&device),
        };
        println!("{}", output_text);
        sleep(Duration::from_secs(1));
    }
}

fn get_used_memory_percentage(device: &nvml_wrapper::device::Device) -> String{
    let memory_info = match device.memory_info(){
        Ok(memory_info) => memory_info,
        Err(error) => panic!("Error gathering memory info"),
    };

    let used_memory_percentage = (memory_info.used * 100) / memory_info.total;

    format!("{:?}%", used_memory_percentage)
}

fn get_used_memory_mb(device: &nvml_wrapper::device::Device) -> String{
    let memory_info = match device.memory_info(){
        Ok(memory_info) => memory_info,
        Err(error) => panic!("Error gathering memory info"),
    };

    let used_memory_mb = memory_info.used / 1024 / 1024;

    format!("{:?} MB", used_memory_mb)
}

fn get_free_memory_percentage(device: &nvml_wrapper::device::Device) -> String{
    let memory_info = match device.memory_info(){
        Ok(memory_info) => memory_info,
        Err(error) => panic!("Error gathering memory info"),
    };

    let free_memory_percentage = (memory_info.free * 100) / memory_info.total;

    format!("{:?}%", free_memory_percentage)
}

fn get_free_memory_mb(device: &nvml_wrapper::device::Device) -> String{
    let memory_info = match device.memory_info(){
        Ok(memory_info) => memory_info,
        Err(error) => panic!("Error gathering memory info"),
    };

    let free_memory_mb = memory_info.free / 1024 / 1024;

    format!("{:?} MB", free_memory_mb)
}

fn get_total_memory_mb(device: &nvml_wrapper::device::Device) -> String{
    let memory_info = match device.memory_info(){
        Ok(memory_info) => memory_info,
        Err(error) => panic!("Error gathering memory info"),
    };

    let total_memory_mb = memory_info.total / 1024 / 1024;

    format!("{:?} MB", total_memory_mb)
}

fn get_clock_speed(device: &nvml_wrapper::device::Device) -> String{
    let clock_speed = match device.clock(Clock::Graphics, ClockId::Current){
        Ok(clock_speed) => clock_speed,
        Err(error) => panic!("Error gathering clock info"),
    };

    format!("{:?} MHz", clock_speed)
}

fn get_max_boost_clock_speed(device: &nvml_wrapper::device::Device) -> String{
    let clock_speed = match device.clock(Clock::Graphics, ClockId::CustomerMaxBoost){
        Ok(clock_speed) => clock_speed,
        Err(error) => panic!("Error gathering clock info"),
    };

    format!("{:?} MHz", clock_speed)
}




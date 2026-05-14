
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

const PATH: &str = "/sys/class/drm/card0/device/pp_od_clk_voltage";

// OD_VDDC_CURVE:
const VC_1_MHZ: &str = "1417";
const VC_1_MV: &str = "806";

const ECO_VC_2_MHZ: &str = "1700";
const ECO_VC_2_MV: &str = "875";

const LOW_VC_2_MHZ: &str = "1900";
const LOW_VC_2_MV: &str = "985";

const HIGH_VC_2_MHZ: &str = "2000";
const HIGH_VC_2_MV: &str = "1050";

const OC_VC_2_MHZ: &str = "2100";
const OC_VC_2_MV: &str = "1100";

fn set_vc(mode: &str) -> std::io::Result<()> {

    	let mut file = File::create(PATH)?;

	let vc_1: String;
	let vc_2: String;
	let max_freq: &str;

	match mode {

		"eco" => {
			vc_1 = format!("vc 1 {} {}", VC_1_MHZ, VC_1_MV);
			vc_2 = format!("vc 2 {} {}", ECO_VC_2_MHZ, ECO_VC_2_MV);
			max_freq = ECO_VC_2_MHZ;
		},

		"low" => {
			vc_1 = format!("vc 1 {} {}", VC_1_MHZ, VC_1_MV);
			vc_2 = format!("vc 2 {} {}", LOW_VC_2_MHZ, LOW_VC_2_MV);
			max_freq = LOW_VC_2_MHZ;
		},

		"high" => {
			vc_1 = format!("vc 1 {} {}", VC_1_MHZ, VC_1_MV);
			vc_2 = format!("vc 2 {} {}", HIGH_VC_2_MHZ, HIGH_VC_2_MV);
			max_freq = HIGH_VC_2_MHZ;
		},

		"oc" => {
			vc_1 = format!("vc 1 {} {}", VC_1_MHZ, VC_1_MV);
			vc_2 = format!("vc 2 {} {}", OC_VC_2_MHZ, OC_VC_2_MV);
			max_freq = OC_VC_2_MHZ;
		},

		_ => return Ok(()),
	}


	// set VC 1 frequency
	file.write_all(vc_1.as_bytes())?;

	// set VC 2 frequency
	file.write_all(vc_2.as_bytes())?;

	// set max frequency
	let max_clk = format!("s 1 {}", max_freq);
	file.write_all(max_clk.as_bytes())?;

	// update new config
	file.write_all("c".as_bytes())?;

	Ok(())
}

fn print_help() {
		println!("Usage: rx5700xt_ctrl [OPTION]\n
		\n\tOptions:
		\n\toc\t\tset GPU to 2100Mhz 1100mV
		\n\thigh\t\tset GPU to 2000Mhz 1050mV
		\n\tlow\t\tset GPU to 1900Mhz  985mV
		\n\teco\t\tset GPU to 1700MHz  875mV
    	");

}

fn main() -> std::io::Result<()> {

	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		print_help();
		return Ok(())

	} else {

		let arg: &str = &args[1];
		match arg {

			"eco"|"max"|"oc" => set_vc(&args[1])?,

			"info" => {
				let info = fs::read_to_string(PATH).expect("Cannot read file");
				println!("{}", info);
				return Ok(())
			},

			_ => {
				print_help();
				return Ok(())
			},

		}
	}

	Ok(())
}

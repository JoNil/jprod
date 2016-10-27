extern crate gcc;

fn main() {
	#[cfg(all(windows, target_pointer_width = "32"))]
    gcc::Config::new()
            .object("rt/win32_crt_math.o")
            .object("rt/mulodi4.o")
            .compile("librt.a");
}
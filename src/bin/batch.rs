use mappy::MappyState;
use retro_rs::{Buttons, Emulator};
use std::path::Path;
use std::time::Instant;
fn main() {
    use std::env;
    let mut emu = Emulator::create(
        Path::new("../../cores/fceumm_libretro"),
        Path::new("../../roms/mario.nes"),
    );
    // Have to run emu for one frame before we can get the framebuffer size
    emu.run([Buttons::new(), Buttons::new()]);
    let (w, h) = emu.framebuffer_size();
    // So reset it afterwards
    emu.reset();
    let mut inputs = vec![];
    let args:Vec<_> = env::args().collect();
    if args.len() > 1 {
        mappy::read_fm2(&mut inputs, &Path::new(&args[1]));
    }
    let mut mappy = MappyState::new(w, h);
    let start = Instant::now();
    for (i,input_pair) in inputs.iter().enumerate() {
        emu.run(*input_pair);
        mappy.process_screen(&emu);
        if i > 280 {
            println!("Scroll: {:?} : {:?}", mappy.splits, mappy.scroll);
            println!("Known tiles: {:?}", mappy.tiles.gfx_count());
        }
    }
    println!("Emulation only: 0.110773661 for 360 inputs, avg 0.00030770523055555557 per frame");
    println!(
        "Net: {:} for {:} inputs, avg {:} per frame",
        start.elapsed().as_secs_f64(),
        inputs.len(),
        start.elapsed().as_secs_f64() / (inputs.len() as f64)
    );
    mappy.dump_tiles(Path::new("../../out/"));
}
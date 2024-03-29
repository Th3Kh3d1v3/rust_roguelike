use super::{
    AreaEndingPosition, AreaStartingPosition, BspInteriorBuilder, BuilderChain, CullUnreachable,
    VoronoiSpawning, XEnd, XStart, YEnd, YStart,
};

pub fn dark_elf_city(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    println!("Dark elf builder");
    let mut chain = BuilderChain::new(new_depth, width, height, "Dark Elven City");
    chain.start_with(BspInteriorBuilder::new());
    chain.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(AreaStartingPosition::new(XStart::RIGHT, YStart::CENTER));
    chain.with(AreaEndingPosition::new(XEnd::LEFT, YEnd::CENTER));
    chain.with(VoronoiSpawning::new());
    chain
}

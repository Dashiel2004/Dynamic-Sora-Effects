mod aerial;
mod jab;
mod smash;
mod tilt;
mod throw;
mod specialhi;
mod speciallw;
mod specials;

pub fn install(){
    aerial::install();
    jab::install();
    smash::install();
    tilt::install();
    throw::install();
    specialhi::install();
    speciallw::install();
    specials::install();
}

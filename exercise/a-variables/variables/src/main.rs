const STARTING_MISSILES : i32= 8;
const READY : i32= 2;

fn main() {
  let (mut missiles, ready)  = (STARTING_MISSILES, READY);

  println!("Firing {} of {} missiles",ready,missiles);

  // READY = 4;
  // missiles = missiles - ready;

  println!("{} missiles left",missiles - ready)
}

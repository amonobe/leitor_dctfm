use libDCTFM::classe_sped::{Sped, TipoSped};

fn main() {
    let exemplo = Sped::definir(TipoSped::Contribuicoes);

    println!("{}", exemplo);
}

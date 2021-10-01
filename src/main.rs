fn nom_mois(numéro: u8) -> &'static str {
    match numéro {
        1 => "Janvier",
        2 => "Février",
        3 => "Mars",
        4 => "Avril",
        5 => "Mai",
        6 => "Juin",
        7 => "Juillet",
        8 => "Août",
        9 => "Septembre",
        10 => "Octobre",
        11 => "Novembre",
        12 => "Décembre",
        _ => panic!("Numéro de mois invalide !"),
    }
}
fn afficher_titre(numéro: u8, année: u16) {
    let signes_égal = "=".repeat(26);
    let titre = format!("{} {}", nom_mois(numéro), année);
    let marge = " ".repeat((signes_égal.len() - titre.chars().count()) / 2);
    println!("{}\n{}{}\n{}", signes_égal, marge, titre, signes_égal)
}
fn afficher_entête() {
    println!("Lu  Ma  Me  Je  Ve  Sa  Di");
}
fn afficher_mois(décalage: u8, nombre_jours: u8) {
    let cellules = (0..décalage)
        .map(|_| "  ".to_owned())
        .chain((1..=nombre_jours).map(|j| format!("{:02}", j)))
        .collect::<Vec<String>>();
    for chunk in cellules.as_slice().chunks(7) {
        println!("{}", chunk.join("  "));
    }
}
fn est_bissextile(année: u16) -> bool {
    (année % 4 == 0 && année % 100 != 0) || année % 400 == 0
}
fn nombre_jours(mois: u8, année: u16) -> u8 {
    match mois {
        1 => 31,
        2 => {
            if est_bissextile(année) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => panic!("Mois invalide !"),
    }
}
fn numéro_jour(jour: u8, mois: u8, année: u16) -> u8 {
    let q = jour as u16;
    let m = match mois {
        1 | 2 => mois + 12,
        autre => autre,
    } as u16;
    let j = année / 100;
    let k = année % 100;

    ((q + 13 * (m + 1) / 5 + k + k / 4 + j / 4 + 5 * j + 5) % 7) as u8
}
fn main() {
    let mut arguments = std::env::args();
    let numéro_mois: u8 = arguments
        .nth(1)
        .expect("Numéro du mois")
        .parse()
        .expect("Entier invalide");
    let numéro_année: u16 = arguments
        .next()
        .expect("Numéro de l'année")
        .parse()
        .expect("Entier invalide");

    // Titre et Entête
    afficher_titre(numéro_mois, numéro_année);
    afficher_entête();
    afficher_mois(
        numéro_jour(1, numéro_mois, numéro_année),
        nombre_jours(numéro_mois, numéro_année),
    );
}

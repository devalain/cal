fn nom_mois(numéro: u8) -> &'static str {
    const NOMS_MOIS: &[&str] = &[
        "Janvier", "Février", "Mars", "Avril", "Mai", "Juin", "Juillet", 
        "Août", "Septembre", "Octobre", "Novembre", "Décembre",
    ];
    match numéro {
        1..=12 => NOMS_MOIS[numéro as usize - 1],
        _ => panic!("Numéro de mois invalide !"),
    }
}
fn afficher_titre(numéro: u8, année: u16) {
    let signes_égal = "=".repeat(20);
    let titre = format!("{} {}", nom_mois(numéro), année);
    let marge = " ".repeat((signes_égal.len() - titre.chars().count()) / 2);
    println!("{}\n{}{}\n{}", signes_égal, marge, titre, signes_égal)
}
fn afficher_entête() {
    println!("Lu Ma Me Je Ve Sa Di");
}
fn afficher_mois(décalage: u8, nombre_jours: u8, jour_opt: Option<u8>) {
    let cellules = if let Some(jour) = jour_opt {
        (0..décalage)
        .map(|_| "  ".to_owned())
        .chain((1..jour).map(|j| format!("{:02}", j)))
        .chain((jour..=jour).map(|j| format!("\u{001B}[47m\u{001B}[30m{:02}\u{001B}[49m\u{001B}[39m", j)))
        .chain(((jour + 1)..=nombre_jours).map(|j| format!("{:02}", j)))
        .collect::<Vec<String>>()
    } else {
        (0..décalage)
        .map(|_| "  ".to_owned())
        .chain((1..=nombre_jours).map(|j| format!("{:02}", j)))
        .collect::<Vec<String>>()
    };
    for chunk in cellules.as_slice().chunks(7) {
        println!("{}", chunk.join(" "));
    }
}
fn est_bissextile(année: u16) -> bool {
    (année % 4 == 0 && année % 100 != 0) || année % 400 == 0
}
fn nombre_jours(mois: u8, année: u16) -> u8 {
    match mois {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if est_bissextile(année) => 29,
        2 => 28,
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
fn afficher_pied_de_page() {
    println!("{}", "=".repeat(20));
}
fn main() {
    let arguments = std::env::args().collect::<Vec<String>>();
    let (numéro_jour_opt, numéro_mois, numéro_année): (Option<u8>, u8, u16) = match arguments.len() {
        3 => (
            None,
            arguments[1].parse().expect("Entier invalide"),
            arguments[2].parse().expect("Entier invalide"),
        ),
        4 => (
            Some(arguments[1].parse().expect("Entier invalide")),
            arguments[2].parse().expect("Entier invalide"),
            arguments[3].parse().expect("Entier invalide"),
        ),
        _ => {
            use chrono::Datelike;
            let today = ::chrono::Local::today();
            (Some(today.day() as u8), today.month() as u8, today.year() as u16)
        }
    };

    afficher_titre(numéro_mois, numéro_année);
    afficher_entête();
    afficher_mois(
        numéro_jour(1, numéro_mois, numéro_année),
        nombre_jours(numéro_mois, numéro_année),
        numéro_jour_opt
    );
    afficher_pied_de_page();
}

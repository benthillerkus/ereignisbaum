use ereignisbaum::Bruch;

fn main() {
    Spiel::new(5, 1).start()
}

struct Spiel {
    zeit: u8,
    münzen: u8,

    gewonnen: Bruch,
    verloren: Bruch,
}

struct Zustand {
    münzen: u8,
    verbleibend: u8,
    fortschritt: u8,
    wahrscheinlichkeit: Bruch,
}

impl Spiel {
    fn new(gesamtzeit: u8, münzen: u8) -> Self {
        Self {
            zeit: gesamtzeit,
            münzen,
            gewonnen: 0.into(),
            verloren: 0.into(),
        }
    }

    fn unterbaum(
        spiel: &mut Self,
        Zustand {
            münzen,
            verbleibend,
            fortschritt,
            wahrscheinlichkeit,
        }: Zustand,
    ) {
        match (münzen, verbleibend) {
            (4, _) => {
                spiel.gewonnen += wahrscheinlichkeit;
                print!("✔ {}", wahrscheinlichkeit)
            }
            (_, 0) => {
                print!("⏰ {}", wahrscheinlichkeit)
            }
            (0, _) => {
                spiel.verloren += wahrscheinlichkeit;
                print!("❌ {}", wahrscheinlichkeit)
            }
            _ => {
                let indentation = "  ".repeat(fortschritt as usize);
                print!("\n{} |_ Erfolg ", indentation);
                Self::unterbaum(
                    spiel,
                    Zustand {
                        münzen: münzen + 1,
                        verbleibend: verbleibend - 1,
                        fortschritt: fortschritt + 1,
                        wahrscheinlichkeit: wahrscheinlichkeit * Bruch::new(1, 2),
                    },
                );
                print!("\n{} |_ Pech ", indentation);
                Self::unterbaum(
                    spiel,
                    Zustand {
                        münzen: münzen - 1,
                        verbleibend: verbleibend - 1,
                        fortschritt: fortschritt + 1,
                        wahrscheinlichkeit: wahrscheinlichkeit * Bruch::new(1, 2),
                    },
                );
            }
        }
    }

    fn start(mut self) {
        let zustand = Zustand {
            münzen: self.münzen,
            verbleibend: self.zeit,
            fortschritt: 0,
            wahrscheinlichkeit: 1.into(),
        };
        Self::unterbaum(&mut self, zustand);
        
        println!("\n\nGewonnen: {}\nVerloren: {}", self.gewonnen, self.verloren);
    }
}

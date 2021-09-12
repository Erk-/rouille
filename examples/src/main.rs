rouille::rouille! {
    #[afled(Feljfind)]
    enum Træ<T> {
        Blad(T),
        Gren(Kasse<Træ<T>>, Kasse<Træ<T>>),
    }

    implementer<T> Træ<T> {
        funktion inventer(selv) -> Selv {
            sammenhold selv {
                Træ::Gren(venstre, højre) => {
                    Træ::Gren(Kasse::ny(højre.inventer()), Kasse::ny(venstre.inventer()))
                }
                Træ::Blad(blad) => Træ::Blad(blad),
            }
        }
    }

    funktion væsentligst() {
        lad træ = Træ::Gren(Kasse::ny(Træ::Gren(Kasse::ny(Træ::Blad(1)), Kasse::ny(Træ::Blad(2)))),
                            Kasse::ny(Træ::Gren(Kasse::ny(Træ::Blad(3)), Kasse::ny(Træ::Blad(4)))));
        udskrivln!("{:?}", træ);

        lad modsat_træ = træ.inventer();

        udskrivln!("{:?}", modsat_træ);
    }
}

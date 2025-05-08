use std::collections::HashMap;

use structures::desenvolvedor::Desenvolvedor;

mod structures {
    pub mod desenvolvedor;
    pub mod slot_semanal;
}

fn main() {
    let mut desenvolvedores: HashMap<String, Desenvolvedor> = HashMap::new();

    // Adicionando desenvolvedores
    let desenvolvedor1 = Desenvolvedor::new("Alice".to_string());
    let desenvolvedor2 = Desenvolvedor::new("Bob".to_string());

    desenvolvedores.insert(desenvolvedor1.nome.clone(), desenvolvedor1);
    desenvolvedores.insert(desenvolvedor2.nome.clone(), desenvolvedor2);

    // Adicionando slots para Alice
    if let Some(alice) = desenvolvedores.get_mut("Alice") {
        alice.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 1,
            hora_inicio: 9,
            hora_fim: 10,
        });
        alice.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 1,
            hora_inicio: 10,
            hora_fim: 11,
        });
        alice.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 1,
            hora_inicio: 11,
            hora_fim: 12,
        });
        alice.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 1,
            hora_inicio: 12,
            hora_fim: 13,
        });
        alice.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 1,
            hora_inicio: 13,
            hora_fim: 14,
        });
        alice.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 1,
            hora_inicio: 14,
            hora_fim: 15,
        });
        alice.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 2,
            hora_inicio: 9,
            hora_fim: 10,
        });
        alice.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 2,
            hora_inicio: 10,
            hora_fim: 11,
        });
        alice.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 2,
            hora_inicio: 11,
            hora_fim: 12,
        });
    }

    // Adicionando slots para Bob
    if let Some(bob) = desenvolvedores.get_mut("Bob") {
        bob.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 1,
            hora_inicio: 9,
            hora_fim: 10,
        });
        bob.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 1,
            hora_inicio: 10,
            hora_fim: 11,
        });
        bob.adicionar_slot(structures::slot_semanal::SlotSemanal {
            dia_semana: 1,
            hora_inicio: 11,
            hora_fim: 12,
        });
    }

    // Encontra o desenvolvedor com mais horas interruptas, mostre o nome e as horas
    let mut max_horas = 0;
    let mut desenvolvedor_max_horas = String::new();

    let mut max_slots = 0;
    let mut desenvolvedor_max_slots = String::new();
    for (nome, desenvolvedor) in &desenvolvedores {
        let (_, slots) = desenvolvedor.horas_interruptas_trabalhadas();
        if slots > max_slots {
            max_slots = slots;
            desenvolvedor_max_slots = nome.clone();
        }
    }
    for (nome, desenvolvedor) in &desenvolvedores {
        let (horas, _) = desenvolvedor.horas_interruptas_trabalhadas();
        if horas > max_horas {
            max_horas = horas;
            desenvolvedor_max_horas = nome.clone();
        }
    }

    println!(
        "Desenvolvedor com mais horas interruptas: {} com {} horas",
        desenvolvedor_max_horas, max_horas
    );
    println!(
        "Desenvolvedor com mais slots: {} com {} slots",
        desenvolvedor_max_slots, max_slots
    );
}

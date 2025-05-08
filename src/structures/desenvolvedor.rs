use crate::structures::slot_semanal::SlotSemanal;

pub struct Desenvolvedor {
    pub nome: String,
    pub slots_semana: Vec<SlotSemanal>,
}

impl Desenvolvedor {
    pub fn new(nome: String) -> Self {
        Desenvolvedor {
            nome,
            slots_semana: Vec::new(),
        }
    }

    pub fn adicionar_slot(&mut self, slot: SlotSemanal) {
        self.slots_semana.push(slot);
    }

    pub fn horas_interruptas_trabalhadas(&self) -> (u8, usize) {
        let mut maior_num_slots = 0;
        let mut maior_total_horas = 0;

        let mut total_horas = 0;
        let mut num_slots = 0;

        let mut slot_anterior: Option<&SlotSemanal> = None;
        for slot in &self.slots_semana {
            let horas_slot = if slot.hora_fim >= slot.hora_inicio {
                slot.hora_fim - slot.hora_inicio
            } else {
                24 + slot.hora_fim - slot.hora_inicio
            };

            if let Some(prev) = slot_anterior {
                let is_consecutive = (prev.dia_semana == slot.dia_semana
                    && prev.hora_fim == slot.hora_inicio)
                    || (prev.hora_fim == 0 && slot.hora_inicio == 0);

                if is_consecutive {
                    num_slots += 1;
                    total_horas += horas_slot as usize;
                } else {
                    num_slots = 1;
                    total_horas = horas_slot as usize;
                }
            } else {
                num_slots = 1;
                total_horas = horas_slot as usize;
            }

            if num_slots > maior_num_slots {
                maior_num_slots = num_slots;
            }
            if total_horas > maior_total_horas {
                maior_total_horas = total_horas;
            }

            slot_anterior = Some(slot);
        }

        (maior_total_horas as u8, maior_num_slots)
    }
}

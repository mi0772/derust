mod operation;
mod utils;

use crate::operation::OperationType;
use dialoguer::theme::ColorfulTheme;
use dialoguer::MultiSelect;

fn main() {
    println!("Welcome to derust! a free and open-source to keep your system clean.");
    let options = &[
        "Cancellare file inutili",
        "Pulire la cache",
        "Svuotare la cache dei browser",
    ];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Seleziona le azioni da eseguire (usa la barra spaziatrice per selezionare)")
        .items(&options[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("Nessuna azione selezionata.");
        return;
    }

    for selection in selections {
        let mut operation = OperationType::from_selection(selection);
        match operation.execute() {
            Ok(r) => {
                println!("Operazione completata: {} file eliminati, {} megabyte liberati", r.file_count, r.total_size/1024/1024);
            }
            Err(e) => {
                println!("Errore durante l'esecuzione dell'operazione: {}", e);
            }
        }
    }
}


mod operation;

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
        let operation = OperationType::from_selection(selection);
        if let Err(e) = operation.execute() {
            println!("Errore durante l'esecuzione dell'operazione: {}", e);
        }
    }
}


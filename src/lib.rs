use aegis_core::{Value, NativeFn};
use std::collections::HashMap;
use dotenvy::dotenv;

#[unsafe(no_mangle)]
pub extern "C" fn _aegis_register(map: &mut HashMap<String, NativeFn>) {
    map.insert("dotenv_load".to_string(), dotenv_load);
}

fn dotenv_load(_: Vec<Value>) -> Result<Value, String> {
    // dotenv() cherche un fichier .env dans le dossier courant ou les parents
    // et charge les variables dans std::env
    match dotenv() {
        Ok(path) => {
            // On retourne le chemin du fichier trouvé pour info
            Ok(Value::String(path.to_string_lossy().to_string()))
        },
        Err(_) => {
            // Si pas de fichier, ce n'est pas forcément une erreur critique pour le script,
            // mais on peut retourner Null pour dire "rien chargé".
            // Ou on peut propager l'erreur si on veut être strict.
            // Ici, on retourne Null (soft fail).
            Ok(Value::Null)
        }
    }
}

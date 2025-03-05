// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::sync::OnceLock;
use jni::{InitArgsBuilder, JNIVersion, JavaVM};
use jni::objects::JValue;

static JVM_INSTANCE: OnceLock<JavaVM> = OnceLock::new();
static mut CLASSPATH: String = String::new();

fn main() {
    tauri::Builder::default()
        .setup(move |app| unsafe {
            let resource_path = app
                .path_resolver()
                .resolve_resource("src-java")
                .expect("path resolver failed");

            CLASSPATH = resource_path.to_string_lossy().to_string();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![desencriptar])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn desencriptar(valor_encriptado: &str, ambiente_uat: bool, ambiente_prod: bool) -> String {
    if ambiente_uat {
        println!("Ambiente Uat {}", ambiente_uat);
    }
    if ambiente_prod {
        println!("Ambiente prod {}", ambiente_prod);
    }
    calcula_dois_valores(valor_encriptado.to_string()).to_string()
}

fn calcula_dois_valores(valor_encriptado: String) -> i32 {
    let jvm = get_or_create_jvm();

    let mut env = jvm.attach_current_thread().unwrap();

    let class = env.find_class("Calculadora").unwrap();

    let obj = env.alloc_object(class).unwrap();

    let resultado = env
        .call_method(
            obj,
            "calculaDoisValores",
            "(II)I",
            &[JValue::from(valor_encriptado.parse::<i32>().unwrap()), JValue::from(9)]
        );

    resultado.unwrap().try_into().unwrap()
}

fn get_or_create_jvm() -> &'static JavaVM {
    JVM_INSTANCE.get_or_init(|| unsafe {
        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option(format!("-Djava.class.path={}", CLASSPATH))
            .build()
            .unwrap();
        JavaVM::new(jvm_args).unwrap()
    })
}
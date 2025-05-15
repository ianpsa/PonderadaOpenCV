use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use crate::image_process;

pub fn app() -> Element {
    let mut file_path = use_signal(|| None::<String>); // Caminho do arquivo selecionado
    let mut is_selecting_file = use_signal(|| false); // O arquivo está selecionado?
    let mut current_image = use_signal(|| None::<String>); // Imagem original
    let mut processed_image = use_signal(|| None::<String>); // Imagem processada

    rsx! { // Código HTML para a interface (dioxus)
        div {
            style: "min-height: 100vh; background-color: #f3f4f6; padding: 2rem 0;",
            div {
                style: "max-width: 72rem; margin: 0 auto; padding: 0 1rem;",
                div {
                    style: "text-align: center; margin-bottom: 2rem;",
                    h1 { 
                        style: "font-size: 2.25rem; font-weight: bold; color: #1f2937; margin-bottom: 0.5rem;",
                        "Editor de fotos do Ian" 
                    }
                    p {
                        style: "color: #4b5563;",
                        "Selecione uma imagem e aplique filtros para transformá-la"
                    }
                }
                
                div { 
                    style: "background: white; border-radius: 0.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); padding: 1.5rem; margin-bottom: 2rem;",
                    div {
                        style: "display: flex; align-items: center; gap: 1rem;",
                        input {
                            style: "flex: 1; border: 1px solid #d1d5db; border-radius: 0.5rem; padding: 0.75rem; background: #f9fafb; color: #374151;",
                            r#type: "text",
                            value: file_path().unwrap_or_default(), // Caminho do arquivo selecionado
                            readonly: true, 
                            placeholder: "Selecione um arquivo..."
                        }
                        button {
                            style: "background: #3b82f6; color: white; padding: 0.75rem 1.5rem; border-radius: 0.5rem; transition: background 0.2s; cursor: pointer;",
                            disabled: is_selecting_file(),
                            onclick: move |_| { 
                                is_selecting_file.set(true);
                                
                                spawn(async move { // Seleção de arquivos
                                    if let Some(file_handle) = AsyncFileDialog::new().pick_file().await {
                                        let path = file_handle.path().display().to_string();
                                        file_path.set(Some(path.clone()));
                                        current_image.set(Some(path));
                                        processed_image.set(None);
                                    }
                                    is_selecting_file.set(false);
                                }); 
                            },
                            if is_selecting_file() { "Selecionando..." } else { "Selecionar Arquivo" } 
                        }
                    }
                }

                div { 
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 2rem; margin-bottom: 2rem;", // Espaço para as imagens
                    div { 
                        style: "background: white; border-radius: 0.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); overflow: hidden;",
                        div {
                            style: "background: #1f2937; color: white; padding: 0.75rem 1rem;",
                            h2 { style: "font-size: 1.125rem; font-weight: 600;", "Imagem Original" }
                        }
                        div {
                            style: "padding: 1rem; background: #f9fafb; min-height: 300px; display: flex; align-items: center; justify-content: center;",
                            if let Some(path) = current_image() { // Se a imagem original estiver selecionada
                                img { 
                                    src: path, 
                                    style: "width: 100%; height: auto; border-radius: 0.375rem;" 
                                }
                            } else { // Se não houver imagem selecionada
                                div {
                                    style: "color: #9ca3af; text-align: center;",
                                    "Nenhuma imagem selecionada"
                                }
                            }
                        }
                    }
                    div { // Espaço para a imagem processada
                        style: "background: white; border-radius: 0.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); overflow: hidden;",
                        div { 
                            style: "background: #1f2937; color: white; padding: 0.75rem 1rem;",
                            h2 { style: "font-size: 1.125rem; font-weight: 600;", "Imagem Processada" }
                        }
                        div { 
                            style: "padding: 1rem; background: #f9fafb; min-height: 300px; display: flex; align-items: center; justify-content: center;",
                            if let Some(path) = processed_image() {
                                img { 
                                    src: path, 
                                    style: "width: 100%; height: auto; border-radius: 0.375rem;" 
                                }
                            } else { 
                                div {
                                    style: "color: #9ca3af; text-align: center;",
                                    "Aplique um filtro para ver o resultado"
                                }
                            }
                        }
                    }
                }

                div {  // Espaço para os filtros
                    style: "background: white; border-radius: 0.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); padding: 1.5rem;",
                    h3 {
                        style: "font-size: 1.25rem; font-weight: 600; color: #1f2937; margin-bottom: 1rem;",
                        "Filtros disponíveis"
                    }
                    div {  
                        style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem;",
                        button { // Botão para o filtro de escala de cinza
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = image_process::add_filter(path, "grayscale".to_string()); // chama a função de escala de cinza em image_process.rs
                                        processed_image.set(Some(result)); // define a imagem processada como a imagem com o filtro de escala de cinza
                                    });
                                }
                            },
                            "Escala de cinza"
                        }
                        button { // Botão para o filtro de inversão de cores
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = image_process::add_filter(path, "invert".to_string()); // chama a função de inversão de cores em image_process.rs
                                        processed_image.set(Some(result)); // define a imagem processada como a imagem com o filtro de inversão de cores
                                    });
                                }
                            },
                            "Inversão de cores"
                        }
                        button { // Botão para o filtro de aumento de contraste
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = image_process::add_filter(path, "contrast".to_string()); // chama a função de aumento de contraste em image_process.rs
                                        processed_image.set(Some(result)); // define a imagem processada como a imagem com o filtro de aumento de contraste
                                    });
                                }
                            },
                            "Aumento de contraste"
                        }
                        button { // Botão para o filtro de desfoque
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = image_process::add_filter(path, "blur".to_string()); // chama a função de desfoque em image_process.rs
                                        processed_image.set(Some(result)); // define a imagem processada como a imagem com o filtro de desfoque
                                    });
                                }
                            },
                            "Desfoque (blur)"
                        }
                        button { // Botão para o filtro de nitidez
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = image_process::add_filter(path, "sharpen".to_string()); // chama a função de nitidez em image_process.rs
                                        processed_image.set(Some(result)); // define a imagem processada como a imagem com o filtro de nitidez
                                    });
                                }
                            },
                            "Nitidez (sharpen)"
                        }
                        button { // Botão para o filtro de detecção de bordas
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = image_process::add_filter(path, "edges".to_string()); // chama a função de detecção de bordas em image_process.rs
                                        processed_image.set(Some(result)); // define a imagem processada como a imagem com o filtro de detecção de bordas
                                    });
                                }
                            },
                            "Detecção de bordas"
                        }
                        button { // Botão para o filtro de redução de tamanho
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = image_process::add_filter(path, "resize_half".to_string()); // chama a função de redução de tamanho em image_process.rs
                                        processed_image.set(Some(result)); // define a imagem processada como a imagem com o filtro de redução de tamanho
                                    });
                                }
                            },
                            "Redução de Resolução"
                        }
                        button { // Botão para o filtro de rotação de 90 graus no sentido horário
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = image_process::add_filter(path, "rotate_90_cw".to_string()); // chama a função de rotação de 90 graus no sentido horário em image_process.rs
                                        processed_image.set(Some(result)); // define a imagem processada como a imagem com o filtro de rotação de 90 graus no sentido horário
                                    });
                                }
                            },
                            "Rotação de 90 graus no sentido horário"
                        }
                        button { // Botão para o filtro de rotação de 90 graus no sentido anti-horário
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = image_process::add_filter(path, "rotate_90_ccw".to_string()); // chama a função de rotação de 90 graus no sentido anti-horário em image_process.rs
                                        processed_image.set(Some(result)); // define a imagem processada como a imagem com o filtro de rotação de 90 graus no sentido anti-horário
                                    });
                                }
                            },
                            "Rotação de 90 graus no sentido anti-horário"
                        }
                        button { // Botão para o filtro de reset
                            style: "background: linear-gradient(to right, #dc2626, #991b1b); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer; text-align: center; align-items: center; justify-content: center;",
                            onclick: move |_| { 
                                if let Some(path) = current_image() {
                                    processed_image.set(Some(path)); // define a imagem processada como a imagem original
                                }
                            },
                            "Reset"
                        }
                    }
                }
            }
        }
    }
}

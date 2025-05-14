use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use crate::imageProcess;

pub fn app() -> Element {
    let mut file_path = use_signal(|| None::<String>);
    let mut is_selecting_file = use_signal(|| false);
    let mut current_image = use_signal(|| None::<String>);
    let mut processed_image = use_signal(|| None::<String>);
    let mut last_filter = use_signal(|| None::<String>);

    rsx! {
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
                            value: file_path().unwrap_or_default(),
                            readonly: true,
                            placeholder: "Selecione um arquivo..."
                        }
                        button {
                            style: "background: #3b82f6; color: white; padding: 0.75rem 1.5rem; border-radius: 0.5rem; transition: background 0.2s; cursor: pointer;",
                            disabled: is_selecting_file(),
                            onclick: move |_| {
                                is_selecting_file.set(true);
                                
                                spawn(async move {
                                    if let Some(file_handle) = AsyncFileDialog::new().pick_file().await {
                                        let path = file_handle.path().display().to_string();
                                        file_path.set(Some(path.clone()));
                                        current_image.set(Some(path));
                                        processed_image.set(None);
                                        last_filter.set(None);
                                    }
                                    is_selecting_file.set(false);
                                });
                            },
                            if is_selecting_file() { "Selecionando..." } else { "Selecionar Arquivo" }
                        }
                    }
                }

                div { 
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 2rem; margin-bottom: 2rem;",
                    div { 
                        style: "background: white; border-radius: 0.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); overflow: hidden;",
                        div {
                            style: "background: #1f2937; color: white; padding: 0.75rem 1rem;",
                            h2 { style: "font-size: 1.125rem; font-weight: 600;", "Imagem Original" }
                        }
                        div {
                            style: "padding: 1rem; background: #f9fafb; min-height: 300px; display: flex; align-items: center; justify-content: center;",
                            if let Some(path) = current_image() {
                                img { 
                                    src: path, 
                                    style: "width: 100%; height: auto; border-radius: 0.375rem;" 
                                }
                            } else {
                                div {
                                    style: "color: #9ca3af; text-align: center;",
                                    "Nenhuma imagem selecionada"
                                }
                            }
                        }
                    }
                    div { 
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

                div { 
                    style: "background: white; border-radius: 0.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); padding: 1.5rem;",
                    h3 {
                        style: "font-size: 1.25rem; font-weight: 600; color: #1f2937; margin-bottom: 1rem;",
                        "Filtros disponíveis"
                    }
                    div { 
                        style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem;",
                        button {
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() && last_filter() == Some("grayscale".to_string()) {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = imageProcess::add_filter(path, "grayscale".to_string());
                                        processed_image.set(Some(result));
                                        last_filter.set(Some("grayscale".to_string()));
                                    });
                                }
                            },
                            "Escala de cinza"
                        }
                        button {
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() && last_filter() == Some("invert".to_string()) {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = imageProcess::add_filter(path, "invert".to_string());
                                        processed_image.set(Some(result));
                                        last_filter.set(Some("invert".to_string()));
                                    });
                                }
                            },
                            "Inversão de cores"
                        }
                        button {
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() && last_filter() == Some("contrast".to_string()) {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = imageProcess::add_filter(path, "contrast".to_string());
                                        processed_image.set(Some(result));
                                        last_filter.set(Some("contrast".to_string()));
                                    });
                                }
                            },
                            "Aumento de contraste"
                        }
                        button {
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() && last_filter() == Some("blur".to_string()) {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = imageProcess::add_filter(path, "blur".to_string());
                                        processed_image.set(Some(result));
                                        last_filter.set(Some("blur".to_string()));
                                    });
                                }
                            },
                            "Desfoque (blur)"
                        }
                        button {
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() && last_filter() == Some("sharpen".to_string()) {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = imageProcess::add_filter(path, "sharpen".to_string());
                                        processed_image.set(Some(result));
                                        last_filter.set(Some("sharpen".to_string()));
                                    });
                                }
                            },
                            "Nitidez (sharpen)"
                        }
                        button {
                            style: "background: linear-gradient(to right, #374151, #1f2937); color: white; padding: 0.75rem 1rem; border-radius: 0.5rem; transition: transform 0.2s; box-shadow: 0 1px 3px rgba(0,0,0,0.1); cursor: pointer;",
                            onclick: move |_| {
                                if let Some(path) = if processed_image().is_some() && last_filter() == Some("edges".to_string()) {
                                    processed_image()
                                } else {
                                    current_image()
                                } {
                                    spawn(async move {
                                        let result = imageProcess::add_filter(path, "edges".to_string());
                                        processed_image.set(Some(result));
                                        last_filter.set(Some("edges".to_string()));
                                    });
                                }
                            },
                            "Detecção de bordas"
                        }
                    }
                }
            }
        }
    }
}

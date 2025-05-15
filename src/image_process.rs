use opencv::{ // Importação de bibliotecas
    core,
    imgcodecs,
    imgproc,
    prelude::*,
    core::AlgorithmHint,
};

pub fn add_filter(input_image_path: String, filter_type: String) -> String { // Função para adicionar um filtro à imagem
    let img = match imgcodecs::imread(&input_image_path, imgcodecs::IMREAD_COLOR) { // Carrega a imagem
        Ok(mat) if !mat.empty() => mat,
        Ok(_) => { // Caso a imagem esteja vazia
            eprintln!("Erro: Imagem carregada de {} está vazia.", input_image_path);
            return input_image_path; 
        }
        Err(e) => { // Caso ocorra um erro ao carregar a imagem (Requisito da ponderada)
            eprintln!("Erro ao carregar a imagem de {}: {}", input_image_path, e);
            return input_image_path;
        }
    };

    let mut result = Mat::default(); // Cria uma matriz para armazenar o resultado

    match filter_type.as_str() {
        "grayscale" => { // Escala de cinza
            imgproc::cvt_color(&img, &mut result, imgproc::COLOR_BGR2GRAY, 0, AlgorithmHint::ALGO_HINT_DEFAULT).unwrap();
        }
        "invert" => { // Inverte as cores
            core::bitwise_not(&img, &mut result, &Mat::default()).unwrap();
        }
        "contrast" => { // Aumenta o contraste
            img.convert_to(&mut result, -1, 1.5, 0.0).unwrap();
        }
        "blur" => { // Desfoca a imagem
            imgproc::gaussian_blur(&img, &mut result, core::Size::new(5, 5), 0.0, 0.0, core::BORDER_DEFAULT, AlgorithmHint::ALGO_HINT_DEFAULT).unwrap();
        }
        "sharpen" => { // Aumenta a nitidez 
            let kernel = Mat::from_slice_2d(&[
                &[0.0, -1.0, 0.0],
                &[-1.0, 5.0, -1.0],
                &[0.0, -1.0, 0.0],
            ]).unwrap();
            imgproc::filter_2d(&img, &mut result, -1, &kernel, core::Point::new(-1, -1), 0.0, core::BORDER_DEFAULT).unwrap();
        }
        "edges" => { // Detecta as bordas
            let mut gray = Mat::default();
            imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0, AlgorithmHint::ALGO_HINT_DEFAULT).unwrap();
            imgproc::canny(&gray, &mut result, 100.0, 200.0, 3, false).unwrap();
        }
        _ => { // opção inválida
            result = img.clone();
        }
    }


    // Cria o nome do arquivo de saída (ainda não há exportação de arquivos)
    let timestamp = std::time::SystemTime::now() 
        .duration_since(std::time::UNIX_EPOCH)
        .map_or_else(|_| 0, |d| d.as_millis());

    let original_path_obj = std::path::Path::new(&input_image_path);
    
    let parent_dir = original_path_obj
        .parent()
        .and_then(std::path::Path::to_str)
        .unwrap_or(".");

    let file_stem = original_path_obj
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("image")
        .to_string()
        .split('_') 
        .next()
        .unwrap_or("image")
        .to_string();
    
    let output_path = format!(
        "{}/{}_{}_{}_processed.jpg",
        parent_dir,
        file_stem,
        filter_type,
        timestamp
    );
        
    if let Err(e) = imgcodecs::imwrite(&output_path, &result, &core::Vector::new()) {
        eprintln!("Erro ao salvar a imagem processada em {}: {}", output_path, e);
        return input_image_path;
    }

    output_path
}
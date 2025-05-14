use opencv::{
    core,
    imgcodecs,
    imgproc,
    prelude::*,
    core::AlgorithmHint, // Importar AlgorithmHint
};

pub fn add_filter(input_image_path: String, filter_type: String) -> String {
    // Carregar a imagem diretamente do input_image_path
    let img = match imgcodecs::imread(&input_image_path, imgcodecs::IMREAD_COLOR) {
        Ok(mat) if !mat.empty() => mat,
        Ok(_) => {
            eprintln!("Erro: Imagem carregada de {} está vazia.", input_image_path);
            return input_image_path; 
        }
        Err(e) => {
            eprintln!("Erro ao carregar a imagem de {}: {}", input_image_path, e);
            return input_image_path;
        }
    };

    let mut result = Mat::default();

    // Verifica se o caminho da imagem já contém o filtro atual
    let is_reapplying_filter = input_image_path.contains(&format!("_{}_", filter_type));

    // Se estiver reaplicando o mesmo filtro, usa a imagem processada como entrada
    let source_img = if is_reapplying_filter {
        img
    } else {
        // Caso contrário, tenta carregar a imagem original
        let original_path = input_image_path.split("_processed").next().unwrap_or(&input_image_path);
        match imgcodecs::imread(original_path, imgcodecs::IMREAD_COLOR) {
            Ok(mat) if !mat.empty() => mat,
            _ => img // Se falhar, usa a imagem atual
        }
    };

    match filter_type.as_str() {
        "grayscale" => {
            if source_img.channels() == 1 {
                result = source_img.clone();
            } else {
                imgproc::cvt_color(&source_img, &mut result, imgproc::COLOR_BGR2GRAY, 0, AlgorithmHint::ALGO_HINT_DEFAULT).unwrap();
            }
        }
        "invert" => {
            core::bitwise_not(&source_img, &mut result, &Mat::default()).unwrap();
        }
        "contrast" => {
            source_img.convert_to(&mut result, -1, 1.5, 0.0).unwrap();
        }
        "blur" => {
            imgproc::gaussian_blur(&source_img, &mut result, core::Size::new(5, 5), 0.0, 0.0, core::BORDER_DEFAULT, AlgorithmHint::ALGO_HINT_DEFAULT).unwrap();
        }
        "sharpen" => {
            let kernel = Mat::from_slice_2d(&[
                &[0.0, -1.0, 0.0],
                &[-1.0, 5.0, -1.0],
                &[0.0, -1.0, 0.0],
            ]).unwrap();
            imgproc::filter_2d(&source_img, &mut result, -1, &kernel, core::Point::new(-1, -1), 0.0, core::BORDER_DEFAULT).unwrap();
        }
        "edges" => {
            let mut gray = Mat::default();
            if source_img.channels() == 1 {
                gray = source_img.clone();
            } else {
                imgproc::cvt_color(&source_img, &mut gray, imgproc::COLOR_BGR2GRAY, 0, AlgorithmHint::ALGO_HINT_DEFAULT).unwrap();
            }
            imgproc::canny(&gray, &mut result, 100.0, 200.0, 3, false).unwrap();
        }
        "sepia" => {
            let mut source_img_for_sepia;
            if source_img.channels() == 1 {
                let mut temp_gray_to_bgr = Mat::default();
                imgproc::cvt_color(&source_img, &mut temp_gray_to_bgr, imgproc::COLOR_GRAY2BGR, 0, AlgorithmHint::ALGO_HINT_DEFAULT).unwrap();
                source_img_for_sepia = temp_gray_to_bgr;
            } else {
                source_img_for_sepia = source_img.clone();
            }

            let kernel_values: [[f32; 3]; 3] = [
                [0.272, 0.534, 0.131],
                [0.349, 0.686, 0.168],
                [0.393, 0.769, 0.189],
            ];
            let kernel_slices: [&[f32]; 3] = [
                &kernel_values[0],
                &kernel_values[1],
                &kernel_values[2],
            ];
            
            let sepia_kernel = Mat::from_slice_2d(&kernel_slices).unwrap();

            let mut source_32f = Mat::default();
            source_img_for_sepia.convert_to(&mut source_32f, opencv::core::CV_32F, 1.0/255.0, 0.0).unwrap();

            let mut transformed_result_32f = Mat::default();
            core::transform(&source_32f, &mut transformed_result_32f, &sepia_kernel).unwrap();
            
            transformed_result_32f.convert_to(&mut result, opencv::core::CV_8U, 255.0, 0.0).unwrap();
        }
        _ => {
            result = source_img.clone();
        }
    }

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
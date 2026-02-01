//! ML-NER Auto-Download Example
//!
//! This example demonstrates how to use the auto-download feature to
//! automatically fetch pre-trained NER models from HuggingFace Hub.
//!
//! The first run will download the model (~65MB for DistilBertQuantized),
//! and subsequent runs will load from the local cache.
//!
//! Run with: cargo run --example ml_ner_download --features ml-ner-download
//!
//! Note: Requires ONNX Runtime to be installed. Set ORT_DYLIB_PATH environment
//! variable to point to your ONNX Runtime library, or install it system-wide.

#[cfg(feature = "ml-ner-download")]
use spatial_narrative::text::{
    cache_size_bytes, is_model_cached, model_cache_dir, model_cache_path, MlNerModel, NerModel,
};

#[cfg(feature = "ml-ner-download")]
fn main() {
    println!("=== ML-NER Auto-Download Demo ===\n");

    // Show available models
    println!("Available models:");
    println!("  - DistilBertQuantized (~65MB)  - Best balance of size/speed/accuracy");
    println!("  - DistilBert (~250MB)          - Slightly more accurate");
    println!("  - BertBase (~400MB)            - Higher accuracy");
    println!("  - BertLarge (~1.2GB)           - Best accuracy");
    println!("  - Multilingual (~700MB)        - 40+ languages");
    println!();

    // Check cache status
    let model = NerModel::DistilBertQuantized;
    println!("Cache directory: {:?}", model_cache_dir());
    println!("Model cached: {}", is_model_cached(&model));

    if let Ok(size) = cache_size_bytes() {
        println!("Total cache size: {:.2} MB", size as f64 / 1024.0 / 1024.0);
    }
    println!();

    // Check if ONNX Runtime is available
    let ort_path = std::env::var("ORT_DYLIB_PATH").ok();
    if ort_path.is_none() {
        println!("Note: ORT_DYLIB_PATH not set. Will try system ONNX Runtime...");
        println!();
    }

    // Download/load the model
    println!("Loading {} ...", model);
    println!("(First run will download from HuggingFace Hub)\n");

    // Use catch_unwind to handle ONNX Runtime loading panics
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        MlNerModel::download_blocking_with_progress(model.clone(), |downloaded, total| {
            if total > 0 {
                let pct = (downloaded as f64 / total as f64) * 100.0;
                print!("\rDownloading: {:.1}%", pct);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
        })
    }));

    let ml_model = match result {
        Ok(Ok(m)) => m,
        Ok(Err(e)) => {
            eprintln!("\nError: {}", e);
            return;
        },
        Err(_panic) => {
            // Panic occurred - likely ONNX Runtime not found
            println!("\n");
            println!("=== ONNX Runtime Required ===");
            println!();
            println!("The model files were downloaded successfully to:");
            println!("  {:?}", model_cache_path(&model));
            println!();
            println!("However, ONNX Runtime is required to run inference.");
            println!();
            println!("To install ONNX Runtime:");
            println!();
            println!("  Option 1: Download from GitHub");
            println!("    1. Download from: https://github.com/microsoft/onnxruntime/releases");
            println!("    2. Extract and set the environment variable:");
            println!("       export ORT_DYLIB_PATH=/path/to/libonnxruntime.dylib");
            println!();
            println!("  Option 2: macOS with Homebrew");
            println!("    brew install onnxruntime");
            println!(
                "    export ORT_DYLIB_PATH=$(brew --prefix onnxruntime)/lib/libonnxruntime.dylib"
            );
            println!();
            println!("  Option 3: Linux");
            println!("    # Ubuntu/Debian");
            println!("    sudo apt install libonnxruntime");
            println!("    export ORT_DYLIB_PATH=/usr/lib/libonnxruntime.so");
            println!();
            return;
        },
    };

    println!("\nModel loaded successfully!\n");

    // Test extraction
    let texts = [
        "Dr. Sarah Chen presented her findings in Paris on March 15, 2024.",
        "Apple Inc. announced new products at their headquarters in Cupertino.",
        "The earthquake struck Tokyo at 3:15 AM, affecting millions of residents.",
        "President Biden met with Chancellor Scholz in Berlin to discuss NATO.",
    ];

    println!("--- Entity Extraction Results ---\n");

    for text in texts {
        println!("Text: \"{}\"", text);

        match ml_model.extract(text) {
            Ok(entities) => {
                if entities.is_empty() {
                    println!("  No entities found.\n");
                } else {
                    for entity in entities {
                        println!(
                            "  [{:4}] \"{}\" (confidence: {:.2})",
                            entity.label, entity.text, entity.score
                        );
                    }
                    println!();
                }
            },
            Err(e) => {
                println!("  Error: {}\n", e);
            },
        }
    }

    // Show how to convert to standard Entity type
    println!("--- Integration with Geoparsing ---\n");

    let text = "The summit was held in Geneva, Switzerland.";
    if let Ok(entities) = ml_model.extract(text) {
        for ml_entity in entities {
            let entity = ml_entity.to_entity();
            println!(
                "ML Entity: \"{}\" ({:?}) -> Can be used with gazetteer lookup",
                entity.text, entity.entity_type
            );
        }
    }
}

#[cfg(not(feature = "ml-ner-download"))]
fn main() {
    eprintln!("This example requires the 'ml-ner-download' feature.");
    eprintln!("Run with: cargo run --example ml_ner_download --features ml-ner-download");
}

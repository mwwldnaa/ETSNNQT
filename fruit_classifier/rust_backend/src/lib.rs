use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use ndarray::{Array2, Array1, Axis};
use rand::Rng;
use ndarray::s;

#[repr(C)]
pub struct TrainingResult {
    accuracies: *mut f64,
    losses: *mut f64,
    final_accuracy: f64,
    length: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn train_network(
    accuracies: *mut *mut f64,
    losses: *mut *mut f64,
    final_accuracy: *mut f64,
    length: *mut usize,
) {
    // Simulate training process
    let epochs = 500;
    let mut acc = Vec::with_capacity(epochs);
    let mut loss = Vec::with_capacity(epochs);
    
    for i in 0..epochs {
        acc.push(i as f64 / epochs as f64);
        loss.push(1.0 - (i as f64 / epochs as f64));
    }

    // Convert Vec to boxed slices and get raw pointers
    let boxed_acc = acc.into_boxed_slice();
    let boxed_loss = loss.into_boxed_slice();
    
    let len = boxed_acc.len();
    let acc_ptr = Box::into_raw(boxed_acc) as *mut f64;
    let loss_ptr = Box::into_raw(boxed_loss) as *mut f64;
    
    unsafe {
        *accuracies = acc_ptr;
        *losses = loss_ptr;
        *final_accuracy = 0.95; // Example accuracy
        *length = len;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn predict(features: *const f64, feature_len: usize) -> *mut c_char {
    let _features_slice = unsafe { std::slice::from_raw_parts(features, feature_len) };
    
    // Simple prediction logic
    let prediction = if feature_len >= 1 && unsafe { *features } > 150.0 {
        "apple"
    } else if feature_len >= 2 && unsafe { *features.add(1) } > 7.0 {
        "orange"
    } else {
        "unknown"
    };
    
    CString::new(prediction).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn free_array(ptr: *mut f64) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(ptr, 0));
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[derive(Debug, Clone)]
pub struct NeuralNet {
    weights1: Array2<f64>,
    bias1: Array1<f64>,
    weights2: Array2<f64>,
    bias2: Array1<f64>,
    learning_rate: f64,
    pub accuracies: Vec<f64>,
    pub losses: Vec<f64>,
}

impl NeuralNet {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize, learning_rate: f64) -> Self {
        let mut rng = rand::thread_rng();
        
        let weights1 = Array2::from_shape_fn((input_size, hidden_size), |_| {
            rng.gen_range(-1.0..1.0) * (2.0 / (input_size + hidden_size) as f64).sqrt()
        });
        
        let bias1 = Array1::zeros(hidden_size);
        
        let weights2 = Array2::from_shape_fn((hidden_size, output_size), |_| {
            rng.gen_range(-1.0..1.0) * (1.0 / (hidden_size + output_size) as f64).sqrt()
        });
        
        let bias2 = Array1::zeros(output_size);
        
        NeuralNet {
            weights1,
            bias1,
            weights2,
            bias2,
            learning_rate,
            accuracies: Vec::new(),
            losses: Vec::new(),
        }
    }
}

pub fn train_model() -> TrainingResult {
    // Simulated training result
    let accuracies = vec![0.1, 0.2, 0.3];
    let losses = vec![0.9, 0.8, 0.7];
    
    let boxed_acc = accuracies.into_boxed_slice();
    let boxed_loss = losses.into_boxed_slice();
    
    TrainingResult {
        accuracies: Box::into_raw(boxed_acc) as *mut f64,
        losses: Box::into_raw(boxed_loss) as *mut f64,
        final_accuracy: 0.95,
        length: 3,
    }
}
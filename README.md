# ETSNNQT
Klasifikasi Buah Menggunakan Metode Artificial Neural Network dan Pemrograman Bahasa Rust

Fruit_classifier merupakan klasifikasi jenis buah menggunakan metode Artificial Neural Network dengan Bahasa Pemrograman Rust berdasarkan dataset dengan 4 input diantaranya weight, size, height, widht. Digunakan 16 neuron pada Hidden Layer dan menghasilkan 1 output yakni jenis buah yang diprediksi. Selain hasil prediksi, dihasilkan juga grafik prediksi yang ditampilkan pada vscode dan QT Framework, dimana pada QT ini digunakan sebagai dashboard untuk menampilkan hasil training neural network. 

------------------------------------------------------------------

# Fruit Classification with Neural Network

**Proyek ini mengimplementasikan neural network dari scratch untuk klasifikasi buah berdasarkan fitur fisik seperti berat, ukuran, lebar, dan tinggi.**

## Fitur Utama

- **Preprocessing Data**:
  - Load dataset CSV ke dalam struktur data Rust
  - Normalisasi fitur menggunakan z-score
  - Encoding label one-hot

- **Arsitektur Neural Network**:
  - Fully connected network dengan 16 hidden layer
  - Aktivasi ReLU untuk hidden layer
  - Softmax untuk output layer
  - Loss function: Cross-entropy
  - Optimizer: SGD dengan L2 regularization

- **Fitur Tambahan**:
  - Pelatihan dengan mini-batch
  - Tracking akurasi dan loss selama training
  - Visualisasi metrik training
  - Mode prediksi interaktif

## Struktur Kode

```
src/
├── data.rs       # Loading dan preprocessing data
├── model.rs      # Implementasi neural network
├── training.rs   # Logika pelatihan model
├── utils.rs      # Fungsi utilitas (normalisasi, encoding)
└── main.rs       # CLI untuk training dan prediksi
```

## Cara Menggunakan

1. **Training Model**:
   ```bash
   cargo run --release
   ```

2. **Prediksi Interaktif**:
   - Setelah training, masukkan pengukuran buah dalam format:
     ```
     weight(g) size(cm) width(cm) height(cm)
     ```
   - Contoh: `150 7 6 6`

3. **Visualisasi**:
   - Plot training otomatis tersimpan sebagai `training_plots.png`
4. **Integrasi dengan Rust**
Aplikasi berkomunikasi dengan library Rust melalui FFI (Foreign Function Interface) untuk:
- train_network: Melatih model neural network
- predict: Memprediksi jenis buah berdasarkan input

## Requirement

- Rust 1.65+
- Cargo
- Dataset CSV dengan kolom: weight, size, width, height, label

## Contoh Dataset

```
weight,size,width,height,label
150,7.3,6.1,6.0,apple
120,6.8,5.8,5.7,apple
130,7.0,5.5,5.5,orange
```



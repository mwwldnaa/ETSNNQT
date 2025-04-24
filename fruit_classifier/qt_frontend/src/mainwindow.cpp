#include "mainwindow.h"
#include "plotwidget.h"
#include <QCoreApplication>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QPushButton>
#include <QLabel>
#include <QLineEdit>
#include <QMessageBox>
#include <QWidget>

// External C functions from Rust
extern "C" {
    void train_network(double**, double**, double*, size_t*);
    void free_array(double* ptr);
    char* predict(double* features, size_t feature_len);
    void free_string(char* ptr);
}

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent)
{
    QWidget *centralWidget = new QWidget(this);
    this->setCentralWidget(centralWidget);
    
    // Main vertical layout
    mainLayout = new QVBoxLayout(centralWidget);
    
    // Train button
    trainButton = new QPushButton("Train Model", centralWidget);
    mainLayout->addWidget(trainButton);
    
    // Accuracy label
    accuracyLabel = new QLabel("Accuracy: N/A", centralWidget);
    accuracyLabel->setAlignment(Qt::AlignCenter);
    mainLayout->addWidget(accuracyLabel);
    
    // Plot widgets
    accuracyPlot = new PlotWidget(centralWidget);
    lossPlot = new PlotWidget(centralWidget);
    
    mainLayout->addWidget(new QLabel("Accuracy Progress:", centralWidget));
    mainLayout->addWidget(accuracyPlot);
    mainLayout->addWidget(new QLabel("Loss Progress:", centralWidget));
    mainLayout->addWidget(lossPlot);
    
    // Prediction section
    mainLayout->addWidget(new QLabel("Input Features for Prediction:", centralWidget));
    
    // Input fields
    QHBoxLayout *inputLayout = new QHBoxLayout();
    const QStringList featureNames = {"Weight (g)", "Size (cm)", "Width (cm)", "Height (cm)"};
    for (const auto& name : featureNames) {
        QLineEdit *input = new QLineEdit(centralWidget);
        input->setPlaceholderText(name);
        inputLayout->addWidget(input);
        featureInputs.append(input);
    }
    mainLayout->addLayout(inputLayout);
    
    // Predict button
    QPushButton *predictButton = new QPushButton("Predict Fruit", centralWidget);
    mainLayout->addWidget(predictButton);
    
    // Prediction result
    predictionLabel = new QLabel("Prediction Result: N/A", centralWidget);
    predictionLabel->setAlignment(Qt::AlignCenter);
    predictionLabel->setStyleSheet("font-weight: bold; font-size: 16px;");
    mainLayout->addWidget(predictionLabel);
    
    // Connect signals
    connect(trainButton, &QPushButton::clicked, this, &MainWindow::trainModel);
    connect(predictButton, &QPushButton::clicked, this, &MainWindow::predictFruit);
}

MainWindow::~MainWindow()
{
    // Qt's parent-child system handles cleanup
}

void MainWindow::trainModel()
{
    trainButton->setEnabled(false);
    trainButton->setText("Training...");
    QCoreApplication::processEvents();

    double* accuracies = nullptr;
    double* losses = nullptr;
    double finalAccuracy = 0;
    size_t length = 0;

    // Call Rust training function
    train_network(&accuracies, &losses, &finalAccuracy, &length);

    if (length == 0 || !accuracies || !losses) {
        QMessageBox::critical(this, "Error", "Training failed or returned empty data.");
        trainButton->setEnabled(true);
        trainButton->setText("Train Model");
        return;
    }

    // Update UI with training results
    accuracyLabel->setText(QString("Accuracy: %1%").arg(finalAccuracy * 100, 0, 'f', 2));

    QVector<double> accVec;
    QVector<double> lossVec;
    accVec.reserve(length);
    lossVec.reserve(length);

    for (size_t i = 0; i < length; ++i) {
        accVec.push_back(accuracies[i]);
        lossVec.push_back(losses[i]);
    }

    accuracyPlot->setData(accVec);
    lossPlot->setData(lossVec);

    // Free Rust-allocated memory
    if (accuracies) free_array(accuracies);
    if (losses) free_array(losses);

    trainButton->setEnabled(true);
    trainButton->setText("Train Model");
}

void MainWindow::predictFruit()
{
    QVector<double> features;
    bool allValid = true;
    
    // Validate all inputs
    for (auto *input : featureInputs) {
        bool ok;
        double val = input->text().toDouble(&ok);
        if (!ok) {
            allValid = false;
            break;
        }
        features.append(val);
    }

    if (!allValid || features.size() != 4) {
        predictionLabel->setText("Please enter valid numbers for all features");
        predictionLabel->setStyleSheet("color: red;");
        return;
    }

    // Call Rust prediction function
    char* result = predict(features.data(), features.size());
    QString prediction = QString::fromUtf8(result);
    free_string(result);

    // Display result
    predictionLabel->setText(QString("Prediction: %1").arg(prediction));
    predictionLabel->setStyleSheet("color: green;");
}
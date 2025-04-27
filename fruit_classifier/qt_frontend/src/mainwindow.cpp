#include "mainwindow.h"
#include "plotwidget.h"

#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QPushButton>
#include <QLabel>
#include <QLineEdit>
#include <QMessageBox>
#include <QGroupBox>
#include <QDoubleValidator>
#include <QCoreApplication>

#ifdef _WIN32
    #define RUST_IMPORT __declspec(dllimport)
#else
    #define RUST_IMPORT
#endif

extern "C" {
    RUST_IMPORT void train_network(double** accuracies, double** losses, double* final_accuracy, size_t* length);
    RUST_IMPORT void free_array(double* ptr);
    RUST_IMPORT char* predict(double* features, size_t feature_len);
    RUST_IMPORT void free_string(char* ptr);
}

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent)
{
    QWidget *centralWidget = new QWidget(this);
    setCentralWidget(centralWidget);
    
    mainLayout = new QVBoxLayout(centralWidget);
    mainLayout->setSpacing(10);
    mainLayout->setContentsMargins(10, 10, 10, 10);

    trainButton = new QPushButton("Train Model", centralWidget);
    mainLayout->addWidget(trainButton);
    
    accuracyLabel = new QLabel("Accuracy: N/A", centralWidget);
    accuracyLabel->setAlignment(Qt::AlignCenter);
    mainLayout->addWidget(accuracyLabel);
    
    accuracyPlot = new PlotWidget(centralWidget);
    accuracyPlot->setMinimumSize(400, 200);
    mainLayout->addWidget(accuracyPlot);
    
    lossPlot = new PlotWidget(centralWidget);
    lossPlot->setMinimumSize(400, 200);
    mainLayout->addWidget(lossPlot);
    
    QGroupBox *predictionGroup = new QGroupBox("Prediction", centralWidget);
    QVBoxLayout *predictionLayout = new QVBoxLayout(predictionGroup);
    
    QHBoxLayout *inputLayout = new QHBoxLayout();
    const QStringList featureNames = {"Weight (g)", "Size (cm)", "Width (cm)", "Height (cm)"};
    for (const auto& name : featureNames) {
        QLineEdit *input = new QLineEdit(centralWidget);
        input->setPlaceholderText(name);
        input->setValidator(new QDoubleValidator(0.1, 10000.0, 2, input));
        inputLayout->addWidget(input);
        featureInputs.append(input);
    }
    predictionLayout->addLayout(inputLayout);
    
    QPushButton *predictButton = new QPushButton("Predict Fruit", centralWidget);
    predictionLayout->addWidget(predictButton);
    
    predictionLabel = new QLabel("Prediction Result: N/A", centralWidget);
    predictionLabel->setAlignment(Qt::AlignCenter);
    predictionLayout->addWidget(predictionLabel);
    
    mainLayout->addWidget(predictionGroup);
    
    connect(trainButton, &QPushButton::clicked, this, &MainWindow::trainModel);
    connect(predictButton, &QPushButton::clicked, this, &MainWindow::predictFruit);
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

    train_network(&accuracies, &losses, &finalAccuracy, &length);

    if (length == 0 || !accuracies || !losses) {
        QMessageBox::critical(this, "Error", "Training failed or returned empty data.");
        trainButton->setEnabled(true);
        trainButton->setText("Train Model");
        return;
    }

    accuracyLabel->setText(QString("Final Accuracy: %1%").arg(finalAccuracy * 100, 0, 'f', 2));

    QVector<double> accVec, lossVec;
    accVec.reserve(length);
    lossVec.reserve(length);

    for (size_t i = 0; i < length; ++i) {
        accVec.push_back(accuracies[i]);
        lossVec.push_back(losses[i]);
    }

    accuracyPlot->setData(accVec);
    accuracyPlot->setPlotColor(Qt::blue);
    accuracyPlot->setYRange(0.0, 1.0);
    accuracyPlot->setTitle("Accuracy Progress");
    
    lossPlot->setData(lossVec);
    lossPlot->setPlotColor(Qt::red);
    lossPlot->setYRange(0.0, 1.0);
    lossPlot->setTitle("Loss Progress");

    if (accuracies) free_array(accuracies);
    if (losses) free_array(losses);

    trainButton->setEnabled(true);
    trainButton->setText("Train Model");
}

void MainWindow::predictFruit()
{
    QVector<double> features;
    bool allValid = true;
    
    for (auto *input : featureInputs) {
        bool ok;
        double val = input->text().toDouble(&ok);
        if (!ok || val <= 0) {
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

    char* result = predict(features.data(), features.size());
    QString prediction = QString::fromUtf8(result);
    free_string(result);

    predictionLabel->setText(QString("Prediction: %1").arg(prediction));
    predictionLabel->setStyleSheet("color: green;");
}

MainWindow::~MainWindow()
{
}